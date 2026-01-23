use serde::Serialize;

use crate::widgets::{DuitWidget, json::DuitJsonWidget};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDescription<'a> {
    tag: &'a str,
    layout_root: DuitJsonWidget<'a>,
}

impl<'a> ComponentDescription<'a> {
    pub fn new(tag: &'a str, widget: DuitWidget<'a>) -> Self {
        let layout_root: DuitJsonWidget = widget.into();
        Self { tag, layout_root }
    }
}
