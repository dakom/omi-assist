use crate::{atoms::buttons::Button, page::landing::Landing, prelude::*};

pub struct Welcome {
    uid: Option<String>,
}

impl Welcome {
    pub fn new(uid: Option<String>) -> Arc<Self> {
        Arc::new(Self { uid })
    }
    pub fn render(self: Arc<Self>) -> Dom {
        let state = self;

        static CLASS: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("gap", "1.3rem")
                .style("align-items", "center")
                .style("margin-top", "5.25rem")
                .style("text-align", "center")
            }
        });

        static BYLINE: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("gap", ".3rem")
                .style("align-items", "center")
                .style("text-align", "center")
            }
        });

        static BUTTONS: LazyLock<String> = LazyLock::new(|| {
            class! {
                .style("display", "flex")
                .style("flex-direction", "row")
                .style("margin-top", "1.5rem")
                .style("gap", "2rem")
                .style("align-items", "center")
                .style("text-align", "center")
            }
        });

        html!("div", {
            .class(&*CLASS)
            .child(html!("div", {
                .class([FontSize::H1.class(), ColorText::Header.class()])
                .text(&get_text!("landing-welcome-header"))
            }))
            .child(html!("div", {
                .class(&*BYLINE)
                .children([
                    html!("div", {
                        .class([FontSize::H2.class(), ColorText::Byline.class()])
                        .text(&get_text!("landing-welcome-byline1"))
                    }),
                    html!("div", {
                        .class([FontSize::H2.class(), ColorText::Byline.class()])
                        .text(&get_text!("landing-welcome-byline2"))
                    }),
                ])
            }))
            .child(html!("div", {
                .class(&*BUTTONS)
                .children([
                    Button::new()
                        .with_text(&get_text!("landing-welcome-register"))
                        .with_link({
                            Route::Landing(Landing::Auth(AuthRoute::RegisterStart{uid: state.uid.clone()}))
                        })
                        .render(),
                    html!("script", {
                        .attrs!{
                            "async": "",
                            "src": "https://telegram.org/js/telegram-widget.js?22",
                            "data-telegram-login": "OmiSmartBot",
                            "data-size": "large",
                            "data-request-access": "write",
                        }
                        .attr("data-auth-url", &Route::Landing(Landing::Auth(AuthRoute::Signin)).link_ext())
                    })
                ])
            }))
        })
    }
}
