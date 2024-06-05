pub mod components;
pub mod app;
pub mod test;
mod router;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}