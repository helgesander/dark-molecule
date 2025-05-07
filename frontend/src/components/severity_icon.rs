use yew::prelude::*;
use crate::utils::Severity;

#[derive(Properties, PartialEq)]
pub struct SeverityIconProps {
    pub severity: Severity,
}


#[function_component(SeverityIcon)]
pub fn severity_icon(props: &SeverityIconProps) -> Html {
    let (icon_path, color_class) = match props.severity {
        Severity::Critical => (
            "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z",
            "severity-critical"
        ),
        Severity::High => (
            "M12 5.99L19.53 19H4.47L12 5.99M12 2L1 21h22L12 2zm1 14h-2v2h2v-2zm0-6h-2v4h2v-4z",
            "severity-high"
        ),
        Severity::Medium => (
            "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z",
            "severity-medium"
        ),
        Severity::Low => (
            "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z",
            "severity-low"
        ),
    };

    html! {
        <svg 
            class={classes!("severity-icon", color_class)} 
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
            style="fill: currentColor;"
        >
            <path d={icon_path}/>
        </svg>
    }
}