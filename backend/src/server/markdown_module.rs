use super::*;

const MAX_MARKDOWN_SIZE: usize = 5_000;

pub fn server(id: &str, shiny: &mut CustomServer, session: &mut CustomSession) {
    let ns = NS!(id);
    if changed!(shiny, (ns("markdown"), ns("style"))) {
        let md_string = shiny.input.get_string(&ns("markdown")).unwrap_or_default();
        let error_html = format!("The markdown has exceeded {MAX_MARKDOWN_SIZE} characters. Please reduce the size of the markdown.");
        if md_string.len() > MAX_MARKDOWN_SIZE {
            show_notification(session, args!({
                "html": error_html,
                "id": &ns("markdown_warning"),
                "type": "error",
                "closeButton": true
            }));
            return;
        }
        let rendered_markdown = markdown_to_html(&md_string, &ComrakOptions::default());
        let document_style = match shiny.input.get_string(&ns("style")).unwrap_or_default().as_str() {
            "light" => "bg-light text-dark p-3",
            "dark" => "bg-dark text-light p-3",
            _ => "bg-light text-dark"
        };
        render_html(session, &ns("rendered_md"), &html! {
            <div>
                <div class={document_style}>{raw_html!(rendered_markdown.as_str())}</div>
                <p class={"text-danger"}>
                    {format!("You have {} characters left!", MAX_MARKDOWN_SIZE - md_string.len())}
                </p>
            </div>
        });
    }
}
