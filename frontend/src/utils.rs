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

pub fn get_severity_color(severity: &Severity) -> &str {
    match severity {
        Severity::Low => "text-blue-500",
        Severity::Medium => "text-yellow-500",
        Severity::High => "text-orange-500",
        Severity::Critical => "text-red-500",
    }
}

pub fn calculate_severity(cvss: f64) -> Severity {
    match cvss {
        cvss if cvss < 4.0 => Severity::Low,
        cvss if cvss < 7.0 => Severity::Medium,
        cvss if cvss < 9.0 => Severity::High,
        _ => Severity::Critical,
    }
}
