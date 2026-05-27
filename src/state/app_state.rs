use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::{Mutex, mpsc::Sender};
use crate::models::{
    notification_job::NotificationJob,
    job_status::JobStatus,
};

pub struct AppState {
    pub tx: Sender<NotificationJob>,
    pub jobs: Arc<Mutex<HashMap<String, JobStatus>>>,
}

impl AppState {
    pub fn new(tx: Sender<NotificationJob>) -> Self {
        Self {
            tx,
            jobs: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}