pub mod app;
pub mod components;
mod router;
pub mod test;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
