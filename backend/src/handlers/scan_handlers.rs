use crate::utils::errors::AppError;
use actix_web::{get, post, web, HttpResponse};
use crate::services::state::AppState;
use serde_json::Value;
use crate::models::scan::ScanResponse;

#[post("/nuclei")]
pub async fn start_nuclei_scan(
    app_state: web::Data<AppState>,
    project_id: web::Path<uuid::Uuid>,
    scan_config: web::Json<Value>,
) -> HttpResponse {
    let mut config = scan_config.into_inner();
    config["project_id"] = project_id.to_string().into();
    
    match app_state.scanner_services.nuclei.create_scan(config).await {
        Ok(task_id) => HttpResponse::Ok().json(serde_json::json!({
            "task_id": task_id,
            "status": "pending"
        })),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[post("/nmap")]
pub async fn start_nmap_scan(
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let result = app_state.scanner_services.nmap.start_scan(/* параметры */).await;
    
    match result {
        Ok(_) => HttpResponse::Ok().json("Nmap scan started successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[post("/gowitness")]
pub async fn start_gowitness_scan(
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let result = app_state.scanner_services.gowitness.start_scan(/* параметры */).await;
    
    match result {
        Ok(_) => HttpResponse::Ok().json("Gowitness scan started successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("/{scan_id}/status")]
pub async fn get_scan_status(
    app_state: web::Data<AppState>,
    scan_id: web::Path<String>,
) -> HttpResponse {
    match app_state.scanner_services.nuclei.get_scan_status(&scan_id).await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("/{scan_id}/results")]
pub async fn get_scan_results(
    app_state: web::Data<AppState>,
    scan_id: web::Path<String>,
) -> HttpResponse {
    // Получаем информацию о скане
    let scan = match app_state.scanner_services.nuclei.get_scan_status(&scan_id).await {
        Ok(scan) => scan,
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    // Получаем результаты в зависимости от типа сканера
    let result = match scan.scanner_type.as_str() {
        "nuclei" => {
            let result = app_state.scanner_services.nuclei.get_scan_result(&scan_id).await?;
            Some(ScanResult::Nuclei(result))
        },
        "nmap" => {
            let result = app_state.scanner_services.nmap.get_scan_result(&scan_id).await?;
            Some(ScanResult::Nmap(result))
        },
        "gowitness" => {
            let result = app_state.scanner_services.gowitness.get_scan_result(&scan_id).await?;
            Some(ScanResult::Gowitness(result))
        },
        _ => None,
    };

    // Создаем ответ
    let mut response = ScanResponse::from(scan);
    response.result = result;

    HttpResponse::Ok().json(response)
}
