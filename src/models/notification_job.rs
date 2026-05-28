use uuid::Uuid;
use serde::Deserialize;

#[derive(Clone)]
pub struct NotificationJob {
    pub id: String,
    pub notification_type: String,
    pub to: String,
    pub message: String,
}


impl NotificationJob{
    pub fn new(not: &str, to: &str, msg: &str) -> Self{
        Self { 
            id: Uuid::new_v4().to_string(),
            notification_type: not.to_string(),
            to: to.to_string(),
            message: msg.to_string() }
    }
}

#[derive(Deserialize)]
pub struct NotificationJobRequest {
    pub notification_type: String,
    pub to: String,
    pub message: String,
}

impl NotificationJobRequest{
    pub fn valid(&self) -> bool {
        if self.message.is_empty() || self.to.is_empty() {
            return false;
        }
        true
    }
}