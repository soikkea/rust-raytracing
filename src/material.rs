use crate::{hittable, ray, vec3};

pub struct ScatterResult {
    pub attenuation: vec3::Color,
    pub scattered: ray::Ray,
}

pub trait Material: Send + Sync {
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
        let reflected = vec3::reflect(&vec3::unit_vector(&ray_in.direction()), &rec.normal);
        let scattered = ray::Ray::new(
            &rec.p,
            &(reflected + self.fuzz * &vec3::random_in_unit_sphere()),
        );
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

        let unit_direction = vec3::unit_vector(ray_in.direction());
        let cos_theta = vec3::dot(&-&unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>()
        {
            vec3::reflect(&unit_direction, &rec.normal)
        } else {
            vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scattered = ray::Ray::new(&rec.p, &direction);
        Option::Some(ScatterResult {
            attenuation,
            scattered,
        })
    }
}
