use actix_web::{web, post, get, HttpResponse};
use crate::services::scanner::{ScannerService, VulnerabilityScanner};
use crate::services::scanner::types::{ScanResult, Error};
use serde::{Deserialize, Serialize};
use crate::utils::errors::AppError;
use crate::services::scanner::nuclei::NucleiScanRequest;
use crate::services::scanner::nmap::service::NmapScanRequest;

#[derive(Debug, Deserialize)]
pub struct ScanRequest {
    pub r#type: String, 
    pub data: String,   
    pub proxy: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ScanResponse {
    pub task_id: String,
    pub status: String,
}

#[post("/service")]
pub async fn start_scan_handler(
    scanner_service: web::Data<ScannerService>,
    request: web::Json<ScanRequest>,
) -> Result<HttpResponse, AppError> {
    match request.r#type.as_str() {
        "nuclei" => {
            let nuclei = scanner_service.get_nuclei().await;
            let mut scanner = nuclei.lock().await;
            let req = NucleiScanRequest {
                target: request.data.clone(),
                templates: None,
                severity: None,
                output_format: None,
            };
            match scanner.create_scan(req).await {
                Ok(task_id) => Ok(HttpResponse::Ok().json(ScanResponse {
                    task_id,
                    status: "queued".to_string(),
                })),
                Err(e) => Err(AppError::InternalServerError),            }
        }
        "nmap" => {
            let nmap = scanner_service.get_nmap().await;
            let mut scanner = nmap.lock().await;
            let req = NmapScanRequest { };
            match scanner.create_scan(req).await {
                Ok(task_id) => Ok(HttpResponse::Ok().json(ScanResponse {
                    task_id,
                    status: "queued".to_string(),
                })),
                Err(e) => Err(AppError::InternalServerError),            }
        }
        "gowitness" => {
            let gowitness = scanner_service.get_gowitness().await;
            let mut scanner = gowitness.lock().await;
            match scanner.create_scan(request.into_inner()).await {
                Ok(task_id) => HttpResponse::Ok().json(ScanResponse {
                    task_id,
                    status: "queued".to_string(),
                }),
                Err(e) => Err(AppError::InternalServerError),            }
        }
        _ => Err(AppError::BadRequest),
    }
}

#[get("/service/{task_id}")]
pub async fn get_scan_result_handler(
    scanner_service: web::Data<ScannerService>,
    scanner_type: web::Path<String>,
    task_id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    match scanner_type.as_str() {
        "nuclei" => {
            let nuclei = scanner_service.get_nuclei().await;
            let mut scanner = nuclei.lock().await;
            match scanner.get_scan_result(&task_id).await {
                Ok(result) => Ok(HttpResponse::Ok().json(result)),
                Err(e) => Err(AppError::InternalServerError),            }
        }
        "nmap" => {
            let nmap = scanner_service.get_nmap().await;
            let mut scanner = nmap.lock().await;
            match scanner.get_scan_result(&task_id).await {
                Ok(result) => Ok(HttpResponse::Ok().json(result)),
                Err(e) => Err(AppError::InternalServerError),
            }
        }
        "gowitness" => {
            let gowitness = scanner_service.get_gowitness().await;
            let mut scanner = gowitness.lock().await;
            match scanner.get_scan_result(&task_id).await {
                Ok(result) => Ok(HttpResponse::Ok().json(result)),
                Err(e) => Err(AppError::InternalServerError),
            }
        }
        _ => Err(AppError::BadRequest),
    }
}


#[post("/service/confirm")]
pub async fn confirm_scan_handler(
    scanner_service: web::Data<ScannerService>,
    request: web::Json<ScanRequest>,
) -> Result<HttpResponse, AppError> {
    // FOR TESTING
    Ok(HttpResponse::Ok().json(ScanResponse {
        task_id: "I DO NOTHING".to_string(),
        status: "completed".to_string(),
    }))
}
