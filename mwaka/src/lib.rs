use leptos::*;
use mwaka_aria::use_button;

#[component]
pub fn Button(
    children: Children,
    #[prop(default = {
        let (disabled, _) = create_signal(false);

        disabled
    })]
    disabled: ReadSignal<bool>,
) -> impl IntoView {
    let button_ref = use_button::<html::Button>(disabled);

    view! {
        <button disabled=move || disabled.get() ref=button_ref>{children()}</button>
    }
}
