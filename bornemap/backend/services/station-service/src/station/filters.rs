use common_utils::error::DomainError;
use super::models::BboxQuery;

pub fn parse_bbox(query: &str) -> Result<BboxQuery, DomainError> {
    let parts: Vec<&str> = query.split(',').collect();
    if parts.len() != 4 {
        return Err(DomainError::Validation(
            "bbox must be 4 comma-separated values: west,south,east,north".into(),
        ));
    }

    let west: f64 = parts[0]
        .parse()
        .map_err(|_| DomainError::Validation("west must be a number".into()))?;
    let south: f64 = parts[1]
        .parse()
        .map_err(|_| DomainError::Validation("south must be a number".into()))?;
    let east: f64 = parts[2]
        .parse()
        .map_err(|_| DomainError::Validation("east must be a number".into()))?;
    let north: f64 = parts[3]
        .parse()
        .map_err(|_| DomainError::Validation("north must be a number".into()))?;

    if west < -180.0 || west > 180.0 || east < -180.0 || east > 180.0 {
        return Err(DomainError::Validation(
            "longitude must be between -180 and 180".into(),
        ));
    }

    if south < -90.0 || south > 90.0 || north < -90.0 || north > 90.0 {
        return Err(DomainError::Validation(
            "latitude must be between -90 and 90".into(),
        ));
    }

    if west > east {
        return Err(DomainError::Validation(
            "west must be less than or equal to east (antimeridian crossing not supported)".into(),
        ));
    }

    if south > north {
        return Err(DomainError::Validation(
            "south must be less than or equal to north".into(),
        ));
    }

    let mut bbox = BboxQuery {
        west,
        south,
        east,
        north,
    };
    quantize_bbox(&mut bbox);
    Ok(bbox)
}

pub fn quantize_bbox(bbox: &mut BboxQuery) {
    bbox.west = (bbox.west * 10_000.0).round() / 10_000.0;
    bbox.south = (bbox.south * 10_000.0).round() / 10_000.0;
    bbox.east = (bbox.east * 10_000.0).round() / 10_000.0;
    bbox.north = (bbox.north * 10_000.0).round() / 10_000.0;
}
