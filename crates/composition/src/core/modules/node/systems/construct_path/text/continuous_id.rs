use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ContinuousId(usize);

impl ContinuousId {
    pub const ZERO: ContinuousId = ContinuousId(0);

    pub fn next_id(&mut self) -> Self {
        let old = self.0;
        self.0 += 1;
        Self(old)
    }
}

impl Display for ContinuousId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
