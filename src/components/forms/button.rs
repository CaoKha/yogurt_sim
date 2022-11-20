use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    pub onclick: Callback<()>
}

#[function_component(CustomButton)]
pub fn custom_button(ButtonProps { label, onclick }: &ButtonProps) -> Html {
    let onclickclone = onclick.clone();
    let click_callback = Callback::from(move |_| {
        onclickclone.emit(());
    });
    html! {
        <button onclick={click_callback}>{label}</button>
    }
}
