use value_ref_holder_macro::with_refs;
use into_cow_macro::into_cow;
use serde::Serialize;

use crate::properties::{FontWeight, TextDirection};

#[with_refs]
#[into_cow]
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextAttributes {
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_direction: Option<TextDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
}

impl TextAttributes {
    /// Устанавливает значение поля data и возвращает владеющий объект
    pub fn data(mut self, value: String) -> Self {
        self.data = value;
        self
    }

    /// Устанавливает значение поля text_direction и возвращает владеющий объект
    pub fn text_direction(mut self, value: TextDirection) -> Self {
        self.text_direction = Some(value);
        self
    }

    /// Устанавливает значение поля font_weight и возвращает владеющий объект
    pub fn font_weight(mut self, value: FontWeight) -> Self {
        self.font_weight = Some(value);
        self
    }
}