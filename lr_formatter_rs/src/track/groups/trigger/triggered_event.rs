use derive_builder::Builder;
use getset::Getters;

use crate::track::primitives::{Event, Trigger};

#[derive(Getters, Builder)]
#[getset(get = "pub")]
pub struct TriggeredEvent<E: Event, T: Trigger> {
    event: E,
    trigger: T,
}
