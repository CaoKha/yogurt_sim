use yew::prelude::*;

use crate::hooks::store::Store;


#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    pub onclick: Callback<()>,
}

#[function_component(CustomButton)]
pub fn custom_button(ButtonProps { label, onclick }: &ButtonProps) -> Html {
    let click_callback = {
        let onclickclone = onclick.clone();
        Callback::from(move |_| onclickclone.emit(()))
    };
    html! {
        <button onclick={click_callback}>{label}</button>
    }
}
