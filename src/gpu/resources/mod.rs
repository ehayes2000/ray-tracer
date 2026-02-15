mod texture_swap;
use encase::StorageBuffer;
use rand::random;
use std::num::NonZeroU64;
pub use texture_swap::TextureSwap;
use wgpu::{BufferUsages, util::DeviceExt};

use crate::gpu::{
    bvh::{BvhShaderArray, BvhShaderNode},
    types::{Material, RenderParameters, SceneBufferEntry, Triangle},
};
use encase::{ShaderType, internal::WriteInto};

pub struct Resources {
    bvh: wgpu::Buffer,
    params: wgpu::Buffer,
    materials: wgpu::Buffer,
    seed: wgpu::Buffer,
    pub pass_count: wgpu::Buffer,
    pub compute_bind_group_layout: wgpu::BindGroupLayout,
    pub compute_bind_group: wgpu::BindGroup,
    pub texture_swap: TextureSwap,
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
        let swap = TextureSwap::new(device, width, height);
        let bvh = Self::create_buffer(device, &bvh_buff.0, "bvh_buf", BufferUsages::STORAGE);
        let materials = Self::create_buffer(device, &materials, "scene_buf", BufferUsages::STORAGE);
        let params =
            Self::create_buffer(device, &render_params, "scene_buf", BufferUsages::UNIFORM);
        let pass_count = Self::create_buffer(
            device,
            &[1u32],
            "pass_count",
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );

        let seed_data = vec![0; (width * height) as _]
            .into_iter()
            .map(|_| random())
            .collect::<Vec<u32>>();
        let seed = Self::create_buffer(device, &seed_data, "seed", BufferUsages::STORAGE);
        let (cbgl, cbg) =
            Self::create_compute_bind_group(device, &bvh, &materials, &params, &pass_count, &seed);

        Self {
            bvh,
            params,
            materials,
            pass_count,
            seed,
            texture_swap: swap,
            compute_bind_group: cbg,
            compute_bind_group_layout: cbgl,
        }
    }

    pub fn texture_bg_layout(&self) -> &wgpu::BindGroupLayout {
        &self.texture_swap.layout
    }

    // 0 output texture
    // 1 scene buffer
    // 2 bvh buffer
    // 3 materials buffer
    // 4 render params
    fn create_compute_bind_group(
        device: &wgpu::Device,
        bvh: &wgpu::Buffer,
        materials: &wgpu::Buffer,
        params: &wgpu::Buffer,
        pass_count: &wgpu::Buffer,
        seed: &wgpu::Buffer,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compute_bg_layout"),
            entries: &[
                // bvh buf
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
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
                    binding: 1,
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
                    binding: 2,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(RenderParameters::min_size()),
                    },
                    visibility: wgpu::ShaderStages::COMPUTE,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(NonZeroU64::new(4).expect("size")),
                    },
                    visibility: wgpu::ShaderStages::COMPUTE,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: Some(NonZeroU64::new(seed.size()).expect("size")),
                    },
                    visibility: wgpu::ShaderStages::COMPUTE,
                },
            ],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("compute_bg"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: bvh,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: materials,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: params,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: pass_count,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: seed,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });
        (bind_group_layout, bind_group)
    }

    fn create_buffer<T>(
        device: &wgpu::Device,
        data: &T,
        name: &str,
        usage: BufferUsages,
    ) -> wgpu::Buffer
    where
        T: ?Sized + ShaderType + WriteInto,
    {
        let mut buf = StorageBuffer::new(Vec::<u8>::new());
        buf.write(data)
            .expect("failed to write render parameter content");

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(name),
            contents: &buf.into_inner(),
            usage,
        })
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.texture_swap.resize(device, width, height);
    }
}
