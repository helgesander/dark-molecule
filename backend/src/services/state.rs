use crate::services::report::{docx::DocxReportService, markdown::MarkdownReportService, pdf::PdfReportService};
use crate::services::scanner::{nuclei::NucleiService, nmap::NmapService, gowitness::GowitnessService};
use crate::utils::config::AppConfig;
use diesel::r2d2::Pool;
use diesel::PgConnection;

pub struct ScannerServices {
    pub nuclei: NucleiService,
    pub nmap: NmapService,
    pub gowitness: GowitnessService,
}

pub struct ReportServices {
    pub docx: DocxReportService,
    pub markdown: MarkdownReportService,
    pub pdf: PdfReportService,
}

pub struct AppState {
    pub scanner_services: ScannerServices,
    pub report_services: ReportServices,
}

impl AppState {
    pub fn new(pool: Pool<PgConnection>, config: AppConfig) -> Self {
        Self {
            scanner_services: ScannerServices {
                nuclei: NucleiService::new(pool.clone(), config.clone()),
                nmap: NmapService::new(pool.clone(), config.clone()),
                gowitness: GowitnessService::new(pool.clone(), config.clone()),
            },
            report_services: ReportServices {
                docx: DocxReportService::new(pool.clone()),
                markdown: MarkdownReportService::new(pool.clone()),
                pdf: PdfReportService::new(pool.clone()),
            },
        }
    }
} 