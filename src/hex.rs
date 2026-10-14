//! Serde helpers that encode integers and byte strings as hexadecimal, so
//! addresses and prologue bytes are readable in the IR file.

/// Encodes an unsigned integer as a `0x`-prefixed lowercase hex string.
///
/// Used via `#[serde(with = "crate::hex::int")]`. On read, a leading `0x` or
/// `0X` is optional.
pub(crate) mod int {
    use core::fmt::{Display, LowerHex};
    use serde::{Deserialize, Deserializer, Serializer};

    pub(crate) fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: LowerHex,
        S: Serializer,
    {
        serializer.collect_str(&format_args!("0x{value:x}"))
    }

    pub(crate) fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: TryFrom<u64>,
        <T as TryFrom<u64>>::Error: Display,
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        let digits = text
            .strip_prefix("0x")
            .or_else(|| text.strip_prefix("0X"))
            .unwrap_or(&text);
        let raw = u64::from_str_radix(digits, 16).map_err(serde::de::Error::custom)?;
        T::try_from(raw).map_err(serde::de::Error::custom)
    }
}

/// Encodes a byte string as a lowercase hex string with no prefix (e.g.
/// `"48895c2408"`).
///
/// Used via `#[serde(with = "crate::hex::bytes")]` on a `Vec<u8>`.
pub(crate) mod bytes {
    use core::fmt::Write;
    use serde::{Deserialize, Deserializer, Serializer};

    pub(crate) fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        let mut text = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            // Writing to a `String` cannot fail.
            let _ = write!(text, "{byte:02x}");
        }
        serializer.serialize_str(&text)
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<u8>, D::Error> {
        let text = String::deserialize(deserializer)?;
        if !text.len().is_multiple_of(2) {
            return Err(serde::de::Error::custom(
                "hex byte string has an odd number of digits",
            ));
        }
        (0..text.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&text[i..i + 2], 16).map_err(serde::de::Error::custom))
            .collect()
    }
}
