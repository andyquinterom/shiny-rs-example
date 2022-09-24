use super::*;
use my_shiny_app::AreaSelect;

pub fn server(id: &str, shiny: &mut CustomServer, session: &mut CustomSession) {
    let ns = NS!(id);
    if changed!(shiny, (ns("mouse_input"))) {
        let mouse_input: AreaSelect = shiny.input.get(&ns("mouse_input")).unwrap_or(AreaSelect {x: [0, 0], y: [0, 0]});
        let start_x = mouse_input.x[0];
        let start_y = mouse_input.y[0];
        let end_x = mouse_input.x[1];
        let end_y = mouse_input.y[1];
        render_html(session, &ns("mouse_output"), &html! {
            <div>
                <p>{"x: "}{format!("You have selected from {} to {}", start_x, end_x)}</p>
                <p>{"y: "}{format!("You have selected from {} to {}", start_y, end_y)}</p>
            </div>
        });
    }
}
