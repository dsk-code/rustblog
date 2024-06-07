use crate::components::common::navbar::NavigationBar;

use yew::prelude::*;


#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <NavigationBar />
            <div class="container mx-auto text-center pt-20 space-y-5">
                <h1 class="text-7xl ">{ "Welcome to Rust Blogs" }</h1>
                <p class="text-xl">{ "We will be adding more functions in the future." }</p>
            </div>
        </>
    }
}