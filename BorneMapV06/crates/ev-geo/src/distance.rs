use crate::point::LatLng;

const EARTH_RADIUS_M: f64 = 6_371_000.0;

/// Calculate the great-circle distance between two points using the Haversine formula.
pub fn haversine_distance(p1: &LatLng, p2: &LatLng) -> f64 {
    let d_lat = (p2.latitude - p1.latitude).to_radians();
    let d_lon = (p2.longitude - p1.longitude).to_radians();

    let a = (d_lat / 2.0).sin().powi(2)
        + p1.latitude_radians().cos()
            * p2.latitude_radians().cos()
            * (d_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_M * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haversine_same_point() {
        let p = LatLng::new(36.806389, 10.181667).unwrap();
        let dist = haversine_distance(&p, &p);
        assert!(dist.abs() < 1.0);
    }

    #[test]
    fn test_haversine_tunis_to_sfax() {
        let tunis = LatLng::new(36.806389, 10.181667).unwrap();
        let sfax = LatLng::new(34.740833, 10.761111).unwrap();
        let dist = haversine_distance(&tunis, &sfax);
        // Tunis to Sfax is ~235 km
        assert!((dist - 235_000.0).abs() < 20_000.0);
    }

    #[test]
    fn test_haversine_tunis_to_paris() {
        let tunis = LatLng::new(36.806389, 10.181667).unwrap();
        let paris = LatLng::new(48.8566, 2.3522).unwrap();
        let dist = haversine_distance(&tunis, &paris);
        // Tunis to Paris is ~1480 km
        assert!((dist - 1_480_000.0).abs() < 50_000.0);
    }
}
