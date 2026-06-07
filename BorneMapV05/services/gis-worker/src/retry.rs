use rand::Rng;

pub fn backoff_delay_ms(attempt: u32, base_delay_ms: u64) -> u64 {
    let exponential = base_delay_ms * 2u64.pow(attempt);
    let jitter = rand::thread_rng().gen_range(0..base_delay_ms);
    exponential + jitter
}

pub fn is_max_retries_exceeded(attempt: u32, max_retries: u32) -> bool {
    attempt >= max_retries
}

pub fn extract_retry_count(payload: &Option<serde_json::Value>) -> u32 {
    payload
        .as_ref()
        .and_then(|p| p.get("retry_count"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32
}

pub fn increment_retry_count(payload: &Option<serde_json::Value>) -> serde_json::Value {
    let current = extract_retry_count(payload);
    let mut map = match payload {
        Some(serde_json::Value::Object(m)) => m.clone(),
        _ => serde_json::Map::new(),
    };
    map.insert(
        "retry_count".to_string(),
        serde_json::Value::Number((current + 1).into()),
    );
    serde_json::Value::Object(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backoff_delay_increases_with_attempts() {
        let d1 = backoff_delay_ms(0, 1000);
        let d2 = backoff_delay_ms(1, 1000);
        assert!(d2 >= d1);
    }

    #[test]
    fn test_max_retries_not_exceeded() {
        assert!(!is_max_retries_exceeded(0, 3));
        assert!(!is_max_retries_exceeded(2, 3));
    }

    #[test]
    fn test_max_retries_exceeded() {
        assert!(is_max_retries_exceeded(3, 3));
        assert!(is_max_retries_exceeded(5, 3));
    }

    #[test]
    fn test_extract_retry_count_none() {
        assert_eq!(extract_retry_count(&None), 0);
        assert_eq!(extract_retry_count(&Some(serde_json::json!({}))), 0);
    }

    #[test]
    fn test_extract_retry_count_existing() {
        let payload = Some(serde_json::json!({"retry_count": 2}));
        assert_eq!(extract_retry_count(&payload), 2);
    }

    #[test]
    fn test_increment_retry_count_none() {
        let result = increment_retry_count(&None);
        assert_eq!(result["retry_count"], 1);
    }

    #[test]
    fn test_increment_retry_count_existing() {
        let payload = Some(serde_json::json!({"retry_count": 2}));
        let result = increment_retry_count(&payload);
        assert_eq!(result["retry_count"], 3);
    }
}
