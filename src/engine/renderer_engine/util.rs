use super::graphics_context::GraphicsContext;
use crate::engine::renderer_engine::asset::sprite_sheet::SpriteSheet;

pub fn create_shader_module(device: &wgpu::Device, path: String) -> wgpu::ShaderModule{
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(&path.clone()),
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::from(path)),
    })
}

pub fn texture_bind_group_from_texture(
    device: &wgpu::Device, sampler: &wgpu::Sampler, texture: &wgpu::Texture
) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
    let layout = device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("Gray Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false },
                        count: None }
            ] }
    );
    let bind_group = device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            label: Some("Gray Bind Group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(sampler) 
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &texture.create_view(&wgpu::TextureViewDescriptor::default())) 
                }
            ] }
    );
    (bind_group, layout)
}

pub (crate) fn create_texture(
    ctx: &GraphicsContext, sprite_sheet: &SpriteSheet, label: Option<&str>,
) -> wgpu::Texture { 
    let dimensions = sprite_sheet.dimensions();
    let texture_size = wgpu::Extent3d {
        width: dimensions.0, height: dimensions.1, depth_or_array_layers: 1,
    };

    ctx.device.create_texture(
        &wgpu::TextureDescriptor {
            label, 
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage:  wgpu::TextureUsages::COPY_DST |
                wgpu::TextureUsages::RENDER_ATTACHMENT |
                wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[]
        })
}

pub (crate) fn write_texture(
    ctx: &GraphicsContext, texture: &wgpu::Texture, 
    data: &SpriteSheet,
) {
    let dimensions = data.dimensions();
    let texture_size = wgpu::Extent3d {
        width: dimensions.0, height: dimensions.1, depth_or_array_layers: 1,
    };    
    ctx.queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        }, 
        &data.sprite_buf, 
        wgpu::ImageDataLayout {
            offset: 0, 
            bytes_per_row: Some(4*dimensions.0),
            rows_per_image: Some(dimensions.1),
        },
        texture_size);
}

pub (crate) fn create_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Gray Sampler"), 
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge, 
        mag_filter: wgpu::FilterMode::Nearest, min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    })
}
