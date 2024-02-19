use std::fmt::Display;

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
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
