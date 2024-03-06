use fontdb::ID;
use tinyvec::TinyVec;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Debug)]
pub enum AssetId {
    Font(FontId),
    Any(AnyAssetId),
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd, Debug, Default)]
pub struct FontId(pub TinyVec<[ID; 8]>);

/// A unique per database ID.
///
/// Since `Database` is not global/unique, we cannot guarantee that a specific ID
/// is actually from the same db instance. This is up to the caller.
///
/// ID overflow will cause a panic, but it's highly unlikely that someone would
/// load more than 4 billion font faces.
///
/// Because the internal representation of ID is private, The `Display` trait
/// implementation for this type only promise that unequal IDs will be displayed
/// as different strings, but does not make any guarantees about format or
/// content of the strings.
///
/// [`KeyData`]: https://docs.rs/slotmap/latest/slotmap/struct.KeyData.html
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, Debug, Default)]
pub struct AnyAssetId(pub(crate) InnerAnyAssetId);

slotmap::new_key_type! {
    /// Internal ID type.
    pub(crate) struct InnerAnyAssetId;
}

impl core::fmt::Display for AnyAssetId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", (self.0).0.as_ffi())
    }
}
