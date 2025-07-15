use serde::Deserializer;
use serde::de::{self, Unexpected};

pub(super) fn bool_from_any<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct BoolVisitor;

    impl de::Visitor<'_> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("true | false | 0 | 1")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match v {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(de::Error::invalid_value(Unexpected::Unsigned(v), &self)),
            }
        }
    }

    deserializer.deserialize_any(BoolVisitor)
}

pub(super) fn option_bool_from_any<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptionBoolVisitor;

    impl<'de> de::Visitor<'de> for OptionBoolVisitor {
        type Value = Option<bool>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("true | false | 0 | 1 | null | undefined")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            bool_from_any(deserializer).map(Some)
        }
    }

    deserializer.deserialize_option(OptionBoolVisitor)
}
