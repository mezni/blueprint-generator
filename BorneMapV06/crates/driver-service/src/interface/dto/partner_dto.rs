//! Partner DTOs for API responses

use serde::{Deserialize, Serialize};

/// Partner DTO for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerDTO {
    pub id: String,
    pub name: String,
    pub email: String,
    pub logo: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub station_count: i32,
}

impl PartnerDTO {
    /// Create from partner entity
    pub fn from_partner(entity: crate::ev_domain::Partner) -> Self {
        Self {
            id: entity.id.clone(),
            name: entity.name.clone().unwrap_or_default(),
            email: entity.email.clone().unwrap_or_default(),
            logo: entity.logo,
            status: entity.status.clone().unwrap_or("active".to_string()),
            created_at: entity.created_at.map(|dt| dt.format("%Y-%m-%d").to_string()).unwrap_or_default(),
            updated_at: entity.updated_at.map(|dt| dt.format("%Y-%m-%d").to_string()).unwrap_or_default(),
            station_count: 0, // Will be populated by query
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partner_dto_creation() {
        let partner = crate::ev_domain::Partner {
            id: "PRT-001".to_string(),
            name: Some("AutoMotive Tunis".to_string()),
            email: Some("partner1@automotive.tn".to_string()),
            logo: None,
            status: Some("active".to_string()),
            created_at: None,
            updated_at: None,
        };

        let dto = PartnerDTO::from_partner(partner);
        assert_eq!(dto.id, "PRT-001");
        assert_eq!(dto.name, "AutoMotive Tunis");
        assert_eq!(dto.email, "partner1@automotive.tn");
    }
}
