use super::models::BboxQuery;
use common_utils::error::DomainError;

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

    if !(-180.0..=180.0).contains(&west) || !(-180.0..=180.0).contains(&east) {
        return Err(DomainError::Validation(
            "longitude must be between -180 and 180".into(),
        ));
    }

    if !(-90.0..=90.0).contains(&south) || !(-90.0..=90.0).contains(&north) {
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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_bbox() {
        let bbox = parse_bbox("9.0,33.0,12.0,38.0").unwrap();
        assert!((bbox.west - 9.0).abs() < 1e-4);
        assert!((bbox.south - 33.0).abs() < 1e-4);
        assert!((bbox.east - 12.0).abs() < 1e-4);
        assert!((bbox.north - 38.0).abs() < 1e-4);
    }

    #[test]
    fn parse_bbox_wrong_part_count_errs() {
        let err = parse_bbox("1,2,3").unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    #[test]
    fn parse_bbox_non_numeric_errs() {
        let err = parse_bbox("a,b,c,d").unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    #[test]
    fn parse_bbox_lng_out_of_range_errs() {
        let err = parse_bbox("181.0,0.0,182.0,1.0").unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    #[test]
    fn parse_bbox_lat_out_of_range_errs() {
        let err = parse_bbox("0.0,91.0,1.0,92.0").unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    #[test]
    fn parse_bbox_west_greater_than_east_errs() {
        let err = parse_bbox("12.0,33.0,9.0,38.0").unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    #[test]
    fn parse_bbox_south_greater_than_north_errs() {
        let err = parse_bbox("9.0,38.0,12.0,33.0").unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    #[test]
    fn quantize_bbox_rounds_correctly() {
        let mut bbox = BboxQuery {
            west: 9.12345,
            south: 33.67890,
            east: 12.11111,
            north: 38.22222,
        };
        quantize_bbox(&mut bbox);
        assert!((bbox.west - 9.1235).abs() < 1e-10);
        assert!((bbox.south - 33.6789).abs() < 1e-10);
        assert!((bbox.east - 12.1111).abs() < 1e-10);
        assert!((bbox.north - 38.2222).abs() < 1e-10);
    }
}
