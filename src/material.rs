use crate::{hittable, ray, vec3};

pub struct ScatterResult {
    pub attenuation: vec3::Color,
    pub scattered: ray::Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &ray::Ray, rec: &hittable::HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    pub albedo: vec3::Color,
}

impl Lambertian {
    pub fn new(albedo: &vec3::Color) -> Lambertian {
        Lambertian { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &ray::Ray, rec: &hittable::HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = rec.normal() + vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Option::Some(ScatterResult {
            attenuation: self.albedo,
            scattered: ray::Ray::new(&rec.p, &scatter_direction),
        })
    }
}

pub struct Metal {
    pub albedo: vec3::Color,
}

impl Metal {
    pub fn new(albedo: &vec3::Color) -> Metal {
        Metal { albedo: *albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &ray::Ray, rec: &hittable::HitRecord) -> Option<ScatterResult> {
        let reflected = vec3::reflect(&vec3::unit_vector(&ray_in.direction()), &rec.normal);
        let scattered = ray::Ray::new(&rec.p, &reflected);
        if vec3::dot(scattered.direction(), &rec.normal) > 0.0 {
            Option::Some(ScatterResult {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}
