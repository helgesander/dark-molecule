
enum ReportFormat {
    Markdown,
    Pdf,
    Docx
}
pub struct ReportService;

impl ReportService {
    // pub fn generate_markdown(data: &ReportTemplate, path_to_template: String, project: &Project) -> String {
    //     // let reg = Handlebars::new();
    //     // let template = include_str!(path_to_template.as_str());
    //     // reg.render_template(template, &json!(data)).unwrap()
    //     unimplemented!()
    //     // TODO: fix this function
    // }
    //
    // pub fn generate_docx(data: &ReportTemplate) -> Vec<u8> {
    //     // Используем docx-rs или pandoc
    //     unimplemented!()
    // }
    //
    // pub fn generate_pdf(data: &ReportTemplate) -> Vec<u8> {
    //     // Используем printpdf или wkhtmltopdf
    //     unimplemented!()
    // }
    //
    // pub async fn generate_report(
    //     data: &ReportTemplate,
    //     format: ReportFormat,
    // ) -> Result<Vec<u8>, dyn Error> {
    //     // match format {
    //     //     ReportFormat::Markdown => Ok(Self::generate_markdown(&data).into_bytes()),
    //     //     ReportFormat::Pdf => Ok(Self::generate_pdf(&data)),
    //     //     ReportFormat::Docx => Ok(Self::generate_docx(&data)),
    //     // }
    //     unimplemented!()
    // }
}