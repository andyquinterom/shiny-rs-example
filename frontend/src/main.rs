use yew::prelude::*;
use shiny_rs_yew::*;

fn main_plot_module(id: String) -> Html {
    let ns = NS!(id);
    html! {
        <div class="row">
            <div class="col-md-6">
                <div class="row">
                    <div class="col-md-6">
                        <NumericInput id={ns("plot1-n")} label="Number of observations" value=500.0 min=1.0 max=10000.0 step=1.0 />
                        <NumericInput id={ns("plot1-mean")} label="µ" value=0.0 min=0.0 max=10000.0 step=1.0 />
                        <NumericInput id={ns("plot1-sd")} label="σ" value=0.1 min=0.0 max=10000.0 step=1.0 />
                    </div>
                    <div class="col-md-6">
                        <NumericInput id={ns("plot2-n")} label="Number of observations" value=500.0 min=1.0 max=10000.0 step=1.0 />
                        <NumericInput id={ns("plot2-mean")} label="µ" value=0.0 min=0.0 max=10000.0 step=1.0 />
                        <NumericInput id={ns("plot2-sd")} label="σ" value=0.1 min=0.0 max=10000.0 step=1.0 />
                    </div>
                </div>
            </div>
            <div class="col-md-6">
                <HtmlOutput id={ns("plot")} />
            </div>
        </div>
    }
}

fn map_module(id: String) -> Html {
    let ns = NS!(id);
    html! {
        <div class="row">
            <div class="col-md-12">
                <ActionButton id={ns("toggle")} label="Toggle movement" />
                <hr/>
                <div id={ns("map")} style="height: 80vh;" />
            </div>
        </div>
    }
}

fn update_inputs_module(id: String) -> Html {
    let ns = NS!(id);
    html! {
        <div class="row">
            <div class="col-md-12">
                <p>
                {r#"The first input updates the label of the second input.
                The second input updates the value of the first input,
                thereby updating its own label"#}
                </p>
                <TextInput id={ns("text1")} label="First input" />
                <TextInput id={ns("text2")} label="Second input" />
            </div>
        </div>
    }
}

fn insert_remove_html_module(id: String) -> Html {
    let ns = NS!(id);
    html! {
        <div class="row">
            <div class="col-md-12">
                <ActionButton id={ns("insert_ui")} label="Insert" />
                <ActionButton id={ns("remove_ui")} label="Remove" />
                <div id={ns("insert_section")} />
            </div>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <head>
            <title>{ "Shiny in Rust?" }</title>
        </head>
        <PageNavbar>
            <Nav active=true title="Inicio" id="inicio">
                {main_plot_module("main_plot".to_string())}
            </Nav>
            <Nav title="Insert and remove UI" id="insert_ui">
                {insert_remove_html_module("insert_remove_html".to_string())}
            </Nav>
            <Nav title="Update inputs" id="update_inputs">
                {update_inputs_module("update_inputs".to_string())}
            </Nav>
            <Nav title="Leaflet Map" id="map">
                {map_module("map".to_string())}
            </Nav>
        </PageNavbar>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
