use leptos::{ev::KeyboardEvent, *};

pub enum ButtonElement {
    Input(Option<&'static str>),
    Link(bool),
    Button,
    Other,
}

pub struct ButtonAttrs {
    pub role: Option<String>,
    pub tabindex: ReadSignal<Option<String>>,
    pub aria_disabled: ReadSignal<Option<String>>,
    pub data_disabled: ReadSignal<Option<String>>,
    pub on_keypress: Box<dyn Fn(KeyboardEvent)>,
}

pub fn create_button(element: ButtonElement, disabled: ReadSignal<bool>) -> ButtonAttrs {
    let is_native_button = matches!(
        element,
        ButtonElement::Button
            | ButtonElement::Input(Some("button"))
            | ButtonElement::Input(Some("color"))
            | ButtonElement::Input(Some("file"))
            | ButtonElement::Input(Some("image"))
            | ButtonElement::Input(Some("reset"))
            | ButtonElement::Input(Some("submit"))
    );

    let is_link = matches!(element, ButtonElement::Link(true));
    let is_input = matches!(element, ButtonElement::Input(_));

    let role = if !is_native_button && !is_link {
        Some("button".to_owned())
    } else {
        None
    };

    let (tabindex, set_tabindex) = create_signal(None::<String>);
    let (aria_disabled, set_aria_disabled) = create_signal(None::<String>);
    let (data_disabled, set_data_disabled) = create_signal(None::<String>);

    create_isomorphic_effect(move |_| {
        if !is_native_button && !is_link {
            set_tabindex.set(match disabled.get() {
                false => Some("0".to_owned()),
                true => None,
            });
        }

        if !is_native_button && !is_input {
            set_aria_disabled.set(match disabled.get() {
                true => Some("true".to_owned()),
                false => None,
            });
        }

        set_data_disabled.set(match disabled.get() {
            true => Some("".to_owned()),
            false => None,
        });
    });

    let on_keypress = Box::new(move |e: KeyboardEvent| {
        if e.key() == "Enter" && !disabled.get() {

        }
    });

    ButtonAttrs {
        role,
        tabindex,
        aria_disabled,
        data_disabled,
        on_keypress,
    }
}
