use encase::{ShaderType, StorageBuffer};
use std::{iter, sync::Arc};
use wgpu::util::DeviceExt;
use wgpu::{FragmentState, VertexState};
use winit::event;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::types::{Material, RenderParameters, SceneBufferEntry, Sphere, V3, cross};
use crate::v3;

pub struct State {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub is_surface_configured: bool,
    pub window: Arc<Window>,
    pub pipeline: wgpu::RenderPipeline,
    pub fps: Fps,
    pub bind_group: wgpu::BindGroup,
    pub param_uniform: wgpu::Buffer,
    pub params: RenderParameters,
    pub move_dir: Option<Direction>,
}

fn scene() -> Vec<SceneBufferEntry> {
    let color = Material {
        kind: 0,
        color: v3!(0.7, 0.2, 0.2),
        refractive_index: 0.,
        roughness: 0.,
    };

    let ground = Material {
        kind: 0,
        color: v3!(0.3, 0.3, 0.3),
        refractive_index: 0.,
        roughness: 0.,
    };

    let glass = Material {
        kind: 1,
        color: v3!(1.0, 0.8, 0.8),
        refractive_index: 2.0,
        roughness: 0.0,
    };

    let metal = Material {
        kind: 1,
        color: v3!(1.0, 1.0, 1.0),
        refractive_index: 0.0,
        roughness: 0.2,
    };

    vec![
        Sphere {
            radius: 1.5,
            location: v3!(0., 1.5, 0.),
            material: color.clone(),
        },
        Sphere {
            radius: 1.0,
            location: v3!(0., 1.0, 2.5),
            material: metal.clone(),
        },
        Sphere {
            radius: 1.0,
            location: v3!(0, 1.0, -2.5),
            material: metal,
        },
        Sphere {
            radius: 10000.0,
            location: v3!(0, -10000, 0),
            material: ground,
        },
        Sphere {
            radius: 0.5,
            location: v3!(1.5, 0.5, -1.0),
            material: glass,
        },
    ]
    .into_iter()
    .map(|s| SceneBufferEntry { sphere: s })
    .collect()
}

#[derive(Clone, Debug)]
pub struct Fps {
    pub frames: u64,
    pub last: std::time::Instant,
    pub sum: u128,
}

impl Fps {
    pub fn new() -> Self {
        Fps {
            frames: 0,
            last: std::time::Instant::now(),
            sum: 0,
        }
    }

    pub fn update(&mut self) {
        self.frames += 1;
        let uspf = self.last.elapsed().as_micros();
        self.last = std::time::Instant::now();
        self.sum += uspf;
        if self.frames % 60 == 0 {
            let avg_uspf = self.sum as f64 / 60.0;
            let avg_fps = 1E6 / avg_uspf;
            println!("{:.3}", avg_fps);
            self.sum = 0;
        }
    }
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        // initial size (pog)
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        // compile shader
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // set layout for scene and storage buffers
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                // scene buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        has_dynamic_offset: false,
                        min_binding_size: Some(SceneBufferEntry::min_size()),
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                    },
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                },
                // render param buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        has_dynamic_offset: false,
                        min_binding_size: Some(RenderParameters::min_size()),
                        ty: wgpu::BufferBindingType::Uniform,
                    },
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                },
            ],
            label: Some("bind_group_layout_0_scene_buf"),
        });

        // create content for scene buffer
        let mut scene_buf = StorageBuffer::new(Vec::<u8>::new());
        scene_buf
            .write(&scene())
            .expect("failed to write contents to scene buffer");

        // ccreate and initialize scene buffer
        let scene_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("scene_buffer"),
            contents: &scene_buf.into_inner(),
            usage: wgpu::BufferUsages::STORAGE,
        });
        // create_content for param buffer
        let mut param_uniform_storage_buffer = StorageBuffer::new(Vec::<u8>::new());
        param_uniform_storage_buffer
            .write(&RenderParameters::default())
            .expect("failed to write render parameter content");

        let param_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("param_buffer"),
            contents: &param_uniform_storage_buffer.into_inner(),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: scene_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: param_uniform.as_entire_binding(),
                },
            ],
            label: Some("bind_group_0"),
            layout: &layout,
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            fragment: Some(FragmentState {
                entry_point: Some("fs_main"),
                module: &shader,
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            vertex: VertexState {
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                entry_point: Some("vs_main"),
                module: &shader,
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

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            pipeline,
            fps: Fps::new(),
            bind_group,
            param_uniform,
            params: RenderParameters::default(),
            move_dir: None,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
            self.params.img_h = height;
            self.params.img_w = width;
        }
    }

    pub fn update(&mut self) {
        self.fps.update();
        self.move_dir.map(|dir| move_camera(&mut self.params, dir));
        let mut data = StorageBuffer::new(Vec::new());
        data.write(&self.params).expect("good ok yes");
        self.queue
            .write_buffer(&self.param_uniform, 0, &data.into_inner());
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

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
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (is_pressed, Direction::try_from(code).ok()) {
            (true, Some(dir)) => self.move_dir = Some(dir),
            (false, Some(_)) => self.move_dir = None,
            _ => {}
        }
        if code == KeyCode::Escape {
            event_loop.exit();
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Forward,
    Backward,
}

impl TryFrom<KeyCode> for Direction {
    type Error = ();
    fn try_from(code: KeyCode) -> Result<Self, Self::Error> {
        match code {
            KeyCode::KeyA | KeyCode::ArrowLeft => Ok(Self::Left),
            KeyCode::KeyD | KeyCode::ArrowRight => Ok(Self::Right),
            KeyCode::KeyW | KeyCode::ArrowUp => Ok(Self::Forward),
            KeyCode::KeyS | KeyCode::ArrowDown => Ok(Self::Backward),
            KeyCode::Space => Ok(Self::Up),
            KeyCode::KeyC | KeyCode::ControlLeft => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

// this obviously needs to be normalized for time but idc atm
fn move_camera(params: &mut RenderParameters, direction: Direction) {
    let epsilon = 0.5;
    let look_vector = params.look_from - params.look_at;
    let fb = v3!(look_vector.0[0], 0, look_vector.0[2]).normalize();
    // left right is orthogonal to where looking and parallel to ground
    let lr = cross(v3!(0, 1, 0), fb);
    // forward backwards  is orthogonal to left right and and parallel to ground
    let translate = match direction {
        Direction::Backward => fb * epsilon,
        Direction::Forward => fb * epsilon * -1.0,
        Direction::Up => v3!(0, 1, 0) * epsilon,
        Direction::Down => v3!(0, -1, 0) * epsilon,
        Direction::Left => lr * epsilon * -1.0,
        Direction::Right => lr * epsilon,
    };
    params.look_from = params.look_from + translate;
    params.look_at = params.look_at + translate;
}
