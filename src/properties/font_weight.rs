use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum FontWeight {
    W100,
    W200,
    W300,
    W400,
    W500,
    W600,
    W700,
    W800,
    W900,
}

impl Serialize for FontWeight {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(match self {
            FontWeight::W100 => 100,
            FontWeight::W200 => 200,
            FontWeight::W300 => 300,
            FontWeight::W400 => 400,
            FontWeight::W500 => 500,
            FontWeight::W600 => 600,
            FontWeight::W700 => 700,
            FontWeight::W800 => 800,
            FontWeight::W900 => 900,
        })
    }
}
