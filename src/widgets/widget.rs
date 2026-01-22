use crate::attributes;
use std::borrow::Cow;


pub enum DuitWidget<'a> {
    Text {
        id: &'a str,
        controlled: bool,
        attributes: Cow<'a, attributes::TextAttributes<'a>>,
    },
    Container {
        id: &'a str,
        controlled: bool,
        attributes: Cow<'a, attributes::ContainerAttributes<'a>>,
        child: Box<DuitWidget<'a>>,
    },
    Row {
        id: &'a str,
        controlled: bool,
        children: Vec<Box<DuitWidget<'a>>>,
    },
}
