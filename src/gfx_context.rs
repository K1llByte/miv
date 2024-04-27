use wgpu::{
    Backends, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, LoadOp, Operations, PowerPreference, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, Surface,
    SurfaceConfiguration, SurfaceError, TextureViewDescriptor,
};
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct Renderer<'win> {
    // Graphics context //
    //
    pub surface: Surface<'win>,
    pub device: Device,
    pub queue: Queue,
    // Surface config is stored so that we
    // may reconfigure surface on resize
    config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
}

impl<'win> Renderer<'win> {
    pub async fn new(window: &'win Window) -> Self {
        // Window size
        let size = window.inner_size();

        // TODO: Enable macos support with the vulkan portability feature
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::VULKAN,
            ..Default::default()
        });

        // Create surface from window and instance
        let surface = instance.create_surface(window).unwrap();

        // Request a device from an adapter (equivalent to a physical device)
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        println!("{:?}", adapter.get_info());
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        // Use default surface config, if color looks funny, try explicitly
        // selecting a surface with sRGB format
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        // Only resize surface when it makes sense
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        // Create command buffer
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        // Submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
