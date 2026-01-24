use encase::StorageBuffer;
use wgpu::util::DeviceExt;

use crate::gpu::{
    bvh::{BvhShaderArray, BvhShaderNode},
    types::{Material, RenderParameters, SceneBufferEntry, Triangle},
};
use encase::ShaderType;
use encase::internal::WriteInto;

pub struct Resources {
    pub texture: wgpu::Texture,
    bvh: wgpu::Buffer,
    params: wgpu::Buffer,
    materials: wgpu::Buffer,
    pub render_bind_group_layout: wgpu::BindGroupLayout,
    pub render_bind_group: wgpu::BindGroup,
    pub compute_bind_group_layout: wgpu::BindGroupLayout,
    pub compute_bind_group: wgpu::BindGroup,
}

impl Resources {
    pub fn create(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        bvh_buff: BvhShaderArray<Triangle>,
        materials: Vec<Material>,
        render_params: RenderParameters,
    ) -> Self {
        let texture = Self::create_texture(device, width, height);
        let bvh = Self::create_buffer(device, &bvh_buff.0, "bvh_buf");
        let materials = Self::create_buffer(device, &materials, "scene_buf");
        let params = Self::create_buffer(device, &RenderParameters::default(), "scene_buf");
        let (cbgl, cbg) =
            Self::create_compute_bind_group(device, &texture, &bvh, &materials, &params);
        let (rbgl, rbg) = Self::create_render_bind_group(device, &texture);
        Self {
            bvh,
            params,
            materials,
            texture,
            compute_bind_group: cbg,
            compute_bind_group_layout: cbgl,
            render_bind_group: rbg,
            render_bind_group_layout: rbgl,
        }
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

    // 0 output texture
    // 1 scene buffer
    // 2 bvh buffer
    // 3 materials buffer
    // 4 render params
    fn create_compute_bind_group(
        device: &wgpu::Device,
        texture: &wgpu::Texture,
        bvh: &wgpu::Buffer,
        materials: &wgpu::Buffer,
        params: &wgpu::Buffer,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compute_bg_layout"),
            entries: &[
                // output texture
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    visibility: wgpu::ShaderStages::COMPUTE,
                },
                // bvh buf
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: Some(BvhShaderNode::<Triangle>::min_size()),
                    },
                    visibility: wgpu::ShaderStages::COMPUTE,
                },
                // materials
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: Some(SceneBufferEntry::min_size()),
                    },
                    visibility: wgpu::ShaderStages::COMPUTE,
                },
                // render params
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(RenderParameters::min_size()),
                    },
                    visibility: wgpu::ShaderStages::COMPUTE,
                },
            ],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("compute_bg"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: bvh,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: materials,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: params,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });
        (bind_group_layout, bind_group)
    }

    fn create_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
        device.create_texture(&wgpu::wgt::TextureDescriptor {
            label: Some("texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING,
            view_formats: &[],
        })
    }

    fn create_buffer<T>(device: &wgpu::Device, data: &T, name: &str) -> wgpu::Buffer
    where
        T: ?Sized + ShaderType + WriteInto,
    {
        let mut buf = StorageBuffer::new(Vec::<u8>::new());
        buf.write(data)
            .expect("failed to write render parameter content");

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(name),
            contents: &buf.into_inner(),
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::STORAGE,
        })
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.texture = Self::create_texture(device, width, height);
        // TODO: this also creates a layout. probably bad
        self.compute_bind_group = Self::create_compute_bind_group(
            device,
            &self.texture,
            &self.bvh,
            &self.materials,
            &self.params,
        )
        .1;
        self.render_bind_group = Self::create_render_bind_group(device, &self.texture).1;
    }
}
