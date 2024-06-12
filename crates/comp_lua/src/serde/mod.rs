/**
 * -----------------------------------------------------------------------------
 * This file includes code derived from the project keyren/piccolo by @kyren.
 * Project Repository: https://github.com/kyren/piccolo/blob/master/util/src/serde/mod.rs
 *
 * Date of Import: 31 May 2024
 * -----------------------------------------------------------------------------
 * The code included in this file is licensed under the MIT License,
 * as per the original project by @kyren.
 * For the license text, see: https://github.com/kyren/piccolo/blob/master/LICENSE-MIT
 * -----------------------------------------------------------------------------
 */
pub mod de;
pub mod markers;
pub mod ser;

use piccolo::Lua;

pub use self::{
    de::from_value,
    ser::{to_value, to_value_with, Options as SerOptions},
};

pub trait LuaSerdeExt {
    fn load_serde(&mut self);
}

impl LuaSerdeExt for Lua {
    fn load_serde(&mut self) {
        self.enter(|ctx| markers::set_globals(ctx));
    }
}
