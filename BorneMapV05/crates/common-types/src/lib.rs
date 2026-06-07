pub mod api;
pub mod events;

use serde::{Deserialize, Serialize};

pub mod sqlx_impl;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EntityPrefix {
    #[serde(rename = "USR")]
    Usr,
    #[serde(rename = "PRT")]
    Prt,
    #[serde(rename = "STN")]
    Stn,
    #[serde(rename = "CHG")]
    Chg,
    #[serde(rename = "REV")]
    Rev,
    #[serde(rename = "EVT")]
    Evt,
    #[serde(rename = "CLK")]
    Clk,
    #[serde(rename = "SESS")]
    Sess,
    #[serde(rename = "ANON")]
    Anon,
}

impl EntityPrefix {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityPrefix::Usr => "USR",
            EntityPrefix::Prt => "PRT",
            EntityPrefix::Stn => "STN",
            EntityPrefix::Chg => "CHG",
            EntityPrefix::Rev => "REV",
            EntityPrefix::Evt => "EVT",
            EntityPrefix::Clk => "CLK",
            EntityPrefix::Sess => "SESS",
            EntityPrefix::Anon => "ANON",
        }
    }
}

pub fn generate_id(prefix: EntityPrefix) -> String {
    let ulid = ulid::Ulid::new();
    format!("{}-{}", prefix.as_str(), ulid)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "registered_driver")]
    RegisteredDriver,
    #[serde(rename = "partner")]
    Partner,
    #[serde(rename = "admin")]
    Admin,
}

impl Role {
    pub fn from_keycloak(s: &str) -> Option<Self> {
        match s {
            "registered_driver" => Some(Role::RegisteredDriver),
            "partner" => Some(Role::Partner),
            "admin" => Some(Role::Admin),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Role::RegisteredDriver => "registered_driver",
            Role::Partner => "partner",
            Role::Admin => "admin",
        }
    }

    pub fn rank(&self) -> u8 {
        match self {
            Role::RegisteredDriver => 1,
            Role::Partner => 2,
            Role::Admin => 3,
        }
    }

    pub fn satisfies(&self, required: Role) -> bool {
        self.rank() >= required.rank()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StationStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "maintenance")]
    Maintenance,
    #[serde(rename = "draft")]
    Draft,
}

impl StationStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active" => Some(Self::Active),
            "inactive" => Some(Self::Inactive),
            "maintenance" => Some(Self::Maintenance),
            "draft" => Some(Self::Draft),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Maintenance => "maintenance",
            Self::Draft => "draft",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum StationAvailabilityStatus {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "limited")]
    Limited,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl StationAvailabilityStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "available" => Some(Self::Available),
            "limited" => Some(Self::Limited),
            "unavailable" => Some(Self::Unavailable),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Limited => "limited",
            Self::Unavailable => "unavailable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PartnerStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "suspended")]
    Suspended,
}

impl PartnerStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active" => Some(Self::Active),
            "suspended" => Some(Self::Suspended),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Suspended => "suspended",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ChargerStatus {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "fault")]
    Fault,
}

impl ChargerStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "available" => Some(Self::Available),
            "offline" => Some(Self::Offline),
            "fault" => Some(Self::Fault),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Offline => "offline",
            Self::Fault => "fault",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ChargerType {
    #[serde(rename = "CCS")]
    Ccs,
    #[serde(rename = "Type2")]
    Type2,
    #[serde(rename = "CHAdeMO")]
    Chademo,
}

impl ChargerType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "CCS" => Some(Self::Ccs),
            "Type2" => Some(Self::Type2),
            "CHAdeMO" => Some(Self::Chademo),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ccs => "CCS",
            Self::Type2 => "Type2",
            Self::Chademo => "CHAdeMO",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ReviewStatus {
    #[serde(rename = "published")]
    Published,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "flagged")]
    Flagged,
    #[serde(rename = "deleted")]
    Deleted,
}

impl ReviewStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "published" => Some(Self::Published),
            "hidden" => Some(Self::Hidden),
            "flagged" => Some(Self::Flagged),
            "deleted" => Some(Self::Deleted),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::Hidden => "hidden",
            Self::Flagged => "flagged",
            Self::Deleted => "deleted",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PartnerRole {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "manager")]
    Manager,
    #[serde(rename = "operator")]
    Operator,
    #[serde(rename = "viewer")]
    Viewer,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GisQueueStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "dead_letter")]
    DeadLetter,
}

impl GisQueueStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "processing" => Some(Self::Processing),
            "done" => Some(Self::Done),
            "failed" => Some(Self::Failed),
            "dead_letter" => Some(Self::DeadLetter),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Done => "done",
            Self::Failed => "failed",
            Self::DeadLetter => "dead_letter",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AvailabilitySource {
    #[serde(rename = "manual_partner")]
    ManualPartner,
    #[serde(rename = "system_sync")]
    SystemSync,
    #[serde(rename = "admin")]
    Admin,
}

impl AvailabilitySource {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "manual_partner" => Some(Self::ManualPartner),
            "system_sync" => Some(Self::SystemSync),
            "admin" => Some(Self::Admin),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ManualPartner => "manual_partner",
            Self::SystemSync => "system_sync",
            Self::Admin => "admin",
        }
    }
}
