use dominator_helpers::futures::AsyncLoader;
use shared::api::action::{
    ActionDestination, ActionDestinationId, ActionDestinationKind, AddAction, AddActionRequest,
    ListActionDestinations, ListActionDestinationsRequest,
};

use crate::{
    atoms::{
        buttons::Button,
        dropdown::Dropdown,
        label::{Label, LabelDirection, LabelSize},
        modal::Modal,
        text_area::TextArea,
    },
    prelude::*,
};

use super::list_actions::ListActionsUi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionKind {
    TelegramDm,
    TelegramGroup,
}

pub struct AddModal {
    action_kind: Mutable<Option<ActionKind>>,
    action_destination_id: Mutable<Option<ActionDestinationId>>,
    prompt: Mutable<Option<String>>,
    message: Mutable<Option<String>>,
    available_destinations: Mutable<Option<std::result::Result<Vec<ActionDestination>, String>>>,
    error: Mutable<Option<String>>,
    add_loader: AsyncLoader,
    list_actions: Arc<ListActionsUi>,
}

impl AddModal {
    pub fn new(list_actions: Arc<ListActionsUi>) -> Arc<Self> {
        Arc::new(Self {
            action_kind: Mutable::new(None),
            action_destination_id: Mutable::new(None),
            prompt: Mutable::new(None),
            message: Mutable::new(None),
            available_destinations: Mutable::new(None),
            error: Mutable::new(None),
            add_loader: AsyncLoader::new(),
            list_actions,
        })
    }

    pub fn open(self: &Arc<Self>) {
        let state = self;

        Modal::open(clone!(state => move || {
            html!("div", {
                .future(clone!(state => async move {
                    match ListActionDestinations::fetch(ListActionDestinationsRequest{cursor: None}).await {
                        Ok(resp) => {
                            state.available_destinations.set(Some(Ok(resp.destinations)));
                        },
                        Err(err) => {
                            state.available_destinations.set(Some(Err(err.to_string())));
                        }
                    }
                }))
                .child_signal(state.error.signal_cloned().map(|error| {
                    error.map(|error| {
                        html!("div", {
                            .class([FontSize::H2.class(), ColorText::Error.class()])
                            .text(&error)
                        })
                    })
                }))
                .child_signal(state.available_destinations.signal_cloned().map(clone!(state => move |available_destinations| {
                    Some(match available_destinations {
                        None => html!("div", {
                            .class(FontSize::H2.class())
                            .text(&get_text!("dashboard-loading"))
                        }),
                        Some(resp) => match resp {
                            Ok(available_destinations) => state.render_with_available_destinations(available_destinations),
                            Err(error) => {
                                html!("div", {
                                    .class([FontSize::H2.class(), ColorText::Error.class()])
                                    .text(&error)
                                })
                            }
                        }
                    })
                })))
            })
        }))
    }

    fn render_with_available_destinations(
        self: &Arc<Self>,
        available_destinations: Vec<ActionDestination>,
    ) -> Dom {
        let state = self;

        static INPUTS: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("justify-content", "center")
                .style("gap", "1rem")
            }
        });
        static INPUT_ROW: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("justify-content", "center")
                .style("gap", "1rem")
            }
        });

        html!("div", {
            .class(&*INPUTS)
            .child(html!("div", {
                .class(&*INPUT_ROW)
                .child(Label::new()
                    .with_direction(LabelDirection::Column)
                    .with_size(LabelSize::Lg)
                    .with_text(&get_text!("dashboard-actions-add-kind-label"))
                    .render(Dropdown::new()
                        .with_bg_color(ColorBackground::ModalContent)
                        .with_options([
                            (get_text!("dashboard-actions-add-kind-tg-dm"), ActionKind::TelegramDm),
                            (get_text!("dashboard-actions-add-kind-tg-group"), ActionKind::TelegramGroup),
                        ])
                        .with_on_change(clone!(state => move |value| {
                            state.action_kind.set_neq(Some(value.clone()));
                        }))
                        .render()
                    )
                )
                .child_signal(state.action_kind.signal().map(clone!(state => move |action_kind| {
                    action_kind.map(|action_kind| {
                        state.action_destination_id.set(None);
                        match action_kind {
                            ActionKind::TelegramDm => html!("div", {
                                .child(Label::new()
                                    .with_direction(LabelDirection::Column)
                                    .with_size(LabelSize::Lg)
                                    .with_text(&get_text!("dashboard-actions-add-destination-tg-dm-label"))
                                    .render(Dropdown::new()
                                        .with_bg_color(ColorBackground::ModalContent)
                                        .with_options(available_destinations.iter().filter_map(|destination| {
                                            match &destination.kind {
                                                ActionDestinationKind::TelegramDm { .. } => Some((destination.name.clone(), destination.id.clone())),
                                                _ => None,
                                            }
                                        }))
                                        .with_on_change(clone!(state => move |id| {
                                            state.action_destination_id.set(Some(id.clone()));
                                        }))
                                        .render()
                                    )
                                )
                            }),
                            ActionKind::TelegramGroup => html!("div", {
                                .child(Label::new()
                                    .with_direction(LabelDirection::Column)
                                    .with_size(LabelSize::Lg)
                                    .with_text(&get_text!("dashboard-actions-add-destination-tg-group-label"))
                                    .render(Dropdown::new()
                                        .with_on_change(|_| {})
                                        .with_bg_color(ColorBackground::ModalContent)
                                        .with_options(available_destinations.iter().filter_map(|destination| {
                                            match &destination.kind {
                                                ActionDestinationKind::TelegramGroup { .. } => Some((destination.name.clone(), destination.id.clone())),
                                                _ => None,
                                            }
                                        }))
                                        .with_on_change(clone!(state => move |id| {
                                            state.action_destination_id.set(Some(id.clone()));
                                        }))
                                        .render()
                                    )
                                )
                            }),
                        }
                    })
                })))
            }))
            .child(html!("div", {
                .class(&*INPUT_ROW)
                .child(Label::new()
                    .with_direction(LabelDirection::Column)
                    .with_size(LabelSize::Lg)
                    .with_text(&get_text!("dashboard-actions-add-prompt"))
                    .render(TextArea::new()
                        .with_on_input(clone!(state => move |text| {
                            state.prompt.set(text);
                        }))
                        .render()
                    )
                )
                .child(Label::new()
                    .with_direction(LabelDirection::Column)
                    .with_size(LabelSize::Lg)
                    .with_text(&get_text!("dashboard-actions-add-message"))
                    .render(TextArea::new()
                        .with_on_input(clone!(state => move |text| {
                            state.message.set(text);
                        }))
                        .render()
                    )
                )
            }))
            .child(html!("div", {
                .style("display", "flex")
                .style("justify-content", "center")
                .child(Button::new()
                    .with_disabled_signal(state.submit_disabled_signal())
                    .with_text(&get_text!("dashboard-actions-add-submit"))
                    .with_on_click(clone!(state => move || {
                        state.error.set(None);

                        match (
                            state.action_destination_id.get_cloned(),
                            state.prompt.get_cloned(),
                            state.message.get_cloned(),
                        ) {
                            (Some(action_destination_id), Some(prompt), Some(message)) => {
                                state.add_loader.load(clone!(state, action_destination_id, prompt, message => async move {
                                    match AddAction::fetch(AddActionRequest {
                                        destination_id: action_destination_id,
                                        prompt,
                                        message,
                                    }).await {
                                        Ok(resp) => {
                                            state.list_actions.add_action(resp.action);
                                            Modal::close();
                                        },
                                        Err(err) => {
                                            state.error.set(Some(err.to_string()));
                                        }
                                    }
                                }));
                            }
                            _ => {}
                        }
                        Modal::close();
                    }))
                    .render()
                )
            }))
            .child_signal(state.add_loader.is_loading().map(|loading| {
                if loading {
                    Some(html!("div", {
                        .class(FontSize::H2.class())
                        .text(&get_text!("dashboard-loading"))
                    }))
                } else {
                    None
                }
            }))
        })
    }

    fn submit_disabled_signal(self: &Arc<Self>) -> impl Signal<Item = bool> {
        map_ref! {
            let action_kind = self.action_kind.signal(),
            let action_destination_id = self.action_destination_id.signal_cloned(),
            let prompt = self.prompt.signal_cloned(),
            let message = self.message.signal_cloned(),
            => {
                action_kind.is_none() || prompt.is_none() || message.is_none() || action_destination_id.is_none()
            }
        }
    }
}
