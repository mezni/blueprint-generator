use std::fmt;

#[derive(Debug, Clone)]
pub enum WorkerError {
    InvalidCoordinates(String),
    StationNotFound(String),
    DbError(String),
    Unknown(String),
}

impl fmt::Display for WorkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkerError::InvalidCoordinates(msg) => write!(f, "INVALID_COORDINATES: {}", msg),
            WorkerError::StationNotFound(id) => write!(f, "STATION_NOT_FOUND: {}", id),
            WorkerError::DbError(msg) => write!(f, "DB_ERROR: {}", msg),
            WorkerError::Unknown(msg) => write!(f, "UNKNOWN_ERROR: {}", msg),
        }
    }
}

impl std::error::Error for WorkerError {}

impl From<sqlx::Error> for WorkerError {
    fn from(e: sqlx::Error) -> Self {
        WorkerError::DbError(e.to_string())
    }
}

impl WorkerError {
    pub fn is_retryable(&self) -> bool {
        matches!(self, WorkerError::DbError(_) | WorkerError::Unknown(_))
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            WorkerError::InvalidCoordinates(_) => "INVALID_COORDINATES",
            WorkerError::StationNotFound(_) => "STATION_NOT_FOUND",
            WorkerError::DbError(_) => "DB_ERROR",
            WorkerError::Unknown(_) => "UNKNOWN_ERROR",
        }
    }
}
