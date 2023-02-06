use std::sync::Arc;

use crate::{
    hittable::{self, HitRecord},
    ray::{self, Ray},
    texture::{SolidColor, TexturePtr},
    vec3::{self, random_in_unit_sphere, Color, Point3},
};

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterResult>;

    fn emitted(&self, _u: f64, _v: f64, _point: &Point3) -> Color {
        Color::origin()
    }
}

pub type MaterialPtr = Arc<dyn Material>;

pub struct Lambertian {
    pub albedo: TexturePtr,
}

impl Lambertian {
    pub fn new(albedo: &TexturePtr) -> Lambertian {
        Lambertian {
            albedo: Arc::clone(albedo),
        }
    }

    pub fn new_from_color(albedo: &vec3::Color) -> Lambertian {
        let texture: TexturePtr = Arc::new(SolidColor::new(*albedo));
        Lambertian::new(&texture)
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &ray::Ray, rec: &hittable::HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = rec.normal() + vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = ray::Ray::new(rec.p, scatter_direction, ray_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        Option::Some(ScatterResult {
            attenuation,
            scattered,
        })
    }
}

pub struct Metal {
    pub albedo: vec3::Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &vec3::Color, f: f64) -> Metal {
        Metal {
            albedo: *albedo,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &ray::Ray, rec: &hittable::HitRecord) -> Option<ScatterResult> {
        let reflected = vec3::reflect(&vec3::unit_vector(&ray_in.direction), &rec.normal);
        let scattered = ray::Ray::new(
            rec.p,
            reflected + self.fuzz * vec3::random_in_unit_sphere(),
            ray_in.time,
        );
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Option::Some(ScatterResult {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Shlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &ray::Ray, rec: &hittable::HitRecord) -> Option<ScatterResult> {
        let attenuation = vec3::Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = vec3::unit_vector(&ray_in.direction);
        let cos_theta = (-&unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>()
        {
            vec3::reflect(&unit_direction, &rec.normal)
        } else {
            vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scattered = ray::Ray::new(rec.p, direction, ray_in.time);
        Option::Some(ScatterResult {
            attenuation,
            scattered,
        })
    }
}

pub struct DiffuseLight {
    emit: TexturePtr,
}

impl DiffuseLight {
    pub fn new(emit: &TexturePtr) -> DiffuseLight {
        DiffuseLight {
            emit: Arc::clone(emit),
        }
    }

    pub fn new_from_color(color: &vec3::Color) -> DiffuseLight {
        let texture: TexturePtr = Arc::new(SolidColor::new(*color));
        DiffuseLight::new(&texture)
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, point: &Point3) -> Color {
        self.emit.value(u, v, point)
    }

    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<ScatterResult> {
        None
    }
}

pub struct Isotropic {
    albedo: TexturePtr,
}

impl Isotropic {
    pub fn new(albedo: &TexturePtr) -> Isotropic {
        Isotropic {
            albedo: Arc::clone(albedo),
        }
    }

    pub fn new_from_color(color: &vec3::Color) -> Isotropic {
        let texture: TexturePtr = Arc::new(SolidColor::new(*color));
        Isotropic::new(&texture)
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let scattered = Ray::new(rec.p, random_in_unit_sphere(), ray_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some(ScatterResult {
            attenuation,
            scattered,
        })
    }
}
