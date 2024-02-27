use bevy_ecs::component::Component;

use crate::common::Color;

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct CompPaint;

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct SolidCompPaint {
    pub color: Color,
}
