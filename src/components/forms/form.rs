use std::ops::Deref;
use super::text_input::CustomTextInput;
use super::button::CustomButton;

use yew::prelude::*;

#[derive(Default, Clone)]
struct Data {
    pub username: String,
    pub count: u16,
}

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let state = use_state(|| Data::default()); 
    let cloned_state = state.clone(); 
    let username_changed = Callback::from(move |username| {
        let mut data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let button_clicked = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.count += 1;
        cloned_state.set(data);
    });
   html! {
       <> 
        <CustomTextInput name="username" on_username_change={username_changed} />
        <CustomButton label="Submit" onclick={button_clicked} />
        <p>{"Username: "}{&state.username}</p>
        <p>{"Button has been clicked "}{&state.count}{" times"}</p>
       </>
   } 
}
