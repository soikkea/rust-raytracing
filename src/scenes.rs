use std::sync::Arc;

use rand::{Rng, SeedableRng};

use crate::{
    aarect::{XYRect, XZRect, YZRect},
    camera::Camera,
    hittable_list::HittableList,
    material::{Dielectric, DiffuseLight, Lambertian, Material, MaterialPtr, Metal},
    moving_sphere::MovingSphere,
    render::RenderConfig,
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture, NoiseTexture, TexturePtr},
    vec3::{Color, Point3, Vec3},
};

pub enum Scene {
    Random,
    TwoSpheres,
    TwoPerlinSpheres,
    Earth,
    SimpleLight,
    CornellBox,
}

pub struct SceneConfig {
    pub camera: Camera,
    pub world: HittableList,
    pub background: Color,
}

impl SceneConfig {
    pub fn get_scene(config: &RenderConfig) -> SceneConfig {
        let v_up = Vec3::new(0.0, 1.0, 0.0);
        let mut v_fov = 20.0;
        let mut aperture = 0.0;
        let time0 = 0.0;
        let time1 = 1.0;
        let mut look_from = Point3::new(13.0, 2.0, 3.0);
        let mut look_at = Point3::new(0.0, 0.0, 0.0);
        let world;
        let focus_dist = 10.0;
        let mut background = Color::origin();
        match &config.scene {
            Scene::Random => {
                world = random_scene();
                background = Color::new(0.70, 0.80, 1.00);
                aperture = 0.1;
            }
            Scene::TwoSpheres => {
                world = two_spheres();
                background = Color::new(0.70, 0.80, 1.00);
            }
            Scene::TwoPerlinSpheres => {
                world = two_perlin_spheres();
                background = Color::new(0.70, 0.80, 1.00);
            }
            Scene::Earth => {
                world = earth();
                background = Color::new(0.70, 0.80, 1.00);
            }
            Scene::SimpleLight => {
                world = simple_light();
                look_from = Point3::new(26.0, 3.0, 6.0);
                look_at = Point3::new(0.0, 2.0, 0.0);
            }
            Scene::CornellBox => {
                world = cornell_box();
                look_from = Point3::new(278.0, 278.0, -800.0);
                look_at = Point3::new(278.0, 278.0, 0.0);
                v_fov = 40.0;
            },
        }
        let camera = Camera::new_with_time(
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
        SceneConfig {
            camera,
            world,
            background,
        }
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

fn two_perlin_spheres() -> HittableList {
    let mut world = HittableList::new();

    let per_text: TexturePtr = Arc::new(NoiseTexture::new(4.0));

    let material: MaterialPtr = Arc::new(Lambertian::new(&per_text));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &material,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        &material,
    )));

    world
}

fn earth() -> HittableList {
    let mut world = HittableList::new();

    let earth_texture: TexturePtr = Arc::new(ImageTexture::new("earthmap.jpg"));

    let earth_surface: MaterialPtr = Arc::new(Lambertian::new(&earth_texture));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        &earth_surface,
    )));

    world
}

fn simple_light() -> HittableList {
    let mut world = HittableList::new();

    let per_text: TexturePtr = Arc::new(NoiseTexture::new(4.0));

    let material: MaterialPtr = Arc::new(Lambertian::new(&per_text));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        &material,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        &material,
    )));

    let diff_light: MaterialPtr =
        Arc::new(DiffuseLight::new_from_color(&Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, &diff_light)));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        2.0,
        &diff_light,
    )));

    world
}

fn cornell_box() -> HittableList {
    let mut world = HittableList::new();

    let red: MaterialPtr = Arc::new(Lambertian::new_from_color(&Color::new(0.65, 0.05, 0.05)));
    let white: MaterialPtr = Arc::new(Lambertian::new_from_color(&Color::new(0.73, 0.73, 0.73)));
    let green: MaterialPtr = Arc::new(Lambertian::new_from_color(&Color::new(0.12, 0.45, 0.15)));
    let light: MaterialPtr = Arc::new(DiffuseLight::new_from_color(&Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &green)));
    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &red)));
    world.add(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, &light,
    )));
    world.add(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &white)));
    world.add(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white)));
    world.add(Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white)));

    world
}
