use super::super::plot::{ get_plot, get_dist, sample_dist };
use super::*;

fn validate_range(session: &mut CustomSession, n: u64) -> bool {
    if (1..=10000).contains(&n) {
        true
    } else {
        show_notification(
            session,
            json!({
                "html": "hola",
                "action": "",
                "deps": [],
                "closeButton": true,
                "id": generate_id(),
                "type": "error"
            })
        );
        false
    }
}


fn plot_params_module(id: &str, shiny: &mut CustomServer, session: &mut CustomSession) -> bool {
    let ns = NS!(id);
    if !changed!(shiny, (ns("n:shiny.number"), ns("mean:shiny.number"), ns("sd:shiny.number"))) { return false; }
    let n = shiny.input.get_u64(&ns("n:shiny.number")).unwrap_or(0);
    if !validate_range(session, n) { return false; };
    shiny.f64_store.insert(
        id.to_string(),
        sample_dist(
            n,
            shiny.input.get_f64(&ns("mean:shiny.number")).unwrap_or(0.0),
            shiny.input.get_f64(&ns("sd:shiny.number")).unwrap_or(0.1)
        )
    );
    true
}

// The init argument is used so that we can tell the function how to populate the f64_store.
// If this is not populated it would lead to an error. In this case the unwrap_or tells
// the function to just use an empty distribution is case of an error, which is fine.
pub fn server(id: &str, shiny: &mut CustomServer, session: &mut CustomSession, init: bool) {
    let ns = NS!(id);
    if init {
        shiny.f64_store.insert(ns("plot1"), sample_dist(500, 0.0, 0.1));
        shiny.f64_store.insert(ns("plot2"), sample_dist(500, 0.0, 0.1));
    }
    let default_distribution: Vec<f64> = Vec::new();
    let plot1_changed = plot_params_module(&ns("plot1"), shiny, session);
    let plot2_changed = plot_params_module(&ns("plot2"), shiny, session);
    if plot1_changed || plot2_changed || init {
        let dist1 = shiny.f64_store.get(&ns("plot1")).unwrap_or(&default_distribution);
        let dist2 = shiny.f64_store.get(&ns("plot2")).unwrap_or(&default_distribution);
        let my_plot = get_plot(&dist1, &dist2);
        render_html(session, &ns("plot"), &my_plot);
    }
}
