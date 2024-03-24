use crate::units::angle::Angle;

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct CornerRadii(Angle, Angle, Angle, Angle);

impl CornerRadii {
    pub fn new(top_left: Angle, top_right: Angle, bottom_right: Angle, bottom_left: Angle) -> Self {
        Self(top_left, top_right, bottom_right, bottom_left)
    }

    pub fn get_tl(&self) -> Angle {
        self.0
    }

    pub fn set_tl(&mut self, top_left: Angle) {
        self.0 = top_left
    }

    pub fn get_tr(&self) -> Angle {
        self.1
    }

    pub fn set_tr(&mut self, top_right: Angle) {
        self.1 = top_right
    }

    pub fn get_br(&self) -> Angle {
        self.2
    }

    pub fn set_br(&mut self, bottom_right: Angle) {
        self.2 = bottom_right
    }

    pub fn get_bl(&self) -> Angle {
        self.3
    }

    pub fn set_bl(&mut self, bottom_left: Angle) {
        self.3 = bottom_left
    }
}
