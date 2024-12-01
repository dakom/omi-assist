use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorBackground {
    ButtonPrimary,
    ButtonPrimaryHover,
    ButtonDisabled,
    SidebarSelected,
    Sidebar,
    ModalContent,
    UnderlinePrimary,
    UnderlineSecondary,
    ButtonRed,
    ButtonRedHover,
}

impl ColorBackground {
    pub fn value(self) -> &'static str {
        match self {
            Self::ButtonPrimary => ColorRaw::Accent.value(),
            Self::ButtonPrimaryHover => ColorRaw::AccentLite.value(),
            Self::ButtonDisabled => ColorRaw::AccentVeryLight.value(),
            Self::SidebarSelected => ColorRaw::GreyAlt1.value(),
            Self::Sidebar => ColorRaw::GreyAlt2.value(),
            Self::ModalContent => ColorRaw::Whiteish.value(),
            Self::UnderlinePrimary => ColorRaw::Accent.value(),
            Self::UnderlineSecondary => ColorRaw::MidGrey.value(),
            Self::ButtonRed => ColorRaw::Red.value(),
            Self::ButtonRedHover => ColorRaw::RedLite.value(),
        }
    }

    pub fn class(self) -> &'static str {
        static BUTTON_PRIMARY: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("background-color", ColorBackground::ButtonPrimary.value())
            }
        });

        static BUTTON_PRIMARY_HOVER: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("background-color", ColorBackground::ButtonPrimaryHover.value())
            }
        });

        static BUTTON_DISABLED: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("background-color", ColorBackground::ButtonDisabled.value())
            }
        });

        static SIDEBAR_SELECTED: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("background-color", ColorBackground::SidebarSelected.value())
            }
        });

        static SIDEBAR: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("background-color", ColorBackground::Sidebar.value())
            }
        });

        static UNDERLINE_PRIMARY: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("background-color", ColorBackground::UnderlinePrimary.value())
            }
        });

        static UNDERLINE_SECONDARY: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("background-color", ColorBackground::UnderlineSecondary.value())
            }
        });

        static MODAL_CONTENT: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("background-color", ColorBackground::ModalContent.value())
            }
        });

        static BUTTON_RED: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("background-color", ColorBackground::ButtonRed.value())
            }
        });

        static BUTTON_RED_HOVER: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("background-color", ColorBackground::ButtonRedHover.value())
            }
        });

        match self {
            Self::ButtonPrimary => &*BUTTON_PRIMARY,
            Self::ButtonPrimaryHover => &*BUTTON_PRIMARY_HOVER,
            Self::ButtonDisabled => &*BUTTON_DISABLED,
            Self::SidebarSelected => &*SIDEBAR_SELECTED,
            Self::Sidebar => &*SIDEBAR,
            Self::UnderlinePrimary => &*UNDERLINE_PRIMARY,
            Self::UnderlineSecondary => &*UNDERLINE_SECONDARY,
            Self::ModalContent => &*MODAL_CONTENT,
            Self::ButtonRed => &*BUTTON_RED,
            Self::ButtonRedHover => &*BUTTON_RED_HOVER,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorText {
    ButtonPrimary,
    Header,
    Byline,
    Paragraph,
    Input,
    InputPlaceholder,
    Error,
    Success,
    Label,
    LabelHover,
}

impl ColorText {
    pub fn value(self) -> &'static str {
        match self {
            Self::ButtonPrimary => ColorRaw::Whiteish.value(),
            Self::Header => ColorRaw::Darkest.value(),
            Self::Byline => ColorRaw::MidGrey.value(),
            Self::Paragraph => ColorRaw::Darkish.value(),
            Self::Label => ColorRaw::Darkish.value(),
            Self::LabelHover => ColorRaw::Accent.value(),
            Self::Input => ColorRaw::Darkish.value(),
            Self::InputPlaceholder => ColorRaw::MidGrey.value(),
            Self::Error => ColorRaw::Red.value(),
            Self::Success => ColorRaw::Green.value(),
        }
    }

    pub fn class(self) -> &'static str {
        static BUTTON_PRIMARY: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("color", ColorText::ButtonPrimary.value())
            }
        });

        static HEADER: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("color", ColorText::Header.value())
            }
        });

        static BYLINE: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("color", ColorText::Byline.value())
            }
        });

        static PARAGRAPH: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("color", ColorText::Paragraph.value())
            }
        });

        static INPUT: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("color", ColorText::Input.value())
            }
        });

        static INPUT_PLACEHOLDER: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("color", ColorText::InputPlaceholder.value())
            }
        });

        static ERROR: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("color", ColorText::Error.value())
            }
        });

        static SUCCESS: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("color", ColorText::Success.value())
            }
        });

        static LABEL: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("color", ColorText::Label.value())
            }
        });

        static LABEL_HOVER: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("color", ColorText::LabelHover.value())
            }
        });

        match self {
            Self::ButtonPrimary => &*BUTTON_PRIMARY,
            Self::Header => &*HEADER,
            Self::Byline => &*BYLINE,
            Self::Paragraph => &*PARAGRAPH,
            Self::Input => &*INPUT,
            Self::InputPlaceholder => &*INPUT_PLACEHOLDER,
            Self::Error => &*ERROR,
            Self::Success => &*SUCCESS,
            Self::Label => &*LABEL,
            Self::LabelHover => &*LABEL_HOVER,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorLabel {
    Input,
}

impl ColorLabel {
    pub fn value(self) -> &'static str {
        match self {
            Self::Input => ColorRaw::Darkest.value(),
        }
    }

    pub fn class(self) -> &'static str {
        static INPUT: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("color", ColorLabel::Input.value())
            }
        });

        match self {
            Self::Input => &*INPUT,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorBorder {
    Input,
    Warning,
    Error,
    Focus,
    UnderlinePrimary,
    UnderlineSecondary,
}

impl ColorBorder {
    pub fn value(self) -> &'static str {
        match self {
            Self::Input => ColorRaw::MidGrey.value(),
            Self::Warning => ColorRaw::Orange.value(),
            Self::Error => ColorRaw::Red.value(),
            Self::Focus => ColorRaw::Focus.value(),
            Self::UnderlinePrimary => ColorRaw::Accent.value(),
            Self::UnderlineSecondary => ColorRaw::MidGrey.value(),
        }
    }

    pub fn class(self) -> &'static str {
        static INPUT: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("border-color", ColorBorder::Input.value())
            }
        });

        static WARNING: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("border-color", ColorBorder::Warning.value())
            }
        });

        static ERROR: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("border-color", ColorBorder::Error.value())
            }
        });

        static FOCUS: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("border-color", ColorBorder::Focus.value())
            }
        });

        static UNDERLINE_PRIMARY: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("border-color", ColorBorder::UnderlinePrimary.value())
            }
        });

        static UNDERLINE_SECONDARY: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("border-color", ColorBorder::UnderlineSecondary.value())
            }
        });

        match self {
            Self::Input => &*INPUT,
            Self::Warning => &*WARNING,
            Self::Error => &*ERROR,
            Self::Focus => &*FOCUS,
            Self::UnderlinePrimary => &*UNDERLINE_PRIMARY,
            Self::UnderlineSecondary => &*UNDERLINE_SECONDARY,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorLogo {
    Primary,
}

impl ColorLogo {
    pub fn value(self) -> &'static str {
        match self {
            Self::Primary => ColorRaw::Accent.value(),
        }
    }
    pub fn class(self) -> &'static str {
        static PRIMARY: LazyLock<String> = LazyLock::new(|| {
            class! {
              .style("color", ColorLogo::Primary.value())
            }
        });

        match self {
            Self::Primary => &*PRIMARY,
        }
    }
}
