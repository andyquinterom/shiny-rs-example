use super::*;
use super::super::plot::{ sample_dist, get_plot };

pub fn server(id: &str, shiny: &mut CustomServer, session: &mut CustomSession) {
    let ns = NS!(id);
    let insert_selector = format!("#{}", &ns("insert_section"));
    if changed!(shiny, (ns("remove_ui:shiny.action"))) {
        remove_html(session, &format!("{insert_selector} div"));
    }
    if changed!(shiny, (ns("insert_ui:shiny.action"))) {
        let dist1 = sample_dist(50, -1.0, 0.5);
        let dist2 = sample_dist(50, -1.0, 0.5);
        insert_html(
            session,
            &insert_selector,
            "afterBegin",
            &get_plot(&dist1, &dist2)
        )
    }
}


