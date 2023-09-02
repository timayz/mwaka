use leptos::*;

#[component]
pub fn AccordionRoot(children: Children) -> impl IntoView {
    view! {
        <>{children()}</>
    }
}
