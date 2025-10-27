// useful docs
// wgpu: https://docs.rs/wgpu/latest
// wgsl: https://www.w3.org/TR/WGSL
// tour of wgsl: https://google.github.io/tour-of-wgsl
// learn wgpu: https://sotrh.github.io/learn-wgpu

// WGSL doesn't use Rust or C layout. This package provides a trait to align
// structs for WGSL and some types to create, read, and write buffers
use encase::{ShaderType, StorageBuffer};
use std::io::Write;
use wgpu::util::DeviceExt;

const WIDTH: u64 = 512;
const HEIGHT: u64 = 512;

// why does ShaderType not solve layout?
// Do I need an attribute?
#[derive(Clone, Debug, ShaderType)]
pub struct Sphere {
    pub radius: f32,
    pub _pad: f32,
    pub location: [f32; 2],
}

#[pollster::main]
async fn main() {
    // Get an instance (top level wgpu abstraction)
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backend_options: wgpu::BackendOptions::default(),
        flags: wgpu::InstanceFlags::default(),
        backends: wgpu::Backends::default(),
        memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
    });
    // Get an adapter (handle to a physical gpu)
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            // headless mode -> no surface
            compatible_surface: None,
        })
        .await
        .expect("adapter");
    // Get a device (an open connection to a physical gpu)
    // connection is a device and command queue
    let (device, cmd_queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("device"),
            ..Default::default()
        })
        .await
        .expect("device");

    // buffer size in bytes. gpus like u32's -> elems * 4
    let img_buffer_size = WIDTH * HEIGHT * 4;

    // create buffers
    let mut scene_buf = StorageBuffer::new(Vec::<u8>::new());
    scene_buf
        .write(&Sphere {
            location: [256., 256.],
            _pad: 0.,
            radius: 128.,
        })
        .expect("write to scene buffer");

    let scene_buf = scene_buf.into_inner();
    let scene_buf_size = scene_buf.len() as u64;
    println!("scene_buf_size {}", scene_buf_size);
    println!("minimum_sphere_size {}", Sphere::min_size());

    let gpu_scene_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("scene_buffer"),
        contents: &scene_buf,
        usage: wgpu::BufferUsages::STORAGE,
    });
    println!("gpu_scene_buffer_size {}", gpu_scene_buffer.size());
    // img output buffer
    let img_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("img_buffer"),
        // mapped = cpu can read bug gpu can't use
        mapped_at_creation: false,
        size: img_buffer_size,
        // store stuff in here and we copy stuff from here
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    // this is the buffer we copy into to see wtf this thing actually did
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("staging_buffer"),
        mapped_at_creation: false,
        size: img_buffer_size,
        // read from and copy to this buffer
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
    });
    // one of the most balls interfaces is bind groups. we need to create and bind this buffer
    // in the same way that we use it in the shader
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            // output buffer layout
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                count: None,
                ty: wgpu::BindingType::Buffer {
                    has_dynamic_offset: false,
                    min_binding_size: None,
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                },
                visibility: wgpu::ShaderStages::COMPUTE,
            },
            // scene buffer layout
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                count: None,
                ty: wgpu::BindingType::Buffer {
                    has_dynamic_offset: false,
                    min_binding_size: Some(Sphere::min_size()),
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                },
                visibility: wgpu::ShaderStages::COMPUTE,
            },
        ],
        label: Some("bind_group_layout_0"),
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        // we only bind the buffer we are writing to
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: img_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: gpu_scene_buffer.as_entire_binding(),
            },
        ],
        label: Some("bind_group_0"),
        layout: &layout,
    });

    // create an encoder
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("command_encoder"),
    });
    // a compute pass defines the work that will be done
    let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
        label: Some("compute_pass"),
        ..Default::default()
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&layout],
        label: Some("pipeline_layout"),
        push_constant_ranges: &[],
    });
    // compile shader and crate a module
    let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
    // create compute pipeline
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        // name for debugging
        label: Some("compute_pipeline"),
        // use shader
        module: &shader,
        cache: None,
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        entry_point: None,
        layout: Some(&pipeline_layout),
    });
    pass.set_pipeline(&pipeline);
    pass.set_bind_group(0, &bind_group, &[]);
    // important!. x * y * z _workgroups_ will be dispatched
    // the shader defines @workgroup_size(wx,wy,wz) so the shader will run on wx * wy * wz threads
    pass.dispatch_workgroups(32, 32, 1);
    // this is how we tell the encoder that the compute pass is over
    // this api is turbo balls. there's probably a reason why it's such balls but idk
    drop(pass);
    // copy from img to staging buffer after compute pass
    encoder.copy_buffer_to_buffer(&img_buffer, 0, &staging_buffer, 0, img_buffer_size);
    // this runs the compute pass
    let encoded_commands = encoder.finish();
    cmd_queue.submit(Some(encoded_commands));
    // block until compute is done
    device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("wait for compute pass to finish");
    println!("finished pipe");

    let output_data = staging_buffer.slice(..);
    // this will map after pass
    output_data.map_async(wgpu::MapMode::Read, |r| {
        if r.is_err() {
            eprintln!("maping staging buffer error {:?}", r);
            panic!()
        }
    });

    device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("wait for compute pass to finish");

    let buffer = output_data.get_mapped_range();
    // kindof like a muckbang but for bytes
    let data: &[u32] = bytemuck::cast_slice(&buffer);
    // println!("{:?}", data);
    // for i in 0..8 {
    //     for j in 0..8 {
    //         print!("{:>2} ", data[i * 8 + j])
    //     }
    //     println!();
    // }
    to_ppm(data).expect("write ppm");
}

fn to_ppm(buf: &[u32]) -> Result<(), std::io::Error> {
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("out.ppm")?;
    writeln!(f, "P3")?;
    writeln!(f, "{} {}", WIDTH, HEIGHT)?;
    writeln!(f, "255")?;
    println!("len arr {}", buf.len());
    for i in buf {
        writeln!(f, "{} 0 0", i)?;
    }
    Ok(())
}
