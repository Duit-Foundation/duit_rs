use std::collections::HashMap;

use crate::widgets::{ComponentDescription, DuitWidget, json::DuitJsonWidget};

pub struct DuitUIBuilder<'a> {
    root: Option<DuitWidget<'a>>,
    embedded: Vec<ComponentDescription<'a>>, //TODO: implement later
    multiview_widgets: HashMap<String, DuitJsonWidget<'a>>,
}

impl<'a> DuitUIBuilder<'a> {
    pub fn new() -> Self {
        Self {
            root: None,
            embedded: vec![],
            multiview_widgets: HashMap::new(),
        }
    }

    pub fn set_root(mut self, root: DuitWidget<'a>) -> Self {
        self.root = Some(root);
        self
    }

    pub fn add_widget(mut self, tag: String, widget: DuitWidget<'a>) -> Self {
        self.multiview_widgets.insert(tag, widget.into());
        self
    }

    pub fn add_component(mut self, component: ComponentDescription<'a>) -> Self {
        self.embedded.push(component);
        self
    }

    #[inline]
    pub fn build_string_pretty(self) -> anyhow::Result<String> {
        if let Some(root) = self.root {
            let root_json: DuitJsonWidget = root.into();
            let content = serde_json::json!({
                "root": root_json,
                "embedded": self.embedded,
            });
            let json = serde_json::to_string_pretty(&content)?;
            Ok(json)
        } else {
            let content = serde_json::json!({
                "widgets": self.multiview_widgets,
                "embedded": self.embedded,
            });
            let json = serde_json::to_string_pretty(&content)?;
            Ok(json)
        }
    }

    #[inline]
    pub fn build_string(self) -> anyhow::Result<String> {
        if let Some(root) = self.root {
            let root_json: DuitJsonWidget = root.into();
            let content = serde_json::json!({
                "root": root_json,
                "embedded": self.embedded,
            });
            let json = serde_json::to_string(&content)?;
            Ok(json)
        } else {
            let content = serde_json::json!({
                "widgets": self.multiview_widgets,
                "embedded": self.embedded,
            });
            let json = serde_json::to_string(&content)?;
            Ok(json)
        }
    }

    #[inline]
    pub fn build_vec(self) -> anyhow::Result<Vec<u8>> {
        if let Some(root) = self.root {
            let root_json: DuitJsonWidget = root.into();
            let content = serde_json::json!({
                "root": root_json,
                "embedded": self.embedded,
            });
            let json = serde_json::to_vec(&content)?;
            Ok(json)
        } else {
            let content = serde_json::json!({
                "widgets": self.multiview_widgets,
                "embedded": self.embedded,
            });
            let json = serde_json::to_vec(&content)?;
            Ok(json)
        }
    }
}
