use leptos::*;
use mwaka_aria::{ButtonElement, ButtonEvent, ButtonRoot};

#[component]
pub fn Button(
    children: Children,
    #[prop(optional, into)] disabled: Option<ReadSignal<bool>>,
    #[prop(optional)] on_click: Option<Box<dyn Fn(ButtonEvent)>>,
) -> impl IntoView {
    view! {
        <ButtonRoot element=ButtonElement::Other disabled=disabled on_click=on_click>
            <button>{children()}</button>
        </ButtonRoot>
    }
}
