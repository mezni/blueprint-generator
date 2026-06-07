//! Rate limiting middleware

use actix_web::{dev::ServiceRequest, error::Error, web, HttpRequest, HttpResponse, ResponseError};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 100,
            burst_size: 10,
        }
    }
}

impl RateLimitConfig {
    /// Create new rate limit config with custom values
    pub fn new(requests_per_minute: u32, burst_size: u32) -> Self {
        Self {
            requests_per_minute,
            burst_size,
        }
    }
}

/// Rate limiter middleware
#[derive(Clone)]
pub struct RateLimiter {
    config: RateLimitConfig,
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
}

impl RateLimiter {
    /// Create new rate limiter
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Check if request should be rate-limited
    pub fn check(&self, ip: &str) -> Result<(), ApiError> {
        let requests = self.requests.lock().map_err(|e| ApiError::InternalServerError(e.to_string()))?;

        let now = Instant::now();
        let window_start = now - Duration::from_secs(60); // 1-minute window

        // Get request history for this IP
        let ip_requests: Vec<Instant> = requests.get(ip)
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|&time| *time > window_start)
            .cloned()
            .collect();

        // Add new request
        let mut ip_requests = requests.entry(ip.to_string()).or_insert_with(Vec::new);
        ip_requests.push(now);

        // Remove old requests
        ip_requests.retain(|&time| time > window_start);

        // Check if rate limit exceeded
        if ip_requests.len() > self.config.requests_per_minute as usize {
            return Err(ApiError::RateLimited);
        }

        Ok(())
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(RateLimitConfig::default())
    }
}

/// Rate limiter middleware handler
pub async fn rate_limiter_middleware(
    request: ServiceRequest,
    limiter: web::Data<RateLimiter>,
) -> Result<ServiceRequest, Error> {
    // Extract IP from X-Forwarded-For header (for proxies) or RemoteAddr
    let ip = request
        .headers()
        .get("X-Forwarded-For")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_else(|| "unknown")
        .to_string();

    // Check rate limit
    if let Err(e) = limiter.check(&ip) {
        return Err(Error::from(e));
    }

    Ok(request)
}

/// Rate limited response
pub struct RateLimitedError;

impl ResponseError for RateLimitedError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::TOO_MANY_REQUESTS
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": "rate_limit_exceeded",
            "message": "Too many requests. Maximum {} per minute per IP.",
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_config_default() {
        let config = RateLimitConfig::default();
        assert_eq!(config.requests_per_minute, 100);
        assert_eq!(config.burst_size, 10);
    }

    #[test]
    fn test_rate_limit_config_custom() {
        let config = RateLimitConfig::new(50, 20);
        assert_eq!(config.requests_per_minute, 50);
        assert_eq!(config.burst_size, 20);
    }

    #[test]
    fn test_rate_limiter_check_normal() {
        let config = RateLimitConfig::new(10, 2);
        let limiter = RateLimiter::new(config);
        let ip = "192.168.1.1";

        // First request should pass
        assert!(limiter.check(ip).is_ok());

        // Second request should pass
        assert!(limiter.check(ip).is_ok());

        // Third request should fail
        assert!(limiter.check(ip).is_err());
    }

    #[test]
    fn test_rate_limiter_cleanup_old_requests() {
        let config = RateLimitConfig::new(10, 2);
        let limiter = RateLimiter::new(config);
        let ip = "192.168.1.2";

        // Add 15 requests over 90 seconds
        let now = Instant::now();
        for i in 0..15 {
            let requests = limiter.requests.lock().unwrap();
            requests.entry(ip.to_string()).or_insert_with(Vec::new).push(now - Duration::from_secs(i));
        }

        // After 60 seconds, only last 10 should remain
        std::thread::sleep(Duration::from_secs(61));

        let requests = limiter.requests.lock().unwrap();
        let ip_requests = requests.get(&ip.to_string()).unwrap();
        assert!(ip_requests.len() <= 10);
    }
}
