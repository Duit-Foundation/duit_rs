use serde::Serialize;

#[derive(Debug, Clone)]
pub enum TextDirection {
    Ltr,
    Rtl,
}

impl Serialize for TextDirection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {   
        serializer.serialize_str(match self {
            TextDirection::Ltr => "ltr",
            TextDirection::Rtl => "rtl",
        })
    }
}