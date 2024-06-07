mod app;
pub mod components;
mod router;
mod pages;
mod structs;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
