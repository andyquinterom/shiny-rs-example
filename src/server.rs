use actix::Actor;
use serde_json::json;
use shiny_rs::session::*;
use shiny_rs::shiny_rs_derive::ShinyHandler;
use shiny_rs::session::input_pool::InputPool;
use shiny_rs::session::traits::*;
use shiny_rs::{ render_html, update_text_input, remove_html, insert_html, show_notification, changed, html, args, NS, raw_html };
use std::time::Instant;
use std::collections::HashMap;
use comrak::{ markdown_to_html, ComrakOptions };

use super::plot::{ get_plot, get_dist };

fn sample_dist(n: u64, mean: f64, sd: f64) -> Vec<f64> {
    get_dist(n as usize, mean, sd).unwrap_or_default()
}

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

#[derive(ShinyHandler)]
pub struct CustomServer {
    hb: Instant,
    pub input: InputPool,
    pub event: String,
    initialize: fn(&mut Self, session: &mut <Self as Actor>::Context),
    update: fn(&mut Self, session: &mut <Self as Actor>::Context),
    tick: fn(&mut Self, session: &mut <Self as Actor>::Context),
    f64_store: HashMap<String, Vec<f64>>,
    hb_interval: std::time::Duration,
    client_timeout: std::time::Duration
}

impl CustomServer {
    pub fn new(
        initialize: fn(&mut Self, session: &mut <Self as Actor>::Context),
        update: fn(&mut Self, session: &mut <Self as Actor>::Context),
        tick: fn(&mut Self, session: &mut <Self as Actor>::Context),
    ) -> Self {
        CustomServer {
            hb: Instant::now(),
            input: InputPool::new(),
            event: String::from("Init"),
            f64_store: HashMap::new(),
            initialize,
            update,
            tick,
            hb_interval: std::time::Duration::from_secs(5),
            client_timeout: std::time::Duration::from_secs(10),
        }
    }
}

impl Actor for CustomServer {
    type Context = ShinyContext<Self>;
    fn started(&mut self, session: &mut Self::Context) {
        self.hb(session);
    }
}

type CustomSession = ShinyContext<CustomServer>;

const MAX_MARKDOWN_SIZE: usize = 5_000;

fn markdown_module(id: &str, shiny: &mut CustomServer, session: &mut CustomSession) {
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
fn main_plot_module(id: &str, shiny: &mut CustomServer, session: &mut CustomSession, init: bool) {
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

fn insert_remove_html_module(id: &str, shiny: &mut CustomServer, session: &mut CustomSession) {
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

fn update_inputs_module(id: &str, shiny: &mut CustomServer, session: &mut CustomSession) {
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

pub fn initialize(shiny: &mut CustomServer, session: &mut CustomSession) {
    main_plot_module("main_plot", shiny, session, true);
}

pub fn update(shiny: &mut CustomServer, session: &mut CustomSession) {
    markdown_module("markdown", shiny, session);
    main_plot_module("main_plot", shiny, session, false);
    insert_remove_html_module("insert_remove_html", shiny, session);
    update_inputs_module("update_inputs", shiny, session);
}

pub fn tick(_shiny: &mut CustomServer, _session: &mut CustomSession) {
}

pub fn create_server() -> CustomServer {
    CustomServer::new(initialize, update, tick)
}
