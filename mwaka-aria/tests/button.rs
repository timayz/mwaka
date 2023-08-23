use leptos::*;
use mwaka_aria::create_button;
use pretty_assertions::assert_eq;

#[test]
fn should_not_have_role_when_native_button() {
    let runtime = create_runtime();

    let (disabled, _) = create_signal(false);
    let attrs = create_button(mwaka_aria::ButtonElement::Button, disabled);

    assert_eq!(
        view! {
            <button role=attrs.role tabindex=attrs.tabindex.get() aria-disabled=attrs.aria_disabled.get() data-disabled=attrs.data_disabled.get()>
                "Click me"
            </button>
        }
        .into_view()
        .render_to_string(),
        "<button     id=\"_0-0-1\">Click me</button>"
    );

    runtime.dispose();
}

#[test]
fn should_not_have_role_when_a_with_href() {
    let runtime = create_runtime();

    let (disabled, _) = create_signal(false);
    let attrs = create_button(mwaka_aria::ButtonElement::Link(true), disabled);

    assert_eq!(
        view! {
            <a href="https://timada.co" role=attrs.role tabindex=attrs.tabindex.get() aria-disabled=attrs.aria_disabled.get()>
                "Click me"
            </a>
        }
        .into_view()
        .render_to_string(),
        "<a href=\"https://timada.co\"    id=\"_0-0-1\">Click me</a>"
    );

    runtime.dispose();
}

#[test]
fn should_have_role_and_tabindex_0_when_not_native_button() {
    let runtime = create_runtime();

    let (disabled, _) = create_signal(false);
    let attrs = create_button(mwaka_aria::ButtonElement::Other, disabled);

    assert_eq!(
        view! {
            <div role=attrs.role tabindex=attrs.tabindex.get() aria-disabled=attrs.aria_disabled.get()>
                "Click me"
            </div>
        }
        .into_view()
        .render_to_string(),
        "<div role=\"button\" tabindex=\"0\"  id=\"_0-0-1\">Click me</div>"
    );

    runtime.dispose();
}

#[test]
fn should_have_role_tabindex_0_when_a_without_href() {
    let runtime = create_runtime();

    let (disabled, _) = create_signal(false);
    let attrs = create_button(mwaka_aria::ButtonElement::Link(false), disabled);

    assert_eq!(
        view! {
            <a role=attrs.role tabindex=attrs.tabindex.get() aria-disabled=attrs.aria_disabled.get()>
                "Click me"
            </a>
        }
        .into_view()
        .render_to_string(),
        "<a role=\"button\" tabindex=\"0\"  id=\"_0-0-1\">Click me</a>"
    );

    runtime.dispose();
}

#[test]
fn should_have_correct_attributes_when_not_native_button_disabled() {
    let runtime = create_runtime();

    let (disabled, _) = create_signal(true);
    let attrs = create_button(mwaka_aria::ButtonElement::Other, disabled);

    assert_eq!(
        view! {
            <div role=attrs.role tabindex=attrs.tabindex.get() aria-disabled=attrs.aria_disabled.get()>
                "Click me"
            </div>
        }
        .into_view()
        .render_to_string(),
        "<div role=\"button\"  aria-disabled=\"true\" id=\"_0-0-1\">Click me</div>"
    );

    runtime.dispose();
}

#[test]
fn should_have_correct_attributes_when_input_disabled() {
    let runtime = create_runtime();

    let (disabled, _) = create_signal(true);
    let attrs = create_button(mwaka_aria::ButtonElement::Input(None), disabled);

    assert_eq!(
        view! {
            <input role=attrs.role tabindex=attrs.tabindex.get() aria-disabled=attrs.aria_disabled.get() />
        }
        .into_view()
        .render_to_string(),
        "<input role=\"button\"   id=\"_0-0-1\"/>"
    );

    runtime.dispose();
}

#[test]
fn should_have_data_disabled_when_native_button_disabled() {
    let runtime = create_runtime();

    let (disabled, _) = create_signal(true);
    let attrs = create_button(mwaka_aria::ButtonElement::Button, disabled);

    assert_eq!(
        view! {
            <button role=attrs.role tabindex=attrs.tabindex.get() aria-disabled=attrs.aria_disabled.get() data-disabled=attrs.data_disabled.get()>
                "Click me"
            </button>
        }
        .into_view()
        .render_to_string(),
        "<button    data-disabled=\"\" id=\"_0-0-1\">Click me</button>"
    );

    runtime.dispose();
}
