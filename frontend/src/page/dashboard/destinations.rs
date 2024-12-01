mod list_destinations;
use list_destinations::ListDestinationsUi;

use crate::prelude::*;

pub struct DashboardDestinations {}

impl DashboardDestinations {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }

    pub fn render(self: &Arc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .future(async {
                AUTH.check().await;
            })
            .child(state.render_add_destination())
            .child(ListDestinationsUi::new().render())
        })
    }

    fn render_add_destination(self: &Arc<Self>) -> Dom {
        static CONTAINER: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("gap", "1rem")
                .style("width", "100%")
                .style("justify-content", "flex-start")
                .style("flex-direction", "column")
                .style("padding-bottom", "1rem")
                .style("margin-bottom", "1rem")
                .style("border-bottom", &format!("1px solid {}", ColorRaw::GreyAlt1.value()))
            }
        });
        html!("div", {
            .class([&*CONTAINER, &*FontSize::Xlg.class()])
            .children([
                html!("div", {
                    .class(&*FontWeight::Bold.class())
                    .text(&get_text!("dashboard-destinations-instructions-tg-dm-title"))
                }),
                html!("div", {
                    .text(&get_text!("dashboard-destinations-instructions-tg-dm-body"))
                }),
                html!("div", {
                    .class(&*FontWeight::Bold.class())
                    .text(&get_text!("dashboard-destinations-instructions-tg-group-title"))
                }),
                html!("div", {
                    .text(&get_text!("dashboard-destinations-instructions-tg-group-body"))
                })
            ])
        })
    }
}
