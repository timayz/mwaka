use std::rc::Rc;

use leptos::{
    ev::{KeyboardEvent, MouseEvent},
    *,
};

pub enum ButtonElement {
    Input(Option<&'static str>),
    Link(bool),
    Button,
    Other,
}

pub enum ButtonEvent {
    KeyboardEvent(KeyboardEvent),
    MouseEvent(MouseEvent),
}

#[component]
pub fn ButtonRoot(
    children: Children,
    element: ButtonElement,
    #[prop(into)] disabled: Option<ReadSignal<bool>>,
    on_click: Option<Box<dyn Fn(ButtonEvent)>>,
) -> impl IntoView {
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
        let disabled = disabled.map(|v| v.get()).unwrap_or_default();

        if !is_native_button && !is_link {
            set_tabindex.set(if disabled { None } else { Some("0".to_owned()) });
        }

        if !is_native_button && !is_input {
            set_aria_disabled.set(if disabled {
                Some("true".to_owned())
            } else {
                None
            });
        }

        set_data_disabled.set(if disabled { Some("".to_owned()) } else { None });
    });

    let on_click = on_click.map(|on_click| Rc::new(on_click));

    children().nodes.first().map(|node| {
        node.as_element().map(|element| {
            let mut element_aria = element
                .clone()
                .into_html_element()
                .attr("role", role.to_owned())
                .attr("tabindex", move || tabindex.get())
                .attr("aria-disabled", move || aria_disabled.get())
                .attr("data-disabled", move || data_disabled.get());

            if !is_native_button && !is_link {
                element_aria = element_aria
                    .optional_event(ev::click, {
                        on_click.clone().map(|on_click| {
                            move |e: MouseEvent| {
                                let disabled = disabled.map(|v| v.get()).unwrap_or_default();

                                if !disabled {
                                    on_click(ButtonEvent::MouseEvent(e));
                                    return;
                                }

                                e.prevent_default();
                                e.stop_propagation();
                            }
                        })
                    })
                    .optional_event(ev::keypress, {
                        on_click.map(|on_click| {
                            move |e: KeyboardEvent| {
                                let disabled = disabled.map(|v| v.get()).unwrap_or_default();

                                if disabled {
                                    e.prevent_default();
                                    e.stop_propagation();

                                    return;
                                }

                                if e.key() == "Enter" {
                                    on_click(ButtonEvent::KeyboardEvent(e));
                                }
                            }
                        })
                    });
            }

            element_aria.into_view()
        })
    })
}
