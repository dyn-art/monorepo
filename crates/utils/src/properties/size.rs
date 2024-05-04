use crate::units::{abs::Abs, ratio::Ratio};
use glam::Vec2;

const MIN_SIZE: f32 = 0.0;

/// An absolute size in 2D with a width and a height.
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub struct Size {
    pub width: Abs,
    pub height: Abs,
}

impl Size {
    /// The zero value.
    pub const fn zero() -> Self {
        Self {
            width: Abs::zero(),
            height: Abs::zero(),
        }
    }

    /// Create a new instance from the two components.
    pub fn new(width: Abs, height: Abs) -> Self {
        Self {
            width: width.max(Abs::pt(MIN_SIZE)),
            height: height.max(Abs::pt(MIN_SIZE)),
        }
    }

    pub fn from_vec2(vec2: Vec2) -> Self {
        Self {
            width: Abs::pt(vec2.x).min(Abs::pt(MIN_SIZE)),
            height: Abs::pt(vec2.y).min(Abs::pt(MIN_SIZE)),
        }
    }

    pub fn width(&self) -> f32 {
        self.width.to_pt()
    }

    pub fn height(&self) -> f32 {
        self.height.to_pt()
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }

    pub fn to_tuple(&self) -> (f32, f32) {
        (self.width(), self.height())
    }

    /// Whether the other size fits into this one (smaller width and height).
    pub fn fits(self, other: Self) -> bool {
        self.width.fits(other.width) && self.height.fits(other.height)
    }

    /// Converts to a ratio of width to height.
    pub fn aspect_ratio(&self) -> Ratio {
        Ratio::new(self.width / self.height)
    }
}

#[cfg(feature = "serde_support")]
const _: () = {
    use serde::{
        de::{self, SeqAccess, Visitor},
        ser::SerializeTupleStruct,
        Deserialize, Deserializer, Serialize, Serializer,
    };
    use std::fmt;

    // https://serde.rs/impl-serialize.html#serializing-a-tuple
    impl Serialize for Size {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_tuple_struct("Size", 2)?;
            state.serialize_field(&self.width)?;
            state.serialize_field(&self.height)?;
            state.end()
        }
    }

    // https://serde.rs/deserialize-struct.html
    impl<'de> Deserialize<'de> for Size {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct SizeVisitor;

            impl<'de> Visitor<'de> for SizeVisitor {
                type Value = Size;

                fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    formatter.write_str("struct Size")
                }

                fn visit_seq<V>(self, mut seq: V) -> Result<Size, V::Error>
                where
                    V: SeqAccess<'de>,
                {
                    let x = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                    let y = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    Ok(Size::new(x, y))
                }
            }

            deserializer.deserialize_tuple_struct(stringify!(Size), 2, SizeVisitor)
        }
    }

    #[derive(specta::Type)]
    #[specta(rename = "Size", remote = Size, crate = specta, export = false)]
    #[allow(dead_code)]
    struct SizeDef([Abs; 2]);
};
