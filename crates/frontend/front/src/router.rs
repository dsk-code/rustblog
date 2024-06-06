use crate::test::Test;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/signup")]
    Signup,
    #[at("/test")]
    Test,
}

pub fn switch(routers: Route) -> Html {
    match routers {
        Route::Home => html! {
            <>
                <h1>{ "Home" }</h1>
                <Link<Route> to={Route::Signup}>{ "Go Signup" }</Link<Route>>
            </>
        },
        Route::Signup => html! {
            <>
                <h1>{ "Signup" }</h1>
                <Link<Route> to={Route::Home}>{ "Go Home" }</Link<Route>>
            </>
        },
        Route::Test => html! {
            <Test />
        },
    }
}
