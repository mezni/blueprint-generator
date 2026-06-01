use crate::error::{AuditEvent, AuditEventType};
use chrono::Utc;

pub struct AuditProducer {
    rabbitmq_url: Option<String>,
    exchange: String,
}

impl AuditProducer {
    pub fn new(rabbitmq_url: Option<String>) -> Self {
        Self {
            rabbitmq_url,
            exchange: "events.exchange".into(),
        }
    }

    pub async fn emit(&self, event: AuditEvent) {
        if let Some(ref url) = self.rabbitmq_url {
            if let Err(e) = self.publish_to_rabbitmq(url, &event).await {
                tracing::warn!("Failed to publish audit event: {e}");
            }
        }
        tracing::info!(
            "AUDIT: {:?} user={:?} client={:?} outcome={}",
            event.event_type,
            event.user_id,
            event.client_id,
            event.outcome,
        );
    }

    async fn publish_to_rabbitmq(
        &self,
        url: &str,
        event: &AuditEvent,
    ) -> Result<(), String> {
        let payload = serde_json::to_string(event)
            .map_err(|e| format!("Serialization failed: {e}"))?;

        let client = reqwest::Client::new();
        let resp = client
            .post(format!("{url}/api/exchanges/%2f/{}/publish", self.exchange))
            .json(&serde_json::json!({
                "properties": {
                    "content_type": "application/json",
                    "delivery_mode": 2,
                },
                "routing_key": "auth.event",
                "payload": payload,
                "payload_encoding": "string",
            }))
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!("RabbitMQ returned {}", resp.status()));
        }
        Ok(())
    }

    pub fn create_event(
        event_type: AuditEventType,
        user_id: Option<String>,
        client_id: Option<String>,
        ip_address: Option<String>,
        details: Option<serde_json::Value>,
    ) -> AuditEvent {
        AuditEvent {
            event_type,
            user_id,
            client_id,
            ip_address,
            outcome: "success".into(),
            timestamp: Utc::now(),
            details,
        }
    }
}
