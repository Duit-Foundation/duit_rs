use crate::attributes;
use crate::core::IntoCow;
use std::borrow::Cow;

pub enum DuitWidget<'a> {
    _Text {
        id: String,
        controlled: bool,
        attributes: Cow<'a, attributes::TextAttributes>,
    },
    _Container {
        id: String,
        controlled: bool,
        child: Box<DuitWidget<'a>>,
    },
    _Row {
        id: String,
        controlled: bool,
        children: Vec<Box<DuitWidget<'a>>>,
    },
}

impl<'a> DuitWidget<'a> {
    pub fn text(
        id: String,
        controlled: bool,
        attributes: impl IntoCow<'a, attributes::TextAttributes>,
    ) -> Self {
        DuitWidget::_Text {
            id,
            controlled,
            attributes: attributes.into_cow(),
        }
    }

    pub fn container(id: String, controlled: bool, child: DuitWidget<'a>) -> Self {
        DuitWidget::_Container {
            id,
            controlled,
            child: Box::new(child),
        }
    }

    pub fn row(id: String, controlled: bool, children: Vec<DuitWidget<'a>>) -> Self {
        DuitWidget::_Row {
            id,
            controlled,
            children: children.into_iter().map(Box::new).collect(),
        }
    }
}