use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub timestamp: f64,
    pub identity_key: String,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct EventMetrics {
    pub hit_count: u64,
    pub first_seen: f64,
    pub last_seen: f64,
    pub last_state: String,
}

#[derive(Debug, Clone)]
pub struct ProcessResult {
    pub status: String,
    pub identity_key: String,
    pub resulting_state: String,
    pub current_learning_mode: bool,
    pub current_threshold: f64,
    pub metrics: EventMetrics,
}

#[derive(Clone)]
pub struct AttackDetection {
    anomaly_threshold: Arc<Mutex<f64>>,
    learning_mode: Arc<Mutex<bool>>,
    state_registry: Arc<Mutex<HashMap<String, EventMetrics>>>,
    audit_log: Arc<Mutex<Vec<AuditLogEntry>>>,
}

impl AttackDetection {
    pub fn new(anomaly_threshold: f64) -> Self {
        Self {
            anomaly_threshold: Arc::new(Mutex::new(anomaly_threshold)),
            learning_mode: Arc::new(Mutex::new(false)),
            state_registry: Arc::new(Mutex::new(HashMap::new())),
            audit_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn process_event(&self, attack_type: &str, source_ip: &str) -> ProcessResult {
        let event_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        
        let registry_key = format!("{}:{}", source_ip, attack_type);

        let mut threshold = self.anomaly_threshold.lock().unwrap();
        let mut learning = self.learning_mode.lock().unwrap();
        let mut registry = self.state_registry.lock().unwrap();
        let mut log = self.audit_log.lock().unwrap();

        let execution_state = if *threshold > 0.0 {
            if !*learning {
                *learning = true;
                *threshold = 0.8;
                "ALERT_DETECTED".to_string()
            } else {
                "LEARNING_ACTIVE".to_string()
            }
        } else {
            "NO_ALERT_NORMAL".to_string()
        };

        // Record the immutable audit log entry
        log.push(AuditLogEntry {
            timestamp: event_timestamp,
            identity_key: registry_key.clone(),
            state: execution_state.clone(),
        });

        let metrics = registry.entry(registry_key.clone()).or_insert_with(|| EventMetrics {
            hit_count: 0,
            first_seen: event_timestamp,
            last_seen: event_timestamp,
            last_state: execution_state.clone(),
        });

        metrics.hit_count += 1;
        metrics.last_seen = event_timestamp;
        metrics.last_state = execution_state.clone();

        ProcessResult {
            status: "SUCCESS".to_string(),
            identity_key: registry_key,
            resulting_state: execution_state,
            current_learning_mode: *learning,
            current_threshold: *threshold,
            metrics: metrics.clone(),
        }
    }

    pub fn get_metrics(&self, attack_type: &str, source_ip: &str) -> Option<EventMetrics> {
        let registry_key = format!("{}:{}", source_ip, attack_type);
        let registry = self.state_registry.lock().unwrap();
        registry.get(&registry_key).cloned()
    }

    pub fn get_audit_log(&self) -> Vec<AuditLogEntry> {
        let log = self.audit_log.lock().unwrap();
        log.clone()
    }
}
