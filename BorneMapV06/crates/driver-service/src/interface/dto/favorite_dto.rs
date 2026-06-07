//! DTOs for favorites endpoint

use serde::{Deserialize, Serialize};

use crate::ev_domain::Station;

/// Request DTO for adding a favorite
#[derive(Debug, Clone, Deserialize)]
pub struct AddFavoriteRequest {
    pub station_id: String,
}

/// Response DTO for a favorite
#[derive(Debug, Clone, Serialize)]
pub struct FavoriteResponse {
    pub favorite_id: String,
    pub user_id: String,
    pub station_id: String,
    pub station: Station,
    pub created_at: String,
}

/// Response DTO for listing favorites
#[derive(Debug, Clone, Serialize)]
pub struct ListFavoritesResponse {
    pub favorites: Vec<FavoriteResponse>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub has_more: bool,
}

impl FavoriteResponse {
    /// Create from favorite and station
    pub fn from_favorite_station(
        favorite: crate::domain::Favorite,
        station: Station,
    ) -> Self {
        Self {
            favorite_id: favorite.id,
            user_id: favorite.user_id,
            station_id: favorite.station_id,
            station,
            created_at: favorite.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_favorite_request() {
        let request = AddFavoriteRequest {
            station_id: "STN-001".to_string(),
        };

        assert_eq!(request.station_id, "STN-001");
    }

    #[test]
    fn test_favorite_response_from_favorite_station() {
        let favorite = crate::domain::Favorite {
            id: "FVT-001".to_string(),
            user_id: "USR-001".to_string(),
            station_id: "STN-001".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let station = crate::ev_domain::Station {
            id: "STN-001".to_string(),
            name: Some("Test Station".to_string()),
            address: Some("Test Address".to_string()),
            latitude: Some(36.8065),
            longitude: Some(10.1815),
            partner_id: Some("PRT-001".to_string()),
            station_type: Some("EV Charging".to_string()),
            power_kw: Some(150),
            available_chargers: Some(4),
            status: Some("active".to_string()),
            created_at: None,
            updated_at: None,
        };

        let response = FavoriteResponse::from_favorite_station(favorite, station);
        assert_eq!(response.favorite_id, "FVT-001");
        assert_eq!(response.station_id, "STN-001");
    }
}
