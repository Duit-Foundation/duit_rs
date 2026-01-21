use std::borrow::Cow;

/// Хелпер-trait для конвертации в Cow
pub trait IntoCow<'a, T: Clone> {
    fn into_cow(self) -> Cow<'a, T>;
}