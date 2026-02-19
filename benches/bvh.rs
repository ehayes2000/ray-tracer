use criterion::{Criterion, criterion_group, criterion_main};
use ray_tracer::{
    bvh::Bvh,
    hit::Hit,
    hittable_list::{Hitify, HittableList},
    material::{DiffuseLight, Lambertian, Materialify},
    math::{Float, Interval, Ray},
    mesh::Mesh,
    mesh_obj, v3,
};
use std::{f64, hint::black_box};

fn create_bvh() -> Bvh {
    let red = Lambertian::new(v3!(0.65, 0.05, 0.05)).materialify();
    let white = Lambertian::new(v3!(0.73, 0.73, 0.73)).materialify();
    let green = Lambertian::new(v3!(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(v3!(15, 15, 15));

    let world = HittableList::empty()
        // right wall
        .push(Mesh::quad(
            v3!(555, 0, 0),
            v3!(0, 555, 0),
            v3!(0, 0, 555),
            green,
        ))
        // light
        .push(Mesh::quad(
            v3!(343, 554, 332),
            v3!(-130, 0, 0),
            v3!(0, 0, -105),
            light,
        ))
        // back wall
        .push(Mesh::quad(
            v3!(0, 0, 555),
            v3!(555, 0, 0),
            v3!(0, 555, 0),
            white.clone(),
        ))
        // left wall
        .push(Mesh::quad(
            v3!(0, 0, 0),
            v3!(0, 555, 0),
            v3!(0, 0, 555),
            red.clone(),
        ))
        // roof
        .push(Mesh::quad(
            v3!(555, 555, 555),
            v3!(-555, 0, 0),
            v3!(0, 0, -555),
            white.clone(),
        ))
        // floor
        .push(Mesh::quad(
            v3!(0, 0, 0),
            v3!(555, 0, 0),
            v3!(0, 0, 555),
            white.clone(),
        ))
        .push(
            Mesh::volume(v3!(130, 0, 60), v3!(165, 165, 165), white.clone())
                .rotate(v3!(0, -0.3, 0)),
        )
        .push(
            Mesh::volume(v3!(265, 0, 295), v3!(165, 330, 165), white.clone())
                .rotate(v3!(0, 0.3, 0)),
        );
    world.into_bvh()
}

fn bench_bvh_cornell(c: &mut Criterion) {
    let bvh = create_bvh();
    let origin = v3!(277, 277, 277);
    let direction = v3!(1, 0, 1);
    let ray = Ray { origin, direction };
    let t = Interval::full();
    let mut group = c.benchmark_group("bvh_hit");
    group.bench_function("fixed_point", |b| b.iter(|| black_box(bvh.hit(&ray, &t))));

    group.bench_function("bvh_hit", |b| {
        let rays: Vec<Ray> = (0..1000)
            .map(|i| {
                let x = -1.0 + 2.0 * (i as f64 / 1000.0);
                Ray {
                    origin,
                    direction: v3!(x, 0, 1),
                }
            })
            .collect();

        b.iter(|| {
            for ray in &rays {
                black_box(bvh.hit(ray, &t));
            }
        })
    });

    group.finish();
}

fn bench_bvh_knight(c: &mut Criterion) {
    let mut group = c.benchmark_group("high_poly_bvh");
    let material = Lambertian::new(v3!(0.7, 0.7, 0.7)).materialify();
    let knight = mesh_obj!("models/chess_knight.obj", material).expect("load knight");
    let radius = knight.bounding_box()[knight.bounding_box().longest()].size() * 2.0;
    let look_at = knight.bounding_box().center();
    let bvh = Bvh::from_objects(vec![knight.hitify()]);
    let interval = Interval::full();
    let polar_steps = || {
        const STEPS: usize = 100;
        (0..=STEPS) // inclusive to hit both poles
            .map(|i| i as f64 * std::f64::consts::PI / STEPS as f64)
            .map(|f| f as Float)
    };

    let azimuthal_steps = || {
        const STEPS: usize = 15;
        (0..STEPS)
            .map(|i| i as f64 * 2.0 * std::f64::consts::PI / STEPS as f64)
            .map(|f| f as Float)
    };

    let rays = azimuthal_steps()
        .map(|rho| {
            polar_steps().map(move |phi| {
                let (sin_rho, cos_rho) = rho.sin_cos();
                let (sin_phi, cos_phi) = phi.sin_cos();
                let x = look_at.0 + radius * sin_phi * cos_rho;
                let y = look_at.1 + radius * sin_phi * sin_rho;
                let z = look_at.2 + radius * cos_phi;
                let look_from = v3!(x, y, z);
                Ray {
                    origin: look_from,
                    direction: (look_at - look_from).normalize(),
                }
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    group.bench_function("bvh_hit_sphere", |b| {
        b.iter(|| {
            for ray in &rays {
                if bvh.hit(&ray, &interval).is_none() {
                    eprintln!("\nexpected hit {:?}", ray);
                    eprintln!(
                        "\n
                        ray_origin_y_up = Vector(({}, {}, {}))
                        ray_direction_y_up = Vector(({}, {}, {}))
                    \n",
                        ray.origin.0,
                        ray.origin.1,
                        ray.origin.2,
                        ray.direction.0,
                        ray.direction.1,
                        ray.direction.2,
                    );
                    panic!("no hit");
                }
            }
        })
    });
    group.finish();
}

criterion_group!(benches, bench_bvh_knight, bench_bvh_cornell);
criterion_main!(benches);
