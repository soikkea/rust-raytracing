use std::sync::Arc;

use rand::{Rng, SeedableRng};

use crate::{
    camera::Camera,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Material, MaterialPtr, Metal},
    moving_sphere::MovingSphere,
    render::RenderConfig,
    sphere::Sphere,
    texture::{CheckerTexture, TexturePtr},
    vec3::{Color, Point3, Vec3},
};

pub enum Scene {
    Random,
    TwoSpheres,
}

pub struct SceneConfig {
    pub camera: Camera,
    pub world: HittableList,
}

impl SceneConfig {
    pub fn get_scene(config: &RenderConfig) -> SceneConfig {
        let v_up = Vec3::new(0.0, 1.0, 0.0);
        let v_fov; // 40.0
        let mut aperture = 0.0;
        let time0 = 0.0;
        let time1 = 1.0;
        let look_from;
        let look_at;
        let world;
        let camera;
        let focus_dist = 10.0;
        match &config.scene {
            Scene::Random => {
                world = random_scene();
                look_from = Point3::new(13.0, 2.0, 3.0);
                look_at = Point3::new(0.0, 0.0, 0.0);
                v_fov = 20.0;
                aperture = 0.1;
            }
            Scene::TwoSpheres => {
                world = two_spheres();
                look_from = Point3::new(13.0, 2.0, 3.0);
                look_at = Point3::new(0.0, 0.0, 0.0);
                v_fov = 20.0;
            }
        }
        camera = Camera::new_with_time(
            look_from,
            look_at,
            v_up,
            v_fov,
            config.aspect_ratio(),
            aperture,
            focus_dist,
            time0,
            time1,
        );
        SceneConfig { camera, world }
    }
}

fn random_scene() -> HittableList {
    const RANDOM_SEED: u64 = 2;

    let mut world = HittableList::new();

    let checker: TexturePtr = Arc::new(CheckerTexture::new_from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(&checker));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    )));

    let mut rng = rand_pcg::Pcg32::seed_from_u64(RANDOM_SEED);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.00)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new_from_color(&albedo));
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        &sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, &sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, &sphere_material)));
                }
            }
        }

        let material: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
        world.add(Arc::new(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            &material,
        )));

        let material: Arc<dyn Material> =
            Arc::new(Lambertian::new_from_color(&Color::new(0.4, 0.2, 0.1)));
        world.add(Arc::new(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            &material,
        )));

        let material: Arc<dyn Material> = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
        world.add(Arc::new(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            &material,
        )));
    }

    let material_center: Arc<dyn Material> =
        Arc::new(Lambertian::new_from_color(&Color::new(0.1, 0.2, 0.5)));
    let material_left: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let material_right: Arc<dyn Material> = Arc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        &material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        &material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        &material_right,
    )));

    world
}

fn two_spheres() -> HittableList {
    let mut world = HittableList::new();

    let checker: TexturePtr = Arc::new(CheckerTexture::new_from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let material: MaterialPtr = Arc::new(Lambertian::new(&checker));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        &material,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        &material,
    )));

    world
}
