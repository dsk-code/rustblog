use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct InputFieldProps {}

#[function_component(InputField)]
pub fn input_field() -> Html {
    
    html! {
        <>
            <input type="text" placeholder="ユーザーネーム" class="border-2 border-gray-300 rounded w-72 min-h-6 pt-6 pb-2 px-4 bg-black/20 text-white"/>
            <input type="password" placeholder="パスワード" class="border-2 border-gray-300 rounded w-72 min-h-6 pt-6 pb-2 px-4 bg-black/20 text-white"/>
        </>
    }
}