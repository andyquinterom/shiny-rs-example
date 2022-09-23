use yew_interop::{ declare_resources, ScriptEffect };
use yew::prelude::*;

declare_resources!(
    ! leaflet_js
    "https://unpkg.com/leaflet@1.8.0/dist/leaflet.js"
    ! plotly
    "https://cdn.plot.ly/plotly-2.14.0.min.js"
);

#[function_component(UseLeaflet)]
fn load_leaflet() -> Html {
    let script = use_leaflet_js();
    html! {
        <>
        <link rel="stylesheet" href="https://unpkg.com/leaflet@1.8.0/dist/leaflet.css" />
        if let Some(script) = script { <ScriptEffect {script} /> } else { }
        </>
    }
}

#[function_component(UsePlotly)]
fn load_plotly() -> Html {
    let script = use_plotly();
    html! {
        if let Some(script) = script { <ScriptEffect {script} /> } else { }
    }
}

#[function_component(LoadExternalDependencies)]
pub fn load_external_dependencies() -> Html {
    html! {
        <ResourceProvider>
            <UseLeaflet />
            <UsePlotly />
        </ResourceProvider>
    }
}
