mod add_modal;
mod list_actions;

use add_modal::AddModal;
use list_actions::ListActionsUi;

use crate::{
    atoms::buttons::{Button, ButtonSize},
    prelude::*,
};

pub struct DashboardActions {
    list_actions: Arc<ListActionsUi>,
}

impl DashboardActions {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            list_actions: ListActionsUi::new(),
        })
    }

    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .future(async {
                AUTH.check().await;
            })
            .child(state.render_add_action())
            .child(state.list_actions.render())
        })
    }

    fn render_add_action(self: &Arc<Self>) -> Dom {
        let state = self;

        static CONTAINER: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("gap", "1rem")
                .style("width", "100%")
                .style("align-items", "center")
                .style("flex-direction", "row")
                .style("padding-bottom", "1rem")
                .style("margin-bottom", "1rem")
                .style("border-bottom", &format!("1px solid {}", ColorRaw::GreyAlt1.value()))
            }
        });
        html!("div", {
            .class(&*CONTAINER)
            .child(Button::new()
                .with_size(ButtonSize::Xlg)
                .with_text(&get_text!("dashboard-actions-add-button"))
                .with_on_click(clone!(state => move || {
                    AddModal::new(state.list_actions.clone()).open();
                }))
                .render()
            )
        })
    }
}
