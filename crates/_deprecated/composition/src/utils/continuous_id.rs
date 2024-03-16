use std::fmt::Display;

use serde::Serialize;
use specta::Type;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, PartialOrd, Ord, Type)]
pub struct ContinuousId(usize);

impl ContinuousId {
    pub const ZERO: ContinuousId = ContinuousId(0);

    pub fn next_id(&mut self) -> Self {
        let old = self.0;
        self.0 += 1;
        Self(old)
    }
}

impl Into<usize> for ContinuousId {
    fn into(self) -> usize {
        self.0
    }
}

impl Display for ContinuousId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
