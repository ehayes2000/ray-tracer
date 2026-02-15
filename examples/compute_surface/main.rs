use std::sync::Arc;
use wgpu::{Extent3d, TextureUsages};
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct VertexBufferData {
    px: [f32; 2],
}

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    window: Arc<Window>,
    render_pipeline: wgpu::RenderPipeline,
    compute_pipeline: wgpu::ComputePipeline,
    config: wgpu::SurfaceConfiguration,
    surface_is_configured: bool,
    render_bind_group: wgpu::BindGroup,
    compute_bind_group: wgpu::BindGroup,
    texture: wgpu::Texture,
}

impl State {
    fn create_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture"),
            size: Extent3d {
                depth_or_array_layers: 1,
                height: height,
                width: width,
            },
            dimension: wgpu::TextureDimension::D2,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
            mip_level_count: 1,
            sample_count: 1,
            format: wgpu::TextureFormat::Rgba8Unorm,
            view_formats: &[],
        })
    }

    fn create_compute_bind_group(
        device: &wgpu::Device,
        texture: &wgpu::Texture,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let compute_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    visibility: wgpu::ShaderStages::COMPUTE,
                }],
                label: Some("compute_bind_group_layout"),
            });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("compute_bind_groupd"),
            layout: &compute_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            }],
        });
        (compute_bind_group_layout, compute_bind_group)
    }

    fn create_render_bind_group(
        device: &wgpu::Device,
        texture: &wgpu::Texture,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let render_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        count: None,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        visibility: wgpu::ShaderStages::FRAGMENT,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        count: None,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        visibility: wgpu::ShaderStages::FRAGMENT,
                    },
                ],
                label: Some("render_bind_group_layout"),
            });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("render_bind_group"),
            layout: &render_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });
        (render_bind_group_layout, render_bind_group)
    }

    async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                trace: wgpu::Trace::Off,
                ..Default::default()
            })
            .await
            .unwrap();
        let capabilities = surface.get_capabilities(&adapter);
        let presentation_format = capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .expect("SRGB surface");

        let size = window.inner_size();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: presentation_format,
            width: size.width,
            height: size.height,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        let render_shaders = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let compute_shaders = device.create_shader_module(wgpu::include_wgsl!("compute.wgsl"));

        let texture = Self::create_texture(&device, 256, 256);

        let (compute_bind_group_layout, compute_bind_group) =
            Self::create_compute_bind_group(&device, &texture);

        let compute_pipeline_layouer =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("compute layout"),
                bind_group_layouts: &[&compute_bind_group_layout],
                push_constant_ranges: &[],
            });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("compute pipe"),
            layout: Some(&compute_pipeline_layouer),
            module: &compute_shaders,
            entry_point: Some("cs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

        let (render_bind_group_layout, render_bind_group) =
            Self::create_render_bind_group(&device, &texture);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render layout"),
                bind_group_layouts: &[&render_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            fragment: Some(wgpu::FragmentState {
                entry_point: Some("fs_main"),
                module: &render_shaders,
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            vertex: wgpu::VertexState {
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                entry_point: Some("vs_main"),
                module: &render_shaders,
            },
            label: Some("render pipeline"),
            layout: Some(&render_pipeline_layout),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: None,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
        });
        Self {
            compute_pipeline,
            render_pipeline,
            config,
            device,
            queue,
            surface,
            window,
            render_bind_group,
            compute_bind_group,
            texture,
            surface_is_configured: false,
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.surface_is_configured = true;
            self.texture = Self::create_texture(&self.device, width, height);
            self.compute_bind_group =
                Self::create_compute_bind_group(&self.device, &self.texture).1;
            self.render_bind_group = Self::create_render_bind_group(&self.device, &self.texture).1;
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();
        if !self.surface_is_configured {
            return Ok(());
        }
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute_pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, Some(&self.compute_bind_group), &[]);
            let dims = self.texture.size();
            let workgroup_size = 8;
            compute_pass.dispatch_workgroups(
                (dims.width + workgroup_size - 1) / workgroup_size,
                (dims.height + workgroup_size - 1) / workgroup_size,
                1,
            );
        }

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_bind_group(0, Some(&self.render_bind_group), &[]);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }
        self.queue.submit(Some(encoder.finish()));
        output.present();
        Ok(())
    }
}

struct App {
    pub state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        self.state = Some(pollster::block_on(State::new(window)));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = &mut self.state else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        eprintln!("unable to render {}", e);
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut app = App { state: None };
    event_loop.run_app(&mut app).unwrap();
}
