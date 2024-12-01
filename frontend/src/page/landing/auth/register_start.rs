use crate::{page::landing::Landing, prelude::*};

pub struct RegisterStart {
    pub uid: Option<String>,
}

impl RegisterStart {
    pub fn new(uid: Option<String>) -> Arc<Self> {
        Arc::new(Self { uid })
    }
    pub fn render(self: &Arc<Self>) -> Dom {
        match &self.uid {
            Some(uid) => self.render_uid(uid.clone()),
            None => self.render_instructions_no_uid(),
        }
    }

    fn render_uid(self: &Arc<Self>, uid: String) -> Dom {
        html!("div", {
            .class(&*CLASS)
            .child(
                html!("div", {
                    .child(html!("div", {
                        .class([FontSize::H1.class(), ColorText::Header.class()])
                        .text(&get_text!("landing-register-header"))
                    }))
                    .child(html!("div", {
                        .class([FontSize::Lg.class(), ColorText::Header.class()])
                        .text(&get_text!("landing-register-header-step2"))
                    }))
                })
            )
            .child(html!("div", {
                .class(&*BYLINE)
                .children([
                    html!("div", {
                        .class([FontSize::H2.class(), ColorText::Byline.class()])
                        .text(&get_text!("landing-register-instructions-with-uid1"))
                    }),
                    html!("div", {
                        .class([FontSize::H2.class(), ColorText::Byline.class()])
                        .text(&get_text!("landing-register-instructions-with-uid2"))
                    }),
                ])
            }))
            .child(
                html!("script", {
                    .attrs!{
                        "async": "",
                        "src": "https://telegram.org/js/telegram-widget.js?22",
                        "data-telegram-login": "OmiSmartBot",
                        "data-size": "large",
                        "data-request-access": "write",
                    }
                    .attr("data-auth-url", &Route::Landing(Landing::Auth(AuthRoute::RegisterComplete{uid: uid.clone()})).link_ext())
                })
            )
        })
    }

    fn render_instructions_no_uid(self: &Arc<Self>) -> Dom {
        html!("div", {
            .class(&*CLASS)
            .child(
                html!("div", {
                    .child(html!("div", {
                        .class([FontSize::H1.class(), ColorText::Header.class()])
                        .text(&get_text!("landing-register-header"))
                    }))
                    .child(html!("div", {
                        .class([FontSize::Lg.class(), ColorText::Header.class()])
                        .text(&get_text!("landing-register-header-step1"))
                    }))
                })
            )
            .child(html!("div", {
                .class(&*BYLINE)
                .children([
                    html!("div", {
                        .class([FontSize::H2.class(), ColorText::Byline.class()])
                        .text(&get_text!("landing-register-instruction-no-uid1"))
                    }),
                    html!("div", {
                        .class([FontSize::H2.class(), ColorText::Byline.class()])
                        .text(&get_text!("landing-register-instruction-no-uid2"))
                    })
                ])
            }))
        })
    }
}

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
