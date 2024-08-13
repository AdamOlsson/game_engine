
pub struct Rectangle {}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectangleInstance {
    pub color: [f32; 3],
    pub top_left: [f32; 3],
    pub bot_right: [f32; 3],
}
