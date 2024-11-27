use super::RenderBodyShape;
use crate::engine::renderer_engine::asset::sprite_sheet::SpriteCoordinate;
use cgmath::Vector3;

pub struct RenderBody {
    pub shape: RenderBodyShape,
    pub color: Vector3<f32>,
    pub sprite_coord: SpriteCoordinate,
}

pub struct RenderBodyBuilder {
    pub shape: Option<RenderBodyShape>,
    pub color: Option<Vector3<f32>>,
    pub sprite_coord: Option<SpriteCoordinate>,
}

impl RenderBodyBuilder {
    pub fn new() -> Self {
        Self {
            shape: None,
            color: None,
            sprite_coord: None,
        }
    }

    pub fn color(mut self, color: [f32; 3]) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn sprite_coord(mut self, sprite_coord: SpriteCoordinate) -> Self {
        self.sprite_coord = Some(sprite_coord);
        self
    }

    pub fn shape(mut self, shape: RenderBodyShape) -> Self {
        self.shape = Some(shape);
        self
    }

    pub fn build(self) -> RenderBody {
        let color = if let Some(c) = self.color {
            c
        } else {
            [1.0, 0.0, 0.0].into()
        };

        let sprite_coord = if let Some(sc) = self.sprite_coord {
            sc
        } else {
            SpriteCoordinate::none()
        };
        RenderBody {
            shape: self.shape.expect("Expected RenderBody to have a shape"),
            color,
            sprite_coord,
        }
    }
}
