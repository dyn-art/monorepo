pub mod bundles;
pub mod events;
pub mod mixins;
pub mod nodes;
pub mod shared;

pub mod prelude {
    pub use crate::bundles::*;
    pub use crate::events::*;
    pub use crate::mixins::*;
    pub use crate::nodes::*;
    pub use crate::shared::*;
}

#[cfg(test)]
mod tests {
    use specta::{
        export,
        ts::{BigIntExportBehavior, ExportConfig},
    };

    use super::*;

    #[test]
    fn specta_works() {
        export::ts_with_cfg(
            "./bindings.ts",
            "".into(),
            &ExportConfig::default().bigint(BigIntExportBehavior::Number),
        )
        .unwrap();
    }
}
