use serde::{Serialize};

use crate::{attributes, widgets::DuitWidget};

const TEXT_TYPE_NAME: &str = "Text";
const CONTAINER_TYPE_NAME: &str = "Container";
const ROW_TYPE_NAME: &str = "Row";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetJson {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<&'static str>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<attributes::TextAttributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child: Option<Box<WidgetJson>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<WidgetJson>>,
}

impl<'a> From<DuitWidget<'a>> for WidgetJson {
    fn from(w: DuitWidget<'a>) -> Self {
        match w {
            DuitWidget::_Text {
                attributes,
                id,
                controlled,
            } => Self {
                type_name: Some(TEXT_TYPE_NAME),
                attributes: Some(attributes.into_owned()),
                id: Some(id),
                controlled: Some(controlled),
                child: None,
                children: None,
            },
            DuitWidget::_Container {
                id,
                controlled,
                child,
            } => Self {
                type_name: Some(CONTAINER_TYPE_NAME),
                attributes: None,
                id: Some(id),
                controlled: Some(controlled),
                child: Some(Box::new((*child).into())),
                children: None,
            },
            DuitWidget::_Row {
                id,
                controlled,
                children,
            } => Self {
                type_name: Some(ROW_TYPE_NAME),
                attributes: None,
                id: Some(id),
                controlled: Some(controlled),
                child: None,
                children: Some(children.into_iter().map(|child| (*child).into()).collect()),
            },
        }
    }
}