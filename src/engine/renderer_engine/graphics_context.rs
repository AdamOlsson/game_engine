use wgpu::{util::DeviceExt, Adapter, Buffer, Device, Instance, Queue};
use winit::window::{Window, WindowId};

pub struct GraphicsContext<'a> {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'a>,
    pub config: wgpu::SurfaceConfiguration,
    pub window_id: WindowId,
}

impl<'a> GraphicsContext<'a> {
    pub fn new(window: Window) -> Self {
        let size = window.inner_size();
        let window_id = window.id();
        let gpu_instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = gpu_instance.create_surface(window).unwrap();

        //let adapter = gpu_instance.request_adapter(&wgpu::RequestAdapterOptions {
        //    power_preference: wgpu::PowerPreference::default(),
        //    compatible_surface: Some(&surface),
        //    force_fallback_adapter: false,
        //}).await.unwrap();

        let adapter = pollster::block_on(Self::request_adapter(&gpu_instance, &surface)).unwrap();
        let (device, queue) = pollster::block_on(Self::request_device(&adapter));
        //let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        //    required_features: wgpu::Features::empty(),
        //    required_limits: wgpu::Limits::default(),
        //    label: Some("Device"),
        //}, None).await.unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        Self { device, queue, surface, config, window_id }
    }

    async fn request_adapter(gpu_instance: &Instance, surface: &wgpu::Surface<'a>) -> Option<Adapter> {
        gpu_instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await
    }

    async fn request_device(adapter: &Adapter) -> (Device, Queue) {
        adapter.request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Device"),
        }, None).await.unwrap()
    }

    pub fn create_buffer(&self, label: &str, size: u32, usage: wgpu::BufferUsages, 
        mapped_at_creation: bool
    ) -> Buffer {
        self.device.create_buffer(
            &wgpu::BufferDescriptor{
                label: Some(label), size: size as u64, usage, mapped_at_creation
        })
    }

    pub fn create_buffer_init(&mut self, label: &str, contents: &[u8], usage: wgpu::BufferUsages) -> wgpu::Buffer {
        self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents,
                usage,
            }
        )
    }
}
