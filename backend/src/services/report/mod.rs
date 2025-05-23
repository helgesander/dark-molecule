pub mod docx;
pub mod markdown;
pub mod pdf;
pub mod traits;
pub mod types;

pub use markdown::service::MarkdownService;
pub use traits::ReportGenerator;
