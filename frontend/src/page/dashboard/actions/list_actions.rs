use shared::api::action::{
    Action, ActionDestinationKind, DeleteAction, DeleteActionRequest, ListActions,
    ListActionsRequest, ListActionsResponse,
};

use crate::{
    atoms::{
        buttons::{Button, ButtonColor},
        modal::Modal,
    },
    prelude::*,
    util::signal::enumerate_signal,
};

pub struct ListActionsUi {
    pub actions: Mutable<Option<MutableVec<Action>>>,
    pub error: Mutable<Option<String>>,
}

impl ListActionsUi {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            actions: Mutable::new(None),
            error: Mutable::new(None),
        })
    }

    pub fn add_action(&self, action: Action) {
        self.actions
            .lock_mut()
            .as_mut()
            .unwrap()
            .lock_mut()
            .push_cloned(action);
    }

    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;

        static LIST: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("gap", "2rem")
                .style("margin-top", "1rem")
                .style("width", "100%")
                .style("justify-content", "flex-start")
                .style("flex-direction", "column")
            }
        });

        html!("div", {
            .future(clone!(state => async move {
                match ListActions::fetch(ListActionsRequest { cursor: None }).await {
                    Ok(ListActionsResponse{actions}) => {
                        state.actions.set(Some(MutableVec::new_with_values(actions)));
                    },
                    Err(err) => {
                        state.error.set(Some(err.to_string()));
                    }
                }
            }))
            .class(FontSize::Xlg.class())
            .text(&get_text!("dashboard-actions-list-title"))
            .child_signal(state.error.signal_cloned().map(|error| {
                error.map(|error| {
                    html!("div", {
                        .class([FontSize::H2.class(), ColorText::Error.class()])
                        .text(&error)
                    })
                })
            }))
            .child_signal(state.actions.signal_cloned().map(clone!(state => move |actions| {
                actions.map(|actions| {
                    html!("div", {
                        .class(&*LIST)
                        .children_signal_vec(enumerate_signal(actions.signal_vec_cloned()).map(clone!(state => move |(action, index)| {
                            state.render_action(action, index)
                        })))
                    })
                })
            })))
        })
    }

    fn render_action(self: &Arc<Self>, action: Action, index: usize) -> Dom {
        let state = self;

        let action_id = action.id.clone();

        static CONTAINER: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("gap", "1rem")
                .style("width", "100%")
                .style("flex-direction", "column")
            }
        });

        static ROWS: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("gap", "0.2rem")
                .style("width", "100%")
                .style("flex-direction", "column")
            }
        });

        html!("div", {
            .class([&*CONTAINER, &*FontSize::Xlg.class()])
            .child(html!("div", {
                .class(&*ROWS)
                .children([
                    html!("div", {
                        .text(&format!("{}: {}", get_text!("dashboard-actions-add-id"), action.id.to_string()))
                    }),
                    html!("div", {
                        .text(&match &action.destination.kind {
                            ActionDestinationKind::TelegramDm { .. } => format!("{}: {}", get_text!("dashboard-destinations-tg-dm-label"), action.destination.name),
                            ActionDestinationKind::TelegramGroup { .. } => format!("{}: {}", get_text!("dashboard-destinations-tg-group-label"), action.destination.name),
                        })
                    }),
                    html!("div", {
                        .text(&format!("{}: {}", get_text!("dashboard-actions-add-prompt"), action.prompt))
                    }),
                    html!("div", {
                        .text(&format!("{}: {}", get_text!("dashboard-actions-add-message"), action.message))
                    })
                ])
            }))
            .child(Button::new()
                .with_color(ButtonColor::Red)
                .with_text(&get_text!("dashboard-actions-delete-button"))
                .with_on_click(clone!(state, action_id, index => move || {
                    Modal::open(clone!(state, action_id, index => move || {
                        let error:Mutable<Option<String>> = Mutable::new(None);
                        html!("div", {
                            .future(clone!(state, action_id, index, error => async move {
                                match DeleteAction::fetch(DeleteActionRequest{id: action_id}).await {
                                    Ok(_) => {
                                        state.actions.lock_ref().as_ref().unwrap().lock_mut().remove(index);
                                        Modal::close();
                                    },
                                    Err(err) => {
                                        error.set(Some(err.to_string()));
                                    }
                                }
                            }))
                            .child_signal(error.signal_cloned().map(|error| {
                                Some(match error {
                                    Some(error) => {
                                        html!("div", {
                                            .class([FontSize::H2.class(), ColorText::Error.class()])
                                            .text(&error)
                                        })
                                    },
                                    None => {
                                        html!("div", {
                                            .class([FontSize::H2.class()])
                                            .text(&get_text!("dashboard-please-wait"))
                                        })
                                    }
                                })
                            }))
                        })
                    }));
                }))
                .render()
            )
        })
    }
}
