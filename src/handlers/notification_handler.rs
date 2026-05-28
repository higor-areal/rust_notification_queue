use crate::{
    models::{
        job_status::JobStatus, notification_job::{NotificationJob, NotificationJobRequest}
    }, responses::response::{ResponseError, ResponseSuccess}, state::{ app_state::AppState}
};
use std::{collections::HashMap, sync::Arc};

use axum::{
    Json,
    http::StatusCode,
    extract::{State, Path}
};
use serde_json::{Value, json};

pub async fn notifications(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NotificationJobRequest>
) -> Result<(StatusCode, Json<ResponseSuccess<Value>>), (StatusCode, Json<ResponseError>)>{
    if !payload.valid() {
        return Err(ResponseError::bad_request("payload invalido"));
    }

    //poderiamos usar essa notification para outras coisas mas por enquanto vamos só jogar na fila

    let job = NotificationJob::new(&payload.notification_type, &payload.to, &payload.message);

    {
        let mut map = state.jobs.lock().await;
        map.insert(job.id.clone(), JobStatus::Queued);
    }

    state.tx.send(job.clone()).await.map_err(
        |_| { ResponseError::internal_server_error("Erro ao enfileirar job")
    })?;

    Ok(ResponseSuccess::created(json!({
        "job_id": job.id,
        "status": JobStatus::Queued
    })))

}

pub  async fn get_job(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<String>
) -> Result<(StatusCode, Json<ResponseSuccess<Value>>), (StatusCode, Json<ResponseError>)>{

    let data = state.jobs.lock().await;

    let job = match data.get(&uuid) {
        Some(t) => t,
        None => return Err(ResponseError::bad_request("Job não processado ou inexistente"))
    };

    Ok(ResponseSuccess::created(json!({
        "job_id": uuid,
        "status": job
    })))

}

pub async fn get_all_jobs(
    State(state): State<Arc<AppState>>
) -> Json<HashMap<String, JobStatus>>{
    let data = state.jobs.lock().await;

    Json(data.clone())

}