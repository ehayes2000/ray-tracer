use super::bvh::{BvhShaderNode, build_shader_bvh};
use super::mesh::Mesh;
use super::scene::Scene;
use encase::{ShaderType, StorageBuffer};
use std::{iter, sync::Arc};
use wgpu::{FragmentState, TextureUsages, VertexState};
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use super::types::{Material, RenderParameters, SceneBufferEntry, Sphere};
use crate::gpu::resources::Resources;
use crate::gpu::types::{MaterialKind, Triangle};
use crate::math::cross;
use crate::v3;

pub struct State {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub compute_pipeline: wgpu::ComputePipeline,
    pub render_pipeline: wgpu::RenderPipeline,
    pub is_surface_configured: bool,
    pub window: Arc<Window>,
    pub resources: Resources,
}

fn sphere_scene() -> Vec<SceneBufferEntry> {
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
            location: v3!(0., 1.0, -2.5),
            material: metal,
        },
        Sphere {
            radius: 10000.0,
            location: v3!(0., -10000., 0.),
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

impl Default for Fps {
    fn default() -> Self {
        Self::new()
    }
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
        if self.frames.is_multiple_of(60) {
            let avg_uspf = self.sum as f32 / 60.0;
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
        let scene = mesh_scene();
        let bvh = build_shader_bvh(scene.triangles());

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
            .expect("SRGB Surface");

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

        let render_shader = device.create_shader_module(wgpu::include_wgsl!("render.wgsl"));
        let compute_shader = device.create_shader_module(wgpu::include_wgsl!("compute.wgsl"));

        let resources = Resources::create(
            &device,
            size.width,
            size.height,
            bvh,
            scene.materials,
            RenderParameters::default(),
        );

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&resources.render_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            fragment: Some(FragmentState {
                entry_point: Some("fs_main"),
                module: &render_shader,
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
                module: &render_shader,
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

        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("compute layout"),
                bind_group_layouts: &[&resources.compute_bind_group_layout],
                push_constant_ranges: &[],
            });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("compute pipe"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: Some("main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

        Ok(Self {
            resources,
            surface,
            device,
            queue,
            config,
            compute_pipeline,
            render_pipeline,
            is_surface_configured: false,
            window,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            println!("configure surface {:?}", self.config);
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
            self.resources.resize(&self.device, width, height);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

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
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute_pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, Some(&self.resources.compute_bind_group), &[]);

            let dims = self.resources.texture.size();
            println!("texture size {:?}", dims);
            let workgroup_size = 12;
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
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.resources.render_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, _is_pressed: bool) {
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
    let fb = v3!(look_vector.0, 0.0, look_vector.2).normalize();
    // left right is orthogonal to where looking and parallel to ground
    let lr = cross(v3!(0.0, 1.0, 0.0), fb);
    // forward backwards  is orthogonal to left right and and parallel to ground
    let translate = match direction {
        Direction::Backward => fb * epsilon,
        Direction::Forward => fb * epsilon * -1.0,
        Direction::Up => v3!(0., 1., 0.) * epsilon,
        Direction::Down => v3!(0., -1., 0.) * epsilon,
        Direction::Left => lr * epsilon * -1.0,
        Direction::Right => lr * epsilon,
    };
    params.look_from += translate;
    params.look_at += translate;
}

fn mesh_scene() -> Scene {
    let ground = Material {
        kind: MaterialKind::LAMBERTIAN,
        color: v3!(0.8, 0.8, 0),
        refractive_index: 0.0,
        roughness: 0.0,
    };

    let glass = Material {
        kind: MaterialKind::DIELECTRIC,
        color: v3!(0.1, 0.2, 0.5),
        refractive_index: 1.5,
        roughness: 0.,
    };

    let bluish = Material {
        kind: MaterialKind::METAL,
        color: v3!(0.1, 0.2, 0.5),
        refractive_index: 0.0,
        roughness: 1.0,
    };

    let metal = Material {
        kind: MaterialKind::LAMBERTIAN,
        color: v3!(0.8, 0.6, 0.2),
        refractive_index: 0.0,
        roughness: 1.0,
    };

    let cube = Mesh::from_file("models/cube.obj").expect("cube");
    let cube2 = cube.clone().translate(v3!(0, 0.0, 3.0));
    let cube3 = cube.clone().translate(v3!(-3., 0.0, 1.0));
    let plane = Mesh::from_file("models/plane.obj")
        .expect("plane")
        .translate(v3!(0, -1.01, 0));
    let dk = Mesh::from_file("models/dk-scaled.obj").expect("dk");
    // let icosphere = Mesh::from_file("models/icosphere.obj")
    //     .expect("icosphere")
    //     .translate(v3!(3, 0, 0));

    // let ground = Lambertian::obj(Vec3(0.8, 0.8, 0.0));
    // let left = Dielectric::obj(1.5);
    // let bubble = Dielectric::obj(1.0 / 1.5);
    // let center = Lambertian::obj(Vec3(0.1, 0.2, 0.5));
    // let right = Metal::obj(Vec3(0.8, 0.6, 0.2), 1.0);

    Scene::new()
        .with_mesh(dk, 2)
        .with_mesh(cube2, 1)
        .with_mesh(plane, 0)
        .with_mesh(cube3, 3)
        .with_material(ground)
        .with_material(glass)
        .with_material(bluish)
        .with_material(metal)
}
