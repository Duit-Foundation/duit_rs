use serde::Serialize;

use crate::{attributes, widgets::DuitWidget};

const TEXT_TYPE_NAME: &str = "Text";
const CONTAINER_TYPE_NAME: &str = "Container";
const ROW_TYPE_NAME: &str = "Row";

/// Enum для представления различных типов атрибутов виджетов
#[derive(Serialize, Clone)]
#[serde(untagged)]
pub enum WidgetAttributes<'a> {
    Text(attributes::TextAttributes<'a>),
    Container(attributes::ContainerAttributes<'a>),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DuitJsonWidget<'a> {
    #[serde(rename = "type")]
    pub type_name: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<WidgetAttributes<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child: Option<Box<DuitJsonWidget<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DuitJsonWidget<'a>>>,
}

impl<'a> From<DuitWidget<'a>> for DuitJsonWidget<'a> {
    fn from(w: DuitWidget<'a>) -> Self {
        match w {
            DuitWidget::Text {
                attributes,
                id,
                controlled,
            } => Self {
                type_name: TEXT_TYPE_NAME,
                attributes: Some(WidgetAttributes::Text(attributes.into_owned())),
                id: Some(id.to_string()),
                controlled: Some(controlled),
                child: None,
                children: None,
            },
            DuitWidget::Container {
                id,
                controlled,
                attributes,
                child,
            } => Self {
                type_name: CONTAINER_TYPE_NAME,
                attributes: Some(WidgetAttributes::Container(attributes.into_owned())),
                id: Some(id.to_string()),
                controlled: Some(controlled),
                child: Some(Box::new((*child).into())),
                children: None,
            },
            DuitWidget::Row {
                id,
                controlled,
                children,
            } => Self {
                type_name: ROW_TYPE_NAME,
                attributes: None,
                id: Some(id.to_string()),
                controlled: Some(controlled),
                child: None,
                children: Some(children.into_iter().map(|child| (*child).into()).collect()),
            },
        }
    }
}
