use rand::Rng;

use crate::vec3::{unit_vector, Point3, Vec3};

const POINT_COUNT: usize = 256;
pub struct Perlin {
    ran_vec: Vec<Vec3>,
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut ran_vec = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ran_vec.push(unit_vector(&Vec3::random_range(-1.0, 1.0)));
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Perlin {
            ran_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, point: &Point3) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();
        let i = point.x().floor() as i32;
        let j = point.y().floor() as i32;
        let k = point.z().floor() as i32;

        let mut c: Vec<Vec<Vec<Vec3>>> = vec![vec![vec![Vec3::origin(); 2]; 2]; 2];

        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ran_vec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        Perlin::perlin_interpolation(&c, u, v, w)
    }

    /// Turbulence function
    ///
    /// Default value for depth is 7.
    pub fn turbulence(&self, point: &Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn perlin_generate_perm() -> [i32; POINT_COUNT] {
        let mut p: [i32; POINT_COUNT] = [0; POINT_COUNT];

        #[allow(clippy::needless_range_loop)]
        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }

        Perlin::permutate(&mut p);

        p
    }

    fn permutate(p: &mut [i32; POINT_COUNT]) {
        let mut rng = rand::thread_rng();
        for i in (1..POINT_COUNT).rev() {
            let target = rng.gen_range(0..i);
            p.swap(i, target);
        }
    }

    fn _trilinear_intepolation(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;

        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            let i_f = i as f64;
            for j in 0..2 {
                let j_f = j as f64;
                for k in 0..2 {
                    let k_f = k as f64;
                    accum += (i_f * u + (1.0 - i_f) * (1.0 - u))
                        * (j_f * v + (1.0 - j_f) * (1.0 - v))
                        * (k_f * w + (1.0 - k_f) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }

        accum
    }

    fn perlin_interpolation(c: &[Vec<Vec<Vec3>>], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            let i_f = i as f64;
            for j in 0..2 {
                let j_f = j as f64;
                for k in 0..2 {
                    let k_f = k as f64;
                    let weight_v = Vec3::new(u - i_f, v - j_f, w - k_f);
                    accum += (i_f * uu + (1.0 - i_f) * (1.0 - uu))
                        * (j_f * vv + (1.0 - j_f) * (1.0 - vv))
                        * (k_f * ww + (1.0 - k_f) * (1.0 - ww))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }

        accum
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
