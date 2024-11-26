use crate::engine::renderer_engine::asset::sprite_sheet::SpriteCoordinate;
use cgmath::Vector3;

pub struct RenderBody {
    pub color: Vector3<f32>,
    pub sprite_coord: SpriteCoordinate,
}

pub struct RenderBodyBuilder {
    pub color: Option<Vector3<f32>>,
    pub sprite_coord: Option<SpriteCoordinate>,
}

impl std::default::Default for RenderBodyBuilder {
    fn default() -> Self {
        let color = None;
        let sprite_coord = None;
        Self {
            color,
            sprite_coord,
        }
    }
}

impl RenderBodyBuilder {
    pub fn color(mut self, color: [f32; 3]) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn sprite_coord(mut self, sprite_coord: SpriteCoordinate) -> Self {
        self.sprite_coord = Some(sprite_coord);
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
            color,
            sprite_coord,
        }
    }
}
