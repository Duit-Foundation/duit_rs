use duit_macro::{into_cow, with_refs};
use serde::Serialize;

use crate::properties::{FontWeight, TextDirection};

#[with_refs]
#[into_cow]
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextAttributes<'a> {
    data: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_direction: Option<TextDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font_weight: Option<FontWeight>,
}

impl<'a> TextAttributes<'a> {
    pub const fn c_new() -> Self {
        Self {
            data: "",
            text_direction: None,
            font_weight: None,
            refs: vec![],
        }
    }
    /// Устанавливает значение поля data и возвращает владеющий объект
    pub fn data(mut self, value: &'a str) -> Self {
        self.data = value;
        self
    }

    pub const fn c_data(mut self, value: &'a str) -> Self {
        self.data = value;
        self
    }

    /// Устанавливает значение поля text_direction и возвращает владеющий объект
    pub fn text_direction(mut self, value: TextDirection) -> Self {
        self.text_direction = Some(value);
        self
    }

    pub const fn c_text_direction(mut self, value: TextDirection) -> Self {
        self.text_direction = Some(value);
        self
    }

    pub fn font_weight(mut self, value: FontWeight) -> Self {
        self.font_weight = Some(value);
        self
    }

    pub const fn c_font_weight(mut self, value: FontWeight) -> Self {
        self.font_weight = Some(value);
        self
    }
}
