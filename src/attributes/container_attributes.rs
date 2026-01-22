use duit_macro::{into_cow, with_refs};
use serde::Serialize;
use std::marker::PhantomData;

#[with_refs]
#[into_cow]
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerAttributes<'a> {
    pub lol: u8,
    #[serde(skip)]
    _phantom: PhantomData<&'a ()>,
}

impl<'a> ContainerAttributes<'a> {
    pub const fn c_new() -> Self {
        Self {
            lol: 0,
            _phantom: PhantomData {},
            refs: vec![],
        }
    }
}