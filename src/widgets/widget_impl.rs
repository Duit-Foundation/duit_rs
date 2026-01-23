use std::borrow::Cow;

use crate::attributes;
use crate::{core::IntoCow, widgets::DuitWidget};

impl<'a> DuitWidget<'a> {
    pub fn text(
        id: &'a str,
        controlled: bool,
        attributes: impl IntoCow<'a, attributes::TextAttributes<'a>>,
    ) -> Self {
        DuitWidget::Text {
            id,
            controlled,
            attributes: attributes.into_cow(),
        }
    }

    pub fn container(
        id: &'a str,
        controlled: bool,
        attributes: attributes::ContainerAttributes<'a>,
        child: DuitWidget<'a>,
    ) -> Self {
        DuitWidget::Container {
            id,
            controlled,
            attributes: Cow::Owned(attributes),
            child: Box::new(child),
        }
    }

    pub fn row(id: &'a str, controlled: bool, children: Vec<DuitWidget<'a>>) -> Self {
        DuitWidget::Row {
            id,
            controlled,
            children: children.into_iter().map(Box::new).collect(),
        }
    }
}
