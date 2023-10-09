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
    let view_string = ssr::render_to_string(|| {
        view! {
            <ButtonRoot element=ButtonElement::Button on_click=None disabled=None>
                <button>"Click me"</button>
            </ButtonRoot>
        }
    });

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<button data-hk=\"0-0-3\" data-hk=\"0-0-3\">Click me</button>"
    );
}

#[test]
fn should_not_have_role_when_a_with_href() {
    let view_string = ssr::render_to_string(|| {
        view! {
            <ButtonRoot element=ButtonElement::Link(true) on_click=None disabled=None>
                <a href="https://timada.co">"Click me"</a>
            </ButtonRoot>
        }
    });

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<a href=\"https://timada.co\" data-hk=\"0-0-3\" data-hk=\"0-0-3\">Click me</a>"
    );
}

#[test]
fn should_have_role_and_tabindex_0_when_not_native_button() {
    let view_string = ssr::render_to_string(|| {
        view! {
            <ButtonRoot element=ButtonElement::Other on_click=None disabled=None>
                <div>"Click me"</div>
            </ButtonRoot>
        }
    });

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<div data-hk=\"0-0-3\" role=\"button\" tabindex=\"0\" data-hk=\"0-0-3\">Click me</div>"
    );
}

#[test]
fn should_have_role_tabindex_0_when_a_without_href() {
    let view_string = ssr::render_to_string(|| {
        view! {
            <ButtonRoot element=ButtonElement::Link(false) on_click=None disabled=None>
                <a>"Click me"</a>
            </ButtonRoot>
        }
    });

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<a data-hk=\"0-0-3\" role=\"button\" tabindex=\"0\" data-hk=\"0-0-3\">Click me</a>"
    );
}

#[test]
fn should_have_correct_attributes_when_not_native_button_disabled() {
    let view_string = ssr::render_to_string(|| {
        let (disabled, _) = create_signal(true);
        view! {
            <ButtonRoot element=ButtonElement::Other disabled=disabled on_click=None>
                <div>"Click me"</div>
            </ButtonRoot>
        }
    });

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<div data-hk=\"0-0-3\" role=\"button\" aria-disabled=\"true\" data-disabled data-hk=\"0-0-3\">Click me</div>"
    );
}

#[test]
fn should_have_correct_attributes_when_input_disabled() {
    let view_string = ssr::render_to_string(|| {
        let (disabled, _) = create_signal(true);
        view! {
            <ButtonRoot element=ButtonElement::Input(None) disabled=disabled on_click=None>
                <input/>
            </ButtonRoot>
        }
    });

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<input data-hk=\"0-0-3\" role=\"button\" data-disabled data-hk=\"0-0-3\"/>"
    );
}

#[test]
fn should_have_data_disabled_when_native_button_disabled() {
    let view_string = ssr::render_to_string(|| {
        let (disabled, _) = create_signal(true);
        view! {
            <ButtonRoot element=ButtonElement::Button disabled=disabled on_click=None>
                <button>"Click me"</button>
            </ButtonRoot>
        }
    });

    assert_eq!(
        HTML_COMMENTS.replace_all(&view_string, ""),
        "<button data-hk=\"0-0-3\" data-disabled data-hk=\"0-0-3\">Click me</button>"
    );
}
