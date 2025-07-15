use core::fmt;

use super::LRAJsonArrayLine;
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{Error as DeError, SeqAccess, Visitor},
    ser::Error,
};

impl Serialize for LRAJsonArrayLine {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Err(S::Error::custom(
            "LRAJsonArrayLine not intended to be serialized",
        ))
    }
}

impl<'de> Deserialize<'de> for LRAJsonArrayLine {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(LRAJsonArrayLineVisitor)
    }
}

struct LRAJsonArrayLineVisitor;

impl<'de> Visitor<'de> for LRAJsonArrayLineVisitor {
    type Value = LRAJsonArrayLine;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an array representing a line")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<LRAJsonArrayLine, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let line_type: u8 = seq
            .next_element()?
            .ok_or_else(|| DeError::invalid_length(0, &self))?;

        match line_type {
            0 => {
                let id: u32 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(1, &self))?;
                let x1: f64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(2, &self))?;
                let y1: f64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(3, &self))?;
                let x2: f64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(4, &self))?;
                let y2: f64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(5, &self))?;
                let extended: u8 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(6, &self))?;
                let flipped: bool = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(7, &self))?;
                Ok(LRAJsonArrayLine::Standard(
                    id, x1, y1, x2, y2, extended, flipped,
                ))
            }
            1 => {
                let id: u32 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(1, &self))?;
                let x1: f64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(2, &self))?;
                let y1: f64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(3, &self))?;
                let x2: f64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(4, &self))?;
                let y2: f64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(5, &self))?;
                let extended: u8 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(6, &self))?;
                let flipped: bool = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(7, &self))?;

                let mut multiplier: u32 = 1;
                if seq.next_element::<serde::de::IgnoredAny>()?.is_some() {
                    let _: serde::de::IgnoredAny = seq
                        .next_element()?
                        .ok_or_else(|| DeError::invalid_length(9, &self))?;
                    multiplier = seq
                        .next_element()?
                        .ok_or_else(|| DeError::invalid_length(10, &self))?;
                }

                Ok(LRAJsonArrayLine::Acceleration(
                    id,
                    x1,
                    y1,
                    x2,
                    y2,
                    extended,
                    flipped,
                    (),
                    (),
                    multiplier,
                ))
            }
            2 => {
                let id: u32 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(1, &self))?;
                let x1: f64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(2, &self))?;
                let y1: f64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(3, &self))?;
                let x2: f64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(4, &self))?;
                let y2: f64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(5, &self))?;
                Ok(LRAJsonArrayLine::Scenery(id, x1, y1, x2, y2))
            }
            _ => Err(DeError::custom(format!("Unknown line type: {}", line_type))),
        }
    }
}
