pub struct TextureSwap {
    pub layout: wgpu::BindGroupLayout,
    bg: wgpu::BindGroup,
    swap_bg: wgpu::BindGroup,
    pub texture: wgpu::Texture,
    _swap_texture: wgpu::Texture,
    pub render_layout: wgpu::BindGroupLayout,
    render_bind_group: wgpu::BindGroup,
    swap_render_bind_group: wgpu::BindGroup,
}

impl TextureSwap {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let texture = Self::create_texture(device, width, height);
        let _swap_texture = Self::create_texture(device, width, height);
        let (layout, bg, swap_bg) = Self::create_bind_group(device, &texture, &_swap_texture);
        let (render_layout, render_bind_group, swap_render_bind_group) =
            Self::create_render_bind_group(device, &texture, &_swap_texture);
        Self {
            layout,
            bg,
            swap_bg,
            texture,
            _swap_texture,
            render_layout,
            render_bind_group,
            swap_render_bind_group,
        }
    }

    pub fn bind_group(&mut self) -> (&wgpu::BindGroup, &wgpu::BindGroup) {
        std::mem::swap(&mut self.bg, &mut self.swap_bg);
        std::mem::swap(
            &mut self.render_bind_group,
            &mut self.swap_render_bind_group,
        );
        (&self.bg, &self.render_bind_group)
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        let texture = Self::create_texture(device, width, height);
        let swap_texture = Self::create_texture(device, width, height);
        let (_, bg, swap_bg) = Self::create_bind_group(device, &texture, &swap_texture);
        let (_, rbg, swap_rbg) = Self::create_render_bind_group(device, &texture, &swap_texture);
        self.bg = bg;
        self.swap_bg = swap_bg;
        self.render_bind_group = rbg;
        self.swap_render_bind_group = swap_rbg;
    }

    fn create_bind_group(
        device: &wgpu::Device,
        texture: &wgpu::Texture,
        swap_texture: &wgpu::Texture,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup, wgpu::BindGroup) {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("texture"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    visibility: wgpu::ShaderStages::COMPUTE,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    count: None,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    visibility: wgpu::ShaderStages::COMPUTE,
                },
            ],
        });
        let (bind_group, swap_bind_group) =
            Self::create_bind_groups(device, &layout, texture, swap_texture);
        (layout, bind_group, swap_bind_group)
    }

    fn create_bind_groups(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        texture: &wgpu::Texture,
        swap_texture: &wgpu::Texture,
    ) -> (wgpu::BindGroup, wgpu::BindGroup) {
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let swap_texture_view = swap_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("texture_bg"),
            layout: layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&swap_texture_view),
                },
            ],
        });

        let swap_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("texture_bg"),
            layout: layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&swap_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
            ],
        });
        (bg, swap_bg)
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

    fn create_render_bind_group(
        device: &wgpu::Device,
        texture: &wgpu::Texture,
        swap_texture: &wgpu::Texture,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup, wgpu::BindGroup) {
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

        let swap_texture_view = swap_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let swap_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let swap_render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
        (
            render_bind_group_layout,
            render_bind_group,
            swap_render_bind_group,
        )
    }
}
