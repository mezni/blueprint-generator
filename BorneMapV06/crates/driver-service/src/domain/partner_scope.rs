//! Partner scope domain model for access control

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::ev_auth::Role;

/// Partner scope for validating user/partner relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerScope {
    pub user_id: String,
    pub partner_id: String,
    pub role: Role,
    pub is_partner: bool,
}

impl PartnerScope {
    /// Create a new partner scope from JWT claims
    pub fn from_claims(claims: &crate::ev_auth::Claims) -> Self {
        let is_partner = claims.role == Role::Partner;

        Self {
            user_id: claims.sub.clone(),
            partner_id: claims.partner_id.clone().unwrap_or_default(),
            role: claims.role.clone(),
            is_partner,
        }
    }

    /// Validate that user has partner role
    pub fn validate_partner_role(&self) -> Result<(), crate::DomainResult> {
        if !self.is_partner {
            return Err(crate::DomainError::BusinessRuleViolation(
                "User must have partner role to access partner endpoints".to_string(),
            ));
        }
        Ok(())
    }

    /// Validate that partner_id matches
    pub fn validate_partner_id(&self, required_partner_id: &str) -> Result<(), crate::DomainResult> {
        if self.is_partner && self.partner_id != required_partner_id {
            return Err(crate::DomainError::BusinessRuleViolation(
                format!(
                    "Partner {} cannot access stations for partner {}",
                    self.partner_id, required_partner_id
                ),
            ));
        }
        Ok(())
    }

    /// Check if user can manage stations for this partner
    pub fn can_manage_stations(&self) -> bool {
        self.is_partner
    }

    /// Check if user can view stations for this partner
    pub fn can_view_stations(&self) -> bool {
        self.is_partner
    }
}

/// Charger status aggregation for partner dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargerStatusSummary {
    pub total: i32,
    pub available: i32,
    pub in_use: i32,
    pub maintenance: i32,
    pub offline: i32,
    pub availability_rate: f64,
}

impl ChargerStatusSummary {
    /// Create from counts
    pub fn new(
        total: i32,
        available: i32,
        in_use: i32,
        maintenance: i32,
        offline: i32,
    ) -> Self {
        let availability_rate = if total > 0 {
            (available as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        Self {
            total,
            available,
            in_use,
            maintenance,
            offline,
            availability_rate,
        }
    }

    /// Create empty summary
    pub fn empty() -> Self {
        Self::new(0, 0, 0, 0, 0)
    }

    /// Calculate totals
    pub fn total_count(&self) -> i32 {
        self.total
    }

    /// Calculate availability rate
    pub fn availability_percentage(&self) -> f64 {
        self.availability_rate
    }
}

/// Partner station statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerStationStats {
    pub total_stations: i32,
    pub active_stations: i32,
    pub offline_stations: i32,
    pub total_chargers: i32,
    pub active_chargers: i32,
    pub offline_chargers: i32,
    pub avg_capacity: f64,
}

impl PartnerStationStats {
    /// Create from station data
    pub fn from_stations(
        stations: &[crate::ev_domain::Station],
        chargers: &[crate::ev_domain::Charger],
    ) -> Self {
        let total_stations = stations.len() as i32;
        let active_stations = stations.iter().filter(|s| s.status.as_deref() == Some("active")).count() as i32;
        let offline_stations = stations.iter().filter(|s| s.status.as_deref() == Some("offline")).count() as i32;

        let total_chargers = chargers.len() as i32;
        let active_chargers = chargers.iter().filter(|c| c.status.as_deref() == Some("active")).count() as i32;
        let offline_chargers = chargers.iter().filter(|c| c.status.as_deref() == Some("offline")).count() as i32;

        let avg_capacity = if total_chargers > 0 {
            chargers.iter().map(|c| c.power_kw.unwrap_or(0)).sum::<i32>() as f64 / total_chargers as f64
        } else {
            0.0
        };

        Self {
            total_stations,
            active_stations,
            offline_stations,
            total_chargers,
            active_chargers,
            offline_chargers,
            avg_capacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partner_scope_creation() {
        let claims = crate::ev_auth::Claims {
            sub: "partner123".to_string(),
            email: Some("partner@example.com".to_string()),
            name: Some("Partner".to_string()),
            role: Role::Partner,
            partner_id: Some("PRT-001".to_string()),
            iat: 1700000000,
            exp: 1700003600,
            jti: Some("jti".to_string()),
        };

        let scope = PartnerScope::from_claims(&claims);
        assert_eq!(scope.user_id, "partner123");
        assert_eq!(scope.partner_id, Some("PRT-001".to_string()));
        assert!(scope.is_partner);
        assert!(scope.can_manage_stations());
    }

    #[test]
    fn test_partner_scope_validation() {
        let scope = PartnerScope {
            user_id: "user123".to_string(),
            partner_id: "PRT-001".to_string(),
            role: Role::RegisteredDriver,
            is_partner: false,
        };

        assert!(scope.validate_partner_role().is_err());
    }

    #[test]
    fn test_charger_status_summary() {
        let summary = ChargerStatusSummary::new(10, 8, 1, 1, 0);
        assert_eq!(summary.total, 10);
        assert_eq!(summary.available, 8);
        assert_eq!(summary.in_use, 1);
        assert_eq!(summary.maintenance, 1);
        assert_eq!(summary.offline, 0);
        assert!((summary.availability_rate - 80.0).abs() < 0.01);
    }

    #[test]
    fn test_partner_station_stats() {
        let stations = vec![
            crate::ev_domain::Station {
                id: "STN-001".to_string(),
                name: Some("Station 1".to_string()),
                address: None,
                latitude: Some(36.8),
                longitude: Some(10.1),
                partner_id: Some("PRT-001".to_string()),
                station_type: Some("EV Charging".to_string()),
                power_kw: Some(150),
                available_chargers: Some(4),
                status: Some("active".to_string()),
                created_at: None,
                updated_at: None,
            },
            crate::ev_domain::Station {
                id: "STN-002".to_string(),
                name: Some("Station 2".to_string()),
                address: None,
                latitude: Some(36.9),
                longitude: Some(10.2),
                partner_id: Some("PRT-001".to_string()),
                station_type: Some("EV Charging".to_string()),
                power_kw: Some(120),
                available_chargers: Some(3),
                status: Some("active".to_string()),
                created_at: None,
                updated_at: None,
            },
        ];

        let chargers = vec![
            crate::ev_domain::Charger {
                id: "CHR-001".to_string(),
                station_id: "STN-001".to_string(),
                connector_type: "CCS2".to_string(),
                power_kw: Some(150),
                status: Some("active".to_string()),
            },
            crate::ev_domain::Charger {
                id: "CHR-002".to_string(),
                station_id: "STN-001".to_string(),
                connector_type: "Type 2".to_string(),
                power_kw: Some(22),
                status: Some("active".to_string()),
            },
        ];

        let stats = PartnerStationStats::from_stations(&stations, &chargers);
        assert_eq!(stats.total_stations, 2);
        assert_eq!(stats.total_chargers, 2);
        assert_eq!(stats.avg_capacity, 86.0);
    }
}
