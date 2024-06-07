use crate::router::Route;

use yew::prelude::*;
use yew_router::prelude::*;

const SIGNUP_TAB: &str = "text-white font-medium text-base pb-0 hover:border-b";
#[derive(Properties, PartialEq)]
pub struct AuthFormProps {
    pub children: Html,
}

#[function_component(AuthForm)]
pub fn auth_form(props: &AuthFormProps) -> Html {
    
    html! {
        <>
            <form class="container mx-auto bg-black/80 flex items-center justify-center flex-col min-h-96 max-w-96 rounded-lg mt-20 space-y-6">
                <h1 class="text-3xl text-white font-bold">{"ログイン"}</h1>
                { props.children.clone() }
                <button class="bg-blue-500 text-white w-72 h-11 rounded">
                    {"ログイン"}
                </button>
                <div>
                    <span class="text-gray-300">{"初めての方はこちら "}</span>
                    <Link<Route> to={Route::Signup} classes={classes!(SIGNUP_TAB)}>{ "新規登録" }</Link<Route>>
                </div>
            </form>
        </>
    }
}