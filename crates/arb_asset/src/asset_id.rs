use fontdb::ID;
use tinyvec::TinyVec;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Debug)]
pub enum AssetId {
    Font(FontId),
    Image(ImageId),
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Debug, Default)]
pub struct FontId(pub TinyVec<[ID; 8]>);

/// A unique per database ID.
///
/// Since `Database` is not global/unique, we cannot guarantee that a specific ID
/// is actually from the same db instance. This is up to the caller.
///
/// ID overflow will cause a panic, but it's highly unlikely that someone would
/// load more than 4 billion assets.
///
/// Because the internal representation of ID is private, The `Display` trait
/// implementation for this type only promise that unequal IDs will be displayed
/// as different strings, but does not make any guarantees about format or
/// content of the strings.
///
/// [`KeyData`]: https://docs.rs/slotmap/latest/slotmap/struct.KeyData.html
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, Debug, Default)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ImageId(pub(crate) InnerImageId);

slotmap::new_key_type! {
    /// Internal ID type.
    pub(crate) struct InnerImageId;
}

#[cfg(feature = "specta_support")]
const _: () = {
    impl specta::Type for ImageId {
        fn inline(_: &mut specta::TypeMap, _: specta::Generics) -> specta::DataType {
            SpectaDataKey {
                idx: specta::PrimitiveType::u32.into(),
                version: specta::PrimitiveType::u32.into(),
            }
            .into()
        }
    }

    #[derive(Clone, specta::DataTypeFrom)]
    struct SpectaDataKey {
        idx: specta::DataType,
        version: specta::DataType,
    }
};

impl core::fmt::Display for ImageId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", (self.0).0.as_ffi())
    }
}
