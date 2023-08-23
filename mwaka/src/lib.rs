use leptos::*;
use mwaka_aria::{create_button, ButtonElement};

#[component]
pub fn Button(
    children: Children,
    #[prop(default = {
        let (disabled, _) = create_signal(false);

        disabled
    })]
    disabled: ReadSignal<bool>,
) -> impl IntoView {
    let attrs = create_button(ButtonElement::Button, disabled);

    view! {
        <button disabled=move || disabled.get() data-disabled=move || attrs.data_disabled.get()>{children()}</button>
    }
}
