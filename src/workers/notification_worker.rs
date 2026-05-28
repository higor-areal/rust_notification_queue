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

        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        {
            let mut map = jobs.lock().await;
            if job.notification_type == "Failed" {
                map.insert(job.id.clone(), JobStatus::Failed);
            } else {
                map.insert(job.id.clone(), JobStatus::Completed);
            }
            
            
        }

    }

}

//essa ideia é interessante, aqui sobrescrevemos o dado, aí liberamos o lock, trabalhos na nossa logica e depois locamos o map de nv e sobrescrevemos de nv, isso é interessante, como eu não sei quanto tempo vai demorar na minha logica, eu libero logo meu appstate pra outro usar e depois peço de vc pra usar mais uma vez.