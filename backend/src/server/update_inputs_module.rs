use super::*;

pub fn server(id: &str, shiny: &mut CustomServer, session: &mut CustomSession) {
    let ns = NS!(id);
    if changed!(shiny, (ns("text1"))) {
        let val = shiny.input.get_string(&ns("text1")).unwrap_or_default();
        update_text_input(
            session,
            &ns("text2"),
            json!({
                "label": val
            })
        )
    }
    if changed!(shiny, (ns("text2"))) {
        let val = shiny.input.get_string(&ns("text2")).unwrap_or_default();
        update_text_input(
            session,
            &ns("text1"),
            json!({
                "value": val
            })
        )
    }
}


