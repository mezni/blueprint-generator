//! Domain layer for driver-service

pub mod station;
pub mod nearby_query;
pub mod validation;
pub mod favorites;
pub mod review;
pub mod partner_scope;
pub mod charger_status;

pub use station::Station;
pub use nearby_query::{NearbyQuery, NearbyQueryResult};
pub use validation::{validate_coordinates, validate_radius};
pub use favorites::{Favorite, AddFavoriteInput, RemoveFavoriteInput};
pub use review::Review;
pub use partner_scope::{PartnerScope, ChargerStatusSummary, PartnerStationStats};
pub use charger_status::{ChargerStatus, ChargerStatusSummary, StationAvailabilityStatus, ChargerStatusDistribution};
