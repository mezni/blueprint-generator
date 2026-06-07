//! Favorite domain model

use serde::{Deserialize, Serialize};

/// Favorite entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Favorite {
    pub id: String,
    pub user_id: String,
    pub station_id: String,
    pub created_at: String,
}

/// Add favorite input
#[derive(Debug, Clone, Deserialize)]
pub struct AddFavoriteInput {
    pub station_id: String,
}

/// Remove favorite input
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveFavoriteInput {
    pub favorite_id: String,
}

/// Update favorite input
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateFavoriteInput {
    pub station_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_favorite_creation() {
        let favorite = Favorite {
            id: "FVT-001".to_string(),
            user_id: "USR-001".to_string(),
            station_id: "STN-001".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(favorite.id, "FVT-001");
        assert_eq!(favorite.user_id, "USR-001");
        assert_eq!(favorite.station_id, "STN-001");
    }

    #[test]
    fn test_favorite_equality() {
        let favorite1 = Favorite {
            id: "FVT-001".to_string(),
            user_id: "USR-001".to_string(),
            station_id: "STN-001".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let favorite2 = Favorite {
            id: "FVT-001".to_string(),
            user_id: "USR-001".to_string(),
            station_id: "STN-001".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(favorite1, favorite2);
    }
}
