use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: String,
}

#[function_component(CustomTextInput)]
pub fn text_input(TextInputProps { name }: &TextInputProps) -> Html {
    html! {
        <input type="text" name={name.clone()} />
    }
}
