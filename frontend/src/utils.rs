#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            gloo::console::log!(format!($($arg)*));
        }
    };
}


#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Low ,
    Medium,
    High,
    Critical,
}

pub fn calculate_severity(cvss: f64) -> Severity {
    match cvss {
        cvss if cvss < 4.0 => Severity::Low,
        cvss if cvss < 7.0 => Severity::Medium,
        cvss if cvss < 9.0 => Severity::High,
        _ => Severity::Critical,
    }
}
