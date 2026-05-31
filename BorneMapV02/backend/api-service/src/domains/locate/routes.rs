use actix_web::{get, patch, web, HttpResponse, Responder};

use crate::AppState;
use crate::domains::locate::model::{
    Charger, ChargerRow, HealthResponse, NearbyQuery, PartnerSnapshot, SearchQuery, Station,
    StationRow, StatusUpdate, StatusUpdateResponse,
};

#[get("/stations/nearby")]
pub async fn get_nearby_stations(
    state: web::Data<AppState>,
    query: web::Query<NearbyQuery>,
) -> impl Responder {
    let pool = &state.db;
    let params = query.into_inner();
    let distance = params.distance.unwrap_or(15000.0);
    let show_staged = params.show_staged.unwrap_or(false);

    let stations = sqlx::query_as::<_, StationRow>(
        r#"
        SELECT
            s.id,
            s.name,
            p.id AS partner_id,
            p.name AS partner_name,
            p.type::TEXT AS partner_type,
            ST_Y(s.geom::geometry) AS latitude,
            ST_X(s.geom::geometry) AS longitude,
            s.status,
            s.is_live,
            s.updated_at
        FROM stations s
        JOIN partners p ON p.id = s.partner_id
        WHERE ST_DWithin(s.geom, ST_MakePoint($2, $1)::geography, $3)
        AND (s.is_live = true OR $4 = true)
        ORDER BY s.name
        "#,
    )
    .bind(params.lat)
    .bind(params.lng)
    .bind(distance)
    .bind(show_staged)
    .fetch_all(pool)
    .await;

    match stations {
        Ok(rows) => {
            let station_ids: Vec<String> = rows.iter().map(|r| r.id.clone()).collect();

            let chargers = if station_ids.is_empty() {
                vec![]
            } else {
                let placeholders: Vec<String> = station_ids
                    .iter()
                    .enumerate()
                    .map(|(i, _)| format!("${}", i + 1))
                    .collect();
                let query = format!(
                    "SELECT id, station_id, plug_type, power_output, status FROM chargers WHERE station_id IN ({}) ORDER BY id",
                    placeholders.join(", ")
                );

                let mut q = sqlx::query_as::<_, ChargerRow>(&query);
                for id in &station_ids {
                    q = q.bind(id);
                }
                match q.fetch_all(pool).await {
                    Ok(c) => c,
                    Err(e) => {
                        log::error!("Charger sub-query failed: {}", e);
                        vec![]
                    }
                }
            };

            let charger_map: std::collections::HashMap<&str, Vec<Charger>> = {
                let mut map: std::collections::HashMap<&str, Vec<Charger>> =
                    std::collections::HashMap::new();
                for c in &chargers {
                    map.entry(c.station_id.as_str())
                        .or_default()
                        .push(Charger {
                            id: c.id.clone(),
                            plug_type: c.plug_type.clone(),
                            power_output: c.power_output,
                            status: c.status.clone(),
                        });
                }
                map
            };

            let result: Vec<Station> = rows
                .into_iter()
                .map(|r| Station {
                    id: r.id.clone(),
                    name: r.name,
                    partner: PartnerSnapshot {
                        id: r.partner_id,
                        name: r.partner_name,
                        partner_type: r.partner_type,
                    },
                    latitude: r.latitude,
                    longitude: r.longitude,
                    status: r.status,
                    chargers: charger_map
                        .get(r.id.as_str())
                        .cloned()
                        .unwrap_or_default(),
                    is_live: r.is_live,
                    updated_at: r.updated_at,
                })
                .collect();

            HttpResponse::Ok().json(result)
        }
        Err(e) => {
            log::error!("Database query failed: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[patch("/stations/{id}/status")]
pub async fn update_station_status(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<StatusUpdate>,
) -> impl Responder {
    let station_id = path.into_inner();
    let new_status = &body.status;

    if new_status != "Available"
        && new_status != "Occupied"
        && new_status != "Offline"
        && new_status != "Maintenance"
    {
        return HttpResponse::BadRequest().json(serde_json::json!({"error": "invalid status"}));
    }

    let result = sqlx::query_as::<_, (String, String, chrono::DateTime<chrono::Utc>)>(
        "UPDATE stations SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING id, status, updated_at",
    )
    .bind(new_status)
    .bind(&station_id)
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some((id, status, updated_at))) => {
            HttpResponse::Ok().json(StatusUpdateResponse { id, status, updated_at })
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({"error": "station not found"})),
        Err(e) => {
            log::error!("Status update failed: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn health(state: web::Data<AppState>) -> impl Responder {
    let db_ok = sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .is_ok();

    if db_ok {
        HttpResponse::Ok().json(HealthResponse {
            status: "ok".to_string(),
            database: "connected".to_string(),
        })
    } else {
        HttpResponse::ServiceUnavailable().json(HealthResponse {
            status: "degraded".to_string(),
            database: "disconnected".to_string(),
        })
    }
}

#[get("/search")]
pub async fn search_stations(
    state: web::Data<AppState>,
    query: web::Query<SearchQuery>,
) -> impl Responder {
    use sqlx::Row;

    let params = query.into_inner();
    let search_pattern = format!("%{}%", params.q);

    let rows = sqlx::query(
        r#"
        SELECT
            s.id, s.name,
            ST_Y(s.geom::geometry) AS latitude,
            ST_X(s.geom::geometry) AS longitude,
            s.status, s.is_live
        FROM stations s
        WHERE s.name ILIKE $1
        AND s.is_live = true
        ORDER BY s.name
        LIMIT 50
        "#,
    )
    .bind(&search_pattern)
    .fetch_all(&state.db)
    .await;

    match rows {
        Ok(rows) => {
            let results: Vec<serde_json::Value> = rows
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "station_id": r.get::<String, _>("id"),
                        "station_name": r.get::<String, _>("name"),
                        "latitude": r.get::<f64, _>("latitude"),
                        "longitude": r.get::<f64, _>("longitude"),
                        "status": r.get::<String, _>("status"),
                        "is_live": r.get::<bool, _>("is_live"),
                    })
                })
                .collect();
            HttpResponse::Ok().json(results)
        }
        Err(e) => {
            log::error!("Search query failed: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/stations/{station_id}")]
pub async fn get_station_detail(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let station_id = path.into_inner();

    let station = sqlx::query_as::<_, StationRow>(
        r#"
        SELECT
            s.id, s.name,
            p.id AS partner_id,
            p.name AS partner_name,
            p.type::TEXT AS partner_type,
            ST_Y(s.geom::geometry) AS latitude,
            ST_X(s.geom::geometry) AS longitude,
            s.status, s.is_live, s.updated_at
        FROM stations s
        JOIN partners p ON p.id = s.partner_id
        WHERE s.id = $1
        "#,
    )
    .bind(&station_id)
    .fetch_optional(&state.db)
    .await;

    match station {
        Ok(Some(row)) => {
            let chargers = sqlx::query_as::<_, ChargerRow>(
                "SELECT id, station_id, plug_type, power_output, status FROM chargers WHERE station_id = $1 ORDER BY id",
            )
            .bind(&station_id)
            .fetch_all(&state.db)
            .await
            .unwrap_or_default();

            let result = serde_json::json!({
                "station_id": row.id,
                "station_name": row.name,
                "address": format!("{}, Tunisia", row.name),
                "available_chargers": chargers.iter().filter(|c| c.status == "Available").count(),
                "total_chargers": chargers.len(),
                "connector_types": chargers.iter().map(|c| c.plug_type.clone()).collect::<std::collections::BTreeSet<String>>(),
                "status": row.status,
                "navigate_url": format!("https://www.google.com/maps/dir/?api=1&destination={},{}", row.latitude, row.longitude),
            });

            HttpResponse::Ok().json(result)
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({"error": "station not found"})),
        Err(e) => {
            log::error!("Station detail query failed: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
