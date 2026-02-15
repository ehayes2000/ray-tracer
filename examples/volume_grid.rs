use std::{fs::OpenOptions, io::BufWriter};

use ray_tracer::{
    camera::{Camera, CameraParameters, RenderParameters},
    hittable::{Hitify, HittableList},
    material::{DiffuseLight, Lambertian, Materialify},
    mesh::Mesh,
    v3,
};
use std::sync::Arc;

fn main() {
    let white = Lambertian::new(v3!(0.73, 0.73, 0.73)).materialify();
    let light_behind = DiffuseLight::new(v3!(10, 10, 10)).materialify();
    let light_top = DiffuseLight::new(v3!(15, 4, 4)).materialify(); // red
    let light_bottom = DiffuseLight::new(v3!(4, 15, 4)).materialify(); // green
    let light_left = DiffuseLight::new(v3!(4, 4, 15)).materialify(); // blue
    let light_right = DiffuseLight::new(v3!(15, 12, 4)).materialify(); // orange

    let mut world = HittableList::empty();

    // Light behind the camera (camera at z=-800)
    world.add(
        Mesh::quad(
            v3!(100, 500, -900),
            v3!(400, 0, 0),
            v3!(0, 0, 200),
            light_behind,
        )
        .hittable(),
    );
    // Top light (just above visible frame)
    world.add(Mesh::quad(v3!(100, 900, 50), v3!(400, 0, 0), v3!(0, 0, 300), light_top).hittable());
    // Bottom light (just below visible frame)
    world.add(
        Mesh::quad(
            v3!(100, -350, 50),
            v3!(400, 0, 0),
            v3!(0, 0, 300),
            light_bottom,
        )
        .hittable(),
    );
    // Left light (just left of visible frame)
    world.add(
        Mesh::quad(
            v3!(-350, 100, 50),
            v3!(0, 400, 0),
            v3!(0, 0, 300),
            light_left,
        )
        .hittable(),
    );
    // Right light (just right of visible frame)
    world.add(
        Mesh::quad(
            v3!(900, 100, 50),
            v3!(0, 400, 0),
            v3!(0, 0, 300),
            light_right,
        )
        .hittable(),
    );

    // 5x5 grid of volume cubes
    // User labels grid (2,2) to (6,6) with center at (4,4)
    // We use loop indices 0..5, offset from center = (i-2, j-2)
    let cube_size = 80.0_f64;
    let spacing = 140.0_f64;
    let grid_center_x = 300.0_f64;
    let grid_center_y = 300.0_f64;
    let grid_z = 200.0_f64;
    let rotation_scale = 0.25_f64; // radians per grid step
    let half_pi = std::f64::consts::FRAC_PI_2;

    for row in 0..5_i32 {
        for col in 0..5_i32 {
            let offset_row = (row - 2) as f64;
            let offset_col = (col - 2) as f64;

            let x = grid_center_x + offset_col * spacing - cube_size / 2.0;
            let y = grid_center_y + offset_row * spacing - cube_size / 2.0;

            // Rotation proportional to offset from center (4,4)
            let rot_x = offset_row * rotation_scale;
            let rot_y = offset_col * rotation_scale;

            world.add(
                Mesh::volume(
                    v3!(x, y, grid_z),
                    v3!(cube_size, cube_size, cube_size),
                    white.clone(),
                )
                .rotate(v3!(rot_x, rot_y, 0))
                .hittable(),
            );
        }
    }

    // 6 cubes in the top-left showing every face of the cube
    // Arranged in a single row
    let face_spacing = 100.0_f64;
    let face_base_x = -200.0_f64;
    let face_y = 720.0_f64;
    let pi = std::f64::consts::PI;

    let face_rotations: [(f64, f64, f64); 6] = [
        (0.0, 0.0, 0.0),      // front face (default, facing camera)
        (0.0, pi, 0.0),       // back face
        (-half_pi, 0.0, 0.0), // top face
        (half_pi, 0.0, 0.0),  // bottom face
        (0.0, half_pi, 0.0),  // left face
        (0.0, -half_pi, 0.0), // right face
    ];

    for (i, (rx, ry, rz)) in face_rotations.iter().enumerate() {
        let x = face_base_x + (i as f64) * face_spacing;
        let y = face_y;

        world.add(
            Mesh::volume(
                v3!(x, y, grid_z),
                v3!(cube_size, cube_size, cube_size),
                white.clone(),
            )
            .rotate(v3!(*rx, *ry, *rz))
            .hittable(),
        );
    }

    let camera = Camera::new(
        CameraParameters {
            vfov: 55.,
            look_from: v3!(300, 300, -800),
            look_at: v3!(300, 300, 200),
            defocus_angle: 0.0,
            focus_distance: 1.0,
        },
        RenderParameters {
            image_width: 800.,
            aspect_ratio: 1.0,
            max_bounces: 15.,
            samples_per_pixel: 14. * 20.,
            background_color: v3!(0, 0, 0),
        },
    );

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("volume_grid.ppm")
        .expect("volume_grid.ppm");

    let writer = BufWriter::new(file);
    camera.render_multi(14, writer, Arc::new(world.into_bvh()));
}
