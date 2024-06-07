use crate::pages::{
    test::Test,
    home::Home,
    login::Login,
};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/signup")]
    Signup,
    #[at("/login")]
    Login,
    #[at("/test")]
    Test,
}

pub fn switch(routers: Route) -> Html {
    match routers {
        Route::Home => html! {
            <>
                <Home />     
            </>
        },
        Route::Signup => html! {
            <>
                <h1>{ "Signup" }</h1>
                <Link<Route> to={Route::Home}>{ "Go Home" }</Link<Route>>
            </>
        },
        Route::Login => html! {
            <>
                <Login />
            </>
        },
        Route::Test => html! {
            <Test />
        },
    }
}
