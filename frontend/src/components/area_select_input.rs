use yew::prelude::*;
use shiny_rs_yew::set_input_value;
use web_sys::{HtmlElement, MouseEvent};
use wasm_bindgen::JsCast;

#[derive(PartialEq, Properties)]
pub struct AreaSelectInputProps {
    pub id: String,
    #[prop_or_default]
    pub children: Children,
}

pub enum Msg {
    Clicked,
    Hover(i32, i32)
}

pub struct AreaSelectInput {
    x: [i32; 2],
    y: [i32; 2],
    n_clicks: u32,
}

fn get_mouse_position(e: MouseEvent) -> (i32, i32) {
    let rect = e
        .target()
        .expect("mouse event doesn't have a target")
        .dyn_into::<HtmlElement>()
        .expect("event target should be of type HtmlElement")
        .get_bounding_client_rect();
    let x = e.client_x() - rect.left() as i32;
    let y = e.client_y() - rect.top() as i32;
    (x, y)
}

impl Component for AreaSelectInput {

    type Message = Msg;
    type Properties = AreaSelectInputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            x: [0, 0],
            y: [0, 0],
            n_clicks: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked => {
                match self.n_clicks {
                    0 => { self.n_clicks = 1; }
                    1 => { self.n_clicks = 2; }
                    _ => { self.n_clicks = 0; self.x = [0, 0]; self.y = [0, 0]; }
                }
                set_input_value(&ctx.props().id.clone(), my_shiny_app::AreaSelect {
                    x: self.x, y: self.y,
                }).unwrap_or_default()
            }
            Msg::Hover(x, y) => {
                match self.n_clicks {
                    0 => { self.x = [x, x]; self.y = [y, y]; }
                    1 => { self.x[1] = x; self.y[1] = y; }
                    _ => {  }
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|e: MouseEvent| {
            Msg::Clicked
        });
        let onmousemove = ctx.link().callback(|e: MouseEvent| {
            let (x, y) = get_mouse_position(e);
            Msg::Hover(x, y)
        });
        let mut border_style = String::default();
        let border_top = self.y[0].min(self.y[1]).to_string() + "px";
        let border_left = self.x[0].min(self.x[1]).to_string() + "px";
        let border_width = (self.x[1] - self.x[0]).abs().to_string() + "px";
        let border_height = (self.y[1] - self.y[0]).abs().to_string() + "px";
        if self.n_clicks != 0 {
            border_style = format!(r#"position: absolute; top: {border_top}; left: {border_left}; width: {border_width}; height: {border_height}; border: 3px solid red; "#);
        }
        html! {
            <div style="position: relative;" {onclick} {onmousemove}>
                { for ctx.props().children.iter() }
                <div style={border_style} />
                <div style="width: 100%; height: 100%; position: absolute; top: 0; left: 0; z-index: 1;" />
            </div>
        }
    }
}
