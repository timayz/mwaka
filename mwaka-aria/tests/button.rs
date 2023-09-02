use lazy_static::lazy_static;
use leptos::*;
use mwaka_aria::{ButtonElement, ButtonRoot};
use pretty_assertions::assert_eq;
use regex_lite::Regex;

lazy_static! {
    static ref HTML_COMMENTS: Regex = Regex::new("<\\!--.*?-->").unwrap();
}

#[test]
fn should_not_have_role_when_native_button() {
    let runtime = create_runtime();

    let view_string = view! {
        <ButtonRoot element=ButtonElement::Button on_click=None disabled=None>
            <button>"Click me"</button>
        </ButtonRoot>
    }
    .into_view()
    .render_to_string();

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<button id=\"_0-0-3\" leptos-hk=\"_0-0-3\">Click me</button>"
    );

    runtime.dispose();
}

#[test]
fn should_not_have_role_when_a_with_href() {
    let runtime = create_runtime();

    let view_string = view! {
        <ButtonRoot element=ButtonElement::Link(true) on_click=None disabled=None>
            <a href="https://timada.co">"Click me"</a>
        </ButtonRoot>
    }
    .into_view()
    .render_to_string();

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<a href=\"https://timada.co\" id=\"_0-0-3\" leptos-hk=\"_0-0-3\">Click me</a>"
    );

    runtime.dispose();
}

#[test]
fn should_have_role_and_tabindex_0_when_not_native_button() {
    let runtime = create_runtime();

    let view_string = view! {
        <ButtonRoot element=ButtonElement::Other on_click=None disabled=None>
            <div>"Click me"</div>
        </ButtonRoot>
    }
    .into_view()
    .render_to_string();

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<div id=\"_0-0-3\" role=\"button\" tabindex=\"0\" leptos-hk=\"_0-0-3\">Click me</div>"
    );

    runtime.dispose();
}

#[test]
fn should_have_role_tabindex_0_when_a_without_href() {
    let runtime = create_runtime();

    let view_string = view! {
        <ButtonRoot element=ButtonElement::Link(false) on_click=None disabled=None>
            <a>"Click me"</a>
        </ButtonRoot>
    }
    .into_view()
    .render_to_string();

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<a id=\"_0-0-3\" role=\"button\" tabindex=\"0\" leptos-hk=\"_0-0-3\">Click me</a>"
    );

    runtime.dispose();
}

#[test]
fn should_have_correct_attributes_when_not_native_button_disabled() {
    let runtime = create_runtime();

    let (disabled, _) = create_signal(true);

    let view_string = view! {
        <ButtonRoot element=ButtonElement::Other disabled=disabled on_click=None>
            <div>"Click me"</div>
        </ButtonRoot>
    }
    .into_view()
    .render_to_string();

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<div id=\"_0-0-3\" role=\"button\" aria-disabled=\"true\" data-disabled leptos-hk=\"_0-0-3\">Click me</div>"
    );

    runtime.dispose();
}

#[test]
fn should_have_correct_attributes_when_input_disabled() {
    let runtime = create_runtime();

    let (disabled, _) = create_signal(true);

    let view_string = view! {
        <ButtonRoot element=ButtonElement::Input(None) disabled=disabled on_click=None>
            <input />
        </ButtonRoot>
    }
    .into_view()
    .render_to_string();

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<input id=\"_0-0-3\" role=\"button\" data-disabled leptos-hk=\"_0-0-3\"/>"
    );

    runtime.dispose();
}

#[test]
fn should_have_data_disabled_when_native_button_disabled() {
    let runtime = create_runtime();

    let (disabled, _) = create_signal(true);

    let view_string = view! {
        <ButtonRoot element=ButtonElement::Button disabled=disabled on_click=None>
            <button>"Click me"</button>
        </ButtonRoot>
    }
    .into_view()
    .render_to_string();

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<button id=\"_0-0-3\" data-disabled leptos-hk=\"_0-0-3\">Click me</button>"
    );

    runtime.dispose();
}
