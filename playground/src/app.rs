use crate::error_template::{AppError, ErrorTemplate};
use leptos::{
    html::{AnyElement, ElementDescriptor},
    *,
};
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/playground.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let btn_ref = use_button::<html::Span>(cx);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <span ref=btn_ref>"Go to" {count}</span>
    }
}

fn use_button<T>(cx: Scope) -> NodeRef<T>
where
    T: ElementDescriptor + 'static,
    T: Clone,
{
    let child_ref = create_node_ref::<T>(cx);

    create_effect(cx, move |_| {
        let button = match child_ref.get() {
            Some(e) => e.into_any(),
            _ => return,
        };

        let is_native_button = is_button(&button);
        let is_native_input = button.tag_name().as_str() == "INPUT";
        let is_native_link =
            button.tag_name().as_str() == "A" && button.get_attribute("href").is_some();

        let disabled = button.get_attribute("disabled").is_some();

        if !is_native_button && !is_native_link {
            let _ = button.set_attribute("role", "button");

            if !disabled {
                let _ = button.set_attribute("tabindex", "0");
            } else {
                let _ = button.remove_attribute("disabled");
            }
        }

        if !is_native_button && !is_native_input && disabled {
            let _ = button.set_attribute("aria-disabled", "true");
        }

        if disabled {
            let _ = button.set_attribute("data-disabled", "");
        }
    });

    child_ref
}

fn is_button(el: &HtmlElement<AnyElement>) -> bool {
    match (el.tag_name().as_str(), el.get_attribute("type").as_deref()) {
        ("BUTTON", _) => true,
        ("INPUT", Some("button")) => true,
        ("INPUT", Some("color")) => true,
        ("INPUT", Some("file")) => true,
        ("INPUT", Some("image")) => true,
        ("INPUT", Some("reset")) => true,
        ("INPUT", Some("submit")) => true,
        _ => false,
    }
}
