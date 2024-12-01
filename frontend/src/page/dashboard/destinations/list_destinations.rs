use shared::api::action::{
    ActionDestination, ActionDestinationKind, ListActionDestinations, ListActionDestinationsRequest,
};

use crate::prelude::*;

pub struct ListDestinationsUi {
    pub destinations: Mutable<Option<Vec<ActionDestination>>>,
    pub error: Mutable<Option<String>>,
}

impl ListDestinationsUi {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            destinations: Mutable::new(None),
            error: Mutable::new(None),
        })
    }

    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;

        static LIST: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("gap", "1rem")
                .style("margin-top", "1rem")
                .style("width", "100%")
                .style("justify-content", "flex-start")
                .style("flex-direction", "column")
            }
        });

        html!("div", {
            .future(clone!(state => async move {
                match ListActionDestinations::fetch(ListActionDestinationsRequest{ cursor: None }).await {
                    Ok(resp) => {
                        state.destinations.set(Some(resp.destinations));
                    },
                    Err(err) => {
                        state.error.set(Some(err.to_string()));
                    }
                }
            }))
            .class(FontSize::Xlg.class())
            .text(&get_text!("dashboard-destinations-list-title"))
            .child_signal(state.error.signal_cloned().map(|error| {
                error.map(|error| {
                    html!("div", {
                        .class([FontSize::H2.class(), ColorText::Error.class()])
                        .text(&error)
                    })
                })
            }))
            .child_signal(state.destinations.signal_cloned().map(|destinations| {
                destinations.map(|destinations| {
                    html!("div", {
                        .class(&*LIST)
                        .children(destinations.iter().map(|destination| {
                            html!("div", {
                                .class([&*FontSize::Xlg.class()])
                                .children(&mut [
                                    html!("div", {
                                        .text(&format!("{}: {}", get_text!("dashboard-destinations-list-id"), destination.id.to_string()))
                                    }),
                                    html!("div", {
                                        .text(&match &destination.kind {
                                            ActionDestinationKind::TelegramDm { .. } => format!("{}: {}", get_text!("dashboard-destinations-tg-dm-label"), destination.name),
                                            ActionDestinationKind::TelegramGroup { .. } => format!("{}: {}", get_text!("dashboard-destinations-tg-group-label"), destination.name),
                                        })
                                    }),
                                ])
                            })
                        }))
                    })
                })
            }))
        })
    }
}
