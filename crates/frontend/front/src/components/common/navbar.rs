use crate::router::Route;

use yew::prelude::*;
use yew_router::prelude::*;

const TITLE_TAB: &str = "text-4xl text-gray-600 my-auto";
const TAB: &str = "text-xl text-gray-600 hover:text-gray-400 my-auto hover:border px-5 py-2.5 hover:border-gray-400 hover:border-2 hover:rounded-lg";

#[function_component(NavigationBar)]
pub fn navigation_bar() -> Html {

    html! {
        <>
            <nav class="flex items-center bg-gray-300/20 h-20 flex justify-between">
                <div class="ml-10">
                    <Link<Route> to={Route::Home} classes={classes!(TITLE_TAB)}>{ "RUST BLOGS" }</Link<Route>>
                </div>
                <div class="flex space-x-5 mr-10">
                    <div>
                        <Link<Route> to={Route::Test} classes={classes!(TAB)}>{ "Test" }</Link<Route>>
                    </div>
                    <div>
                        <Link<Route> to={Route::Login} classes={classes!(TAB)}>{ "LogIn" }</Link<Route>>
                    </div>
                </div>  
            </nav>
        </>
    }
}