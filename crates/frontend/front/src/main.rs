pub mod app;
pub mod components;
mod router;
pub mod pages;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
