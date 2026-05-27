use tokio::sync::mpsc::Receiver;
use std::{
    sync::Arc,
    collections::HashMap
};
use tokio::sync::Mutex;
use crate::models::{
    notification_job::NotificationJob,
    job_status::JobStatus
};



pub async fn notification_worker(
    mut rx: Receiver<NotificationJob>,
    jobs: Arc<Mutex<HashMap<String, JobStatus>>>
) {
    
    while let Some(job) = rx.recv().await {
        //logica de trabalho
        {
            let mut map = jobs.lock().await;
            map.insert(job.id.clone(), JobStatus::Processing);
            
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        {
            let mut map = jobs.lock().await;
            map.insert(job.id.clone(), JobStatus::Completed);
            
        }

    }

}