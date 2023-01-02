use std::sync::Arc;

use rand::{Rng, SeedableRng};

use crate::{
    aarect::{XYRect, XZRect, YZRect},
    box_struct::Box,
    bvh::BVHNode,
    camera::Camera,
    color::Background,
    constant_medium::ConstantMedium,
    hittable::{HittablePtr, RotateY, Translate},
    hittable_list::HittableList,
    material::{Dielectric, DiffuseLight, Lambertian, Material, MaterialPtr, Metal},
    moving_sphere::MovingSphere,
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
    CornellSmoke,
    FinalScene,
}

pub struct SceneConfig {
    pub camera: Camera,
    pub world: HittableList,
    pub background: Background,
    pub image_width: u32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl SceneConfig {
    pub fn get_scene(scene: Scene) -> SceneConfig {
        let v_up = Vec3::new(0.0, 1.0, 0.0);
        let mut v_fov = 20.0;
        let mut aperture = 0.0;
        let time0 = 0.0;
        let time1 = 1.0;
        let mut look_from = Point3::new(13.0, 2.0, 3.0);
        let mut look_at = Point3::new(0.0, 0.0, 0.0);
        let world;
        let focus_dist = 10.0;
        let mut background = Background::Solid(Color::origin());

        let mut image_width = 400;
        let mut aspect_ratio = 16.0 / 9.0;
        let mut sample_per_pixel = 100;
        let max_depth = 50;
        match scene {
            Scene::Random => {
                world = random_scene();
                background =
                    Background::Gradient(Color::new(0.5, 0.7, 1.0), Color::new(1.0, 1.0, 1.0));
                aperture = 0.1;
            }
            Scene::TwoSpheres => {
                world = two_spheres();
                background = Background::Solid(Color::new(0.70, 0.80, 1.00));
            }
            Scene::TwoPerlinSpheres => {
                world = two_perlin_spheres();
                background = Background::Solid(Color::new(0.70, 0.80, 1.00));
            }
            Scene::Earth => {
                world = earth();
                background = Background::Solid(Color::new(0.70, 0.80, 1.00));
            }
            Scene::SimpleLight => {
                world = simple_light();
                look_from = Point3::new(26.0, 3.0, 6.0);
                look_at = Point3::new(0.0, 2.0, 0.0);
                sample_per_pixel = 400;
            }
            Scene::CornellBox => {
                world = cornell_box();
                look_from = Point3::new(278.0, 278.0, -800.0);
                look_at = Point3::new(278.0, 278.0, 0.0);
                v_fov = 40.0;
                aspect_ratio = 1.0;
                image_width = 600;
                sample_per_pixel = 200;
            }
            Scene::CornellSmoke => {
                world = cornell_smoke();
                look_from = Point3::new(278.0, 278.0, -800.0);
                look_at = Point3::new(278.0, 278.0, 0.0);
                v_fov = 40.0;
                aspect_ratio = 1.0;
                image_width = 600;
                sample_per_pixel = 200;
            }
            Scene::FinalScene => {
                world = final_scene();
                look_from = Point3::new(478.0, 278.0, -600.0);
                look_at = Point3::new(278.0, 278.0, 0.0);
                v_fov = 40.0;
                aspect_ratio = 1.0;
                image_width = 800;
                sample_per_pixel = 10000;
            }
        }
        let camera = Camera::new_with_time(
            look_from,
            look_at,
            v_up,
            v_fov,
            aspect_ratio,
            aperture,
            focus_dist,
            time0,
            time1,
        );
        SceneConfig {
            camera,
            world,
            background,
            image_width,
            aspect_ratio,
            samples_per_pixel: sample_per_pixel,
            max_depth,
        }
    }

    pub fn image_size(&self) -> (u32, u32) {
        let image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        (self.image_width, image_height)
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

    let box1: HittablePtr = Arc::new(Box::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        &white,
    ));

    let box1: HittablePtr = Arc::new(RotateY::new(&box1, 15.0));
    let box1: HittablePtr = Arc::new(Translate::new(&box1, &Vec3::new(265.0, 0.0, 295.0)));

    world.add(box1);

    let box2: HittablePtr = Arc::new(Box::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(168.0, 165.0, 165.0),
        &white,
    ));

    let box2: HittablePtr = Arc::new(RotateY::new(&box2, -18.0));
    let box2: HittablePtr = Arc::new(Translate::new(&box2, &Vec3::new(130.0, 0.0, 65.0)));

    world.add(box2);

    world
}

fn cornell_smoke() -> HittableList {
    let mut world = HittableList::new();

    let red: MaterialPtr = Arc::new(Lambertian::new_from_color(&Color::new(0.65, 0.05, 0.05)));
    let white: MaterialPtr = Arc::new(Lambertian::new_from_color(&Color::new(0.73, 0.73, 0.73)));
    let green: MaterialPtr = Arc::new(Lambertian::new_from_color(&Color::new(0.12, 0.45, 0.15)));
    let light: MaterialPtr = Arc::new(DiffuseLight::new_from_color(&Color::new(7.0, 7.0, 7.0)));

    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &green)));
    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &red)));
    world.add(Arc::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, &light,
    )));
    world.add(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white)));
    world.add(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, &white)));
    world.add(Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, &white)));

    let box1: HittablePtr = Arc::new(Box::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        &white,
    ));

    let box1: HittablePtr = Arc::new(RotateY::new(&box1, 15.0));
    let box1: HittablePtr = Arc::new(Translate::new(&box1, &Vec3::new(265.0, 0.0, 295.0)));

    let box2: HittablePtr = Arc::new(Box::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(168.0, 165.0, 165.0),
        &white,
    ));

    let box2: HittablePtr = Arc::new(RotateY::new(&box2, -18.0));
    let box2: HittablePtr = Arc::new(Translate::new(&box2, &Vec3::new(130.0, 0.0, 65.0)));

    world.add(Arc::new(ConstantMedium::new_from_color(
        &box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    world.add(Arc::new(ConstantMedium::new_from_color(
        &box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    world
}

fn final_scene() -> HittableList {
    const RANDOM_SEED: u64 = 3;

    // Ground
    let mut boxes1 = HittableList::new();
    let ground: MaterialPtr = Arc::new(Lambertian::new_from_color(&Color::new(0.48, 0.83, 0.53)));

    let mut rng = rand_pcg::Pcg32::seed_from_u64(RANDOM_SEED);
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Box::new(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                &ground,
            )));
        }
    }

    let mut objects = HittableList::new();

    objects.add(Arc::new(BVHNode::from_hittable_list(&boxes1, 0.0, 1.0)));

    // Light
    let light: MaterialPtr = Arc::new(DiffuseLight::new_from_color(&Color::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, &light,
    )));

    // Moving sphere
    let center1 = Point3::new(400.0, 400.0, 400.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material: MaterialPtr =
        Arc::new(Lambertian::new_from_color(&Color::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        &moving_sphere_material,
    )));

    // Glass and metal spheres
    let dielectric_material: MaterialPtr = Arc::new(Dielectric::new(1.5));
    objects.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        &dielectric_material,
    )));
    let material: MaterialPtr = Arc::new(Metal::new(&Color::new(0.8, 0.8, 0.9), 1.0));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        &material,
    )));

    // Glass spheres with fog inside
    let boundary: HittablePtr = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        &dielectric_material,
    ));
    objects.add(Arc::clone(&boundary));
    objects.add(Arc::new(ConstantMedium::new_from_color(
        &boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary: HittablePtr = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        &dielectric_material,
    ));
    objects.add(Arc::new(ConstantMedium::new_from_color(
        &boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    // Earth
    let earth_texture: TexturePtr = Arc::new(ImageTexture::new("earthmap.jpg"));
    let emat: MaterialPtr = Arc::new(Lambertian::new(&earth_texture));
    objects.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        &emat,
    )));
    let per_text: TexturePtr = Arc::new(NoiseTexture::new(0.1));
    let per_mat: MaterialPtr = Arc::new(Lambertian::new(&per_text));
    objects.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        &per_mat,
    )));

    // Box of spheres
    let mut boxes2 = HittableList::new();
    let white: MaterialPtr = Arc::new(Lambertian::new_from_color(&Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random_range(0.0, 165.0),
            10.0,
            &white,
        )));
    }

    let hittable: HittablePtr = Arc::new(BVHNode::from_hittable_list(&boxes2, 0.0, 1.0));
    let rotate_y: HittablePtr = Arc::new(RotateY::new(&hittable, 15.0));
    objects.add(Arc::new(Translate::new(
        &rotate_y,
        &Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
}
