use crate::components::common::{
    navbar::NavigationBar,
    auth_form::AuthForm,
    input_field::InputField,
};

use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <>
            <NavigationBar />
            <AuthForm>
                <InputField />
            </AuthForm>
        </>
    }
}