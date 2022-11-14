use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String,
}

#[function_component(CustomButton)]
pub fn text_input(ButtonProps { label }: &ButtonProps) -> Html {
    html! {
        <button>{label}</button>
    }
}
