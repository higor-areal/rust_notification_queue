use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum JobStatus {
    Queued,
    Processing,
    Completed,
    Failed,
}