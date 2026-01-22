use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueRef<'a> {
    attribute_key: &'a str,
    object_key: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_value: Option<serde_json::Value>,
}

impl<'a> ValueRef<'a> {
    pub const fn c_new(
        attribute_key: &'a str,
        object_key: &'a str,
        default_value: Option<serde_json::Value>,
    ) -> Self {
        Self {
            attribute_key,
            object_key,
            default_value,
        }
    }

    pub fn new(
        attribute_key: &'a str,
        object_key: &'a str,
        default_value: Option<serde_json::Value>,
    ) -> Self {
        Self {
            attribute_key,
            object_key,
            default_value: default_value,
        }
    }
}
