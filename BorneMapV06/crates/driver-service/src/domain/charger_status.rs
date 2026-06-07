//! Charger status domain model for partner dashboard

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Charger status types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ChargerStatus {
    Available,
    InUse,
    Maintenance,
    Offline,
}

impl ChargerStatus {
    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "available" => Some(ChargerStatus::Available),
            "in_use" | "in use" => Some(ChargerStatus::InUse),
            "maintenance" => Some(ChargerStatus::Maintenance),
            "offline" => Some(ChargerStatus::Offline),
            _ => None,
        }
    }

    /// Get display name
    pub fn display_name(&self) -> String {
        match self {
            ChargerStatus::Available => "Available".to_string(),
            ChargerStatus::InUse => "In Use".to_string(),
            ChargerStatus::Maintenance => "Maintenance".to_string(),
            ChargerStatus::Offline => "Offline".to_string(),
        }
    }

    /// Get status color for UI
    pub fn color(&self) -> &'static str {
        match self {
            ChargerStatus::Available => "#22c55e", // green
            ChargerStatus::InUse => "#3b82f6", // blue
            ChargerStatus::Maintenance => "#f59e0b", // yellow
            ChargerStatus::Offline => "#ef4444", // red
        }
    }
}

/// Charger status aggregation
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
    /// Create new summary
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

    /// Calculate totals
    pub fn total_count(&self) -> i32 {
        self.total
    }

    /// Calculate availability rate
    pub fn availability_percentage(&self) -> f64 {
        self.availability_rate
    }

    /// Check if any chargers are available
    pub fn has_available(&self) -> bool {
        self.available > 0
    }

    /// Check if station is fully occupied
    pub fn is_full(&self) -> bool {
        self.available == 0 && self.total > 0
    }
}

/// Station availability status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationAvailabilityStatus {
    pub status: String,
    pub is_available: bool,
    pub total_chargers: i32,
    pub available_chargers: i32,
    pub occupancy_rate: f64,
}

impl StationAvailabilityStatus {
    /// Create from counts
    pub fn new(
        total: i32,
        available: i32,
    ) -> Self {
        let status = if available == 0 {
            "fully_occupied".to_string()
        } else if available > 0 {
            "partially_available".to_string()
        } else {
            "offline".to_string()
        };

        let occupancy_rate = if total > 0 {
            (total - available) as f64 / total as f64 * 100.0
        } else {
            0.0
        };

        Self {
            status,
            is_available: available > 0,
            total_chargers: total,
            available_chargers: available,
            occupancy_rate,
        }
    }

    /// Format occupancy rate
    pub fn formatted_occupancy_rate(&self) -> String {
        format!("{:.1}%", self.occupancy_rate)
    }
}

/// Charger status distribution for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargerStatusDistribution {
    pub available: i32,
    pub in_use: i32,
    pub maintenance: i32,
    pub offline: i32,
    pub total: i32,
}

impl ChargerStatusDistribution {
    /// Create from HashMap
    pub fn from_map(status_map: &HashMap<ChargerStatus, i32>) -> Self {
        let mut distribution = Self::default();
        distribution.available = *status_map.get(&ChargerStatus::Available).unwrap_or(&0);
        distribution.in_use = *status_map.get(&ChargerStatus::InUse).unwrap_or(&0);
        distribution.maintenance = *status_map.get(&ChargerStatus::Maintenance).unwrap_or(&0);
        distribution.offline = *status_map.get(&ChargerStatus::Offline).unwrap_or(&0);
        distribution.total = distribution.available + distribution.in_use + distribution.maintenance + distribution.offline;
        distribution
    }
}

impl Default for ChargerStatusDistribution {
    fn default() -> Self {
        Self {
            available: 0,
            in_use: 0,
            maintenance: 0,
            offline: 0,
            total: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charger_status_from_str() {
        assert_eq!(ChargerStatus::from_str("available"), Some(ChargerStatus::Available));
        assert_eq!(ChargerStatus::from_str("In Use"), Some(ChargerStatus::InUse));
        assert_eq!(ChargerStatus::from_str("MAINTENANCE"), Some(ChargerStatus::Maintenance));
        assert_eq!(ChargerStatus::from_str("offline"), Some(ChargerStatus::Offline));
        assert_eq!(ChargerStatus::from_str("unknown"), None);
    }

    #[test]
    fn test_charger_status_display_name() {
        assert_eq!(ChargerStatus::Available.display_name(), "Available");
        assert_eq!(ChargerStatus::InUse.display_name(), "In Use");
        assert_eq!(ChargerStatus::Maintenance.display_name(), "Maintenance");
        assert_eq!(ChargerStatus::Offline.display_name(), "Offline");
    }

    #[test]
    fn test_charger_status_color() {
        assert_eq!(ChargerStatus::Available.color(), "#22c55e");
        assert_eq!(ChargerStatus::InUse.color(), "#3b82f6");
        assert_eq!(ChargerStatus::Maintenance.color(), "#f59e0b");
        assert_eq!(ChargerStatus::Offline.color(), "#ef4444");
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
    fn test_station_availability_status() {
        let status = StationAvailabilityStatus::new(10, 8);
        assert_eq!(status.status, "partially_available");
        assert!(status.is_available);
        assert_eq!(status.total_chargers, 10);
        assert_eq!(status.available_chargers, 8);
        assert!((status.occupancy_rate - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_station_availability_status_full() {
        let status = StationAvailabilityStatus::new(10, 0);
        assert_eq!(status.status, "fully_occupied");
        assert!(!status.is_available);
    }

    #[test]
    fn test_charger_status_distribution() {
        let mut map = HashMap::new();
        map.insert(ChargerStatus::Available, 5);
        map.insert(ChargerStatus::InUse, 3);
        map.insert(ChargerStatus::Maintenance, 1);
        map.insert(ChargerStatus::Offline, 1);

        let distribution = ChargerStatusDistribution::from_map(&map);
        assert_eq!(distribution.available, 5);
        assert_eq!(distribution.in_use, 3);
        assert_eq!(distribution.maintenance, 1);
        assert_eq!(distribution.offline, 1);
        assert_eq!(distribution.total, 10);
    }
}
