use crate::router::Route;

use yew::prelude::*;
use yew_router::prelude::*;

const TAB: &str = "text-base border-4 rounded-lg border-gray-800 p-1";
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container mx-auto text-center pt-20 space-y-5">
            <h1 class="text-7xl">{ "Welcome to Rust Blogs" }</h1>
            <p class="text-xl">{ "We will be adding more functions in the future." }</p>
            <div>
                <Link<Route> to={Route::Signup} classes={classes!(TAB)}>{ "Go Signup" }</Link<Route>>
            </div>
        </div>
    }
}