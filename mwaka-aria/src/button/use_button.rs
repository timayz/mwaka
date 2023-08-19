use leptos::{
    html::{AnyElement, ElementDescriptor},
    *,
};

pub fn use_button<T>(disabled: ReadSignal<bool>) -> NodeRef<T>
where
    T: ElementDescriptor + 'static,
    T: Clone,
{
    let child_ref = create_node_ref::<T>();

    create_effect(move |_| {
        let button = match child_ref.get() {
            Some(e) => e.into_any(),
            _ => return,
        };

        let is_native_button = is_button(&button);
        let is_native_link =
            button.tag_name().as_str() == "A" && button.get_attribute("href").is_some();

        if !is_native_button && !is_native_link {
            let _ = button.set_attribute("role", "button");
        }
    });

    create_effect(move |_| {
        let button = match child_ref.get() {
            Some(e) => e.into_any(),
            _ => return,
        };

        let is_native_button = is_button(&button);
        let is_native_input = button.tag_name().as_str() == "INPUT";
        let is_native_link =
            button.tag_name().as_str() == "A" && button.get_attribute("href").is_some();

        if !is_native_button && !is_native_link {
            if disabled.get() {
                let _ = button.remove_attribute("tabindex");
            } else {
                let _ = button.set_attribute("tabindex", "0");
            }
        }

        if !is_native_button && !is_native_input {
            if disabled.get() {
                let _ = button.set_attribute("aria-disabled", "true");
            } else {
                let _ = button.remove_attribute("aria-disabled");
            }
        }

        if disabled.get() {
            let _ = button.set_attribute("data-disabled", "");
        } else {
            let _ = button.remove_attribute("data-disabled");
        }
    });

    child_ref
}

pub(crate) fn is_button(el: &HtmlElement<AnyElement>) -> bool {
    matches!(
        (el.tag_name().as_str(), el.get_attribute("type").as_deref()),
        ("BUTTON", _)
            | ("INPUT", Some("button"))
            | ("INPUT", Some("color"))
            | ("INPUT", Some("file"))
            | ("INPUT", Some("image"))
            | ("INPUT", Some("reset"))
            | ("INPUT", Some("submit"))
    )
}
