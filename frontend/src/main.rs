use yew::prelude::*;
use shiny_rs_yew::*;
mod dependencies;
use dependencies::LoadExternalDependencies;
mod components;
use components::AreaSelectInput;

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

fn custom_input(id: String) -> Html {
    let ns = NS!(id);
    html! {
        <div class="row">
            <h1>{"Custom inputs!"}</h1>
            <p>{"This is an example of a custom input built with Yew and Shiny. Click once to select a starting position, a second time to select the area, and a third time to reset."}</p>
            <hr/ >
            <div class="container row">
                <div class="col-md-auto">
                    <div style="border: 1px solid black; margin: 10px; width: min-content; height: min-content;">
                        <AreaSelectInput id={ns("mouse_input")}>
                            <img width=500 src="https://rustacean.net/assets/rustacean-orig-noshadow.png" />
                        </AreaSelectInput>
                    </div>
                </div>
                <div class="col-md-auto">
                    <h3>{"Output"}</h3>
                    <HtmlOutput id={ns("mouse_output")} />
                </div>
            </div>
        </div>
    }
}



#[function_component(App)]
fn app() -> Html {
    html! {
        <ShinyApp title="Shiny in Rust">
            <LoadExternalDependencies />
            <link rel="stylesheet" href="/lib/custom.css" />
            <PageNavbar title="Shiny-RS">
                <Nav active=true title="Reactive Plot" id="inicio">
                    {main_plot_module("main_plot".to_string())}
                </Nav>
                <Nav title="Custom input" id="custom_input" >
                    {custom_input("custom_input".to_string())}
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
        </ShinyApp>
    }
}

fn main() {
    yew::start_app::<App>();
}
