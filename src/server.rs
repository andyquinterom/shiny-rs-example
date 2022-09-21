use actix::Actor;
use serde_json::json;
use shiny_rs::session::*;
use shiny_rs::shiny_rs_derive::ShinyHandler;
use shiny_rs::session::input_pool::InputPool;
use shiny_rs::session::traits::*;
use shiny_rs::{ run_js, render_html, update_text_input, remove_html, insert_html, show_notification, changed, html, args, NS, raw_html };
use std::time::Instant;
use std::collections::HashMap;
use comrak::{ markdown_to_html, ComrakOptions };
use rand::Rng;

mod markdown_module;
mod main_plot_module;
mod map_module;
mod insert_remove_html_module;
mod update_inputs_module;

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
            hb_interval: std::time::Duration::from_millis(200),
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
pub fn initialize(shiny: &mut CustomServer, session: &mut CustomSession) {
    main_plot_module::server("main_plot", shiny, session, true);
    map_module::server("map", shiny, session, true);
}

pub fn update(shiny: &mut CustomServer, session: &mut CustomSession) {
    markdown_module::server("markdown", shiny, session);
    main_plot_module::server("main_plot", shiny, session, false);
    insert_remove_html_module::server("insert_remove_html", shiny, session);
    update_inputs_module::server("update_inputs", shiny, session);
}

pub fn tick(shiny: &mut CustomServer, session: &mut CustomSession) {
    map_module::server("map", shiny, session, false);
}

pub fn create_server() -> CustomServer {
    CustomServer::new(initialize, update, tick)
}
