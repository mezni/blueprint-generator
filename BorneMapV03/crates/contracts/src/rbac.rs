use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    RegisteredDriver,
    Partner,
    Admin,
}
