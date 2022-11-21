use super::button::CustomButton;
use super::text_input::CustomTextInput;
use crate::hooks::store::use_store;
use std::ops::Deref;

use gloo::console::log;
use yew::prelude::*;

#[derive(Default, Clone)]
struct Data {
    pub username: String,
    pub count: u16,
}

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let state = use_state(|| Data::default());
    let username_changed = {
        let cloned_state = state.clone();
        Callback::from(move |username| {
            cloned_state.set(Data {
                username,
                ..cloned_state.deref().clone()
            });
        })
    };

    let button_clicked = {
        let cloned_state = state.clone();
        Callback::from(move |_| {
            let mut data = cloned_state.deref().clone();
            data.count += 1;
            cloned_state.set(data);
        })
    };

    let theme = use_store();
    log!(theme.foreground);
    html! {
        <form>
         <CustomTextInput name="username" on_username_change={username_changed} />
         <CustomButton label="Submit" onclick={button_clicked} />
         <p>{"Username: "}{&state.username}</p>
         <p>{"Button has been clicked "}{&state.count}{" times"}</p>
        </form>
    }
}
