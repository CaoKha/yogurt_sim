use yew::{function_component, use_state, html, Children, Properties, ContextProvider, use_context};

#[derive(Clone, Debug, PartialEq)]
pub struct Store {
    pub foreground: String,
    pub background: String,
}

pub fn use_store() -> Store {
    use_context::<Store>().unwrap()
}

#[derive(Properties, PartialEq)]
pub struct ChildrenProps {
    pub children: Children,
}

#[function_component(StoreProvider)]
pub fn store_provider(ChildrenProps { children }: &ChildrenProps) -> Html {
    let ctx = use_state(|| Store {
        foreground: "#000111".to_owned(),
        background: "#eeeeee".to_owned(),
    });

    html! {
        <ContextProvider<Store> context={(*ctx).clone()}>
            {for children.iter()}
        </ContextProvider<Store>>
    }
}
