use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::texture::Texture;

fn random_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    let unit = Vector3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - unit;
        if p.magnitude_squared() < 1.0 {
            return p
        }
    }
}

fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(&n) * n
}

fn refract(v: &Vector3<f32>, n: &Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 -r0) * (1.0 - cosine).powi(5)
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)>;
}

pub struct Lambertian<T: Texture> {
    albedo: T
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self { Lambertian { albedo } }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p, ray.time());
        Some((scattered, self.albedo.value(hit.u, hit.v, &hit.p)))
    }
}

pub struct Metal {
    albedo: Vector3<f32>,
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Self {
        Metal { albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let mut reflected = reflect(&ray.direction().normalize(), &hit.normal);
        if self.fuzz > 0.0 { reflected += self.fuzz * random_in_unit_sphere() };
        if reflected.dot(&hit.normal) > 0.0 {
            let scattered = Ray::new(hit.p, reflected, ray.time());
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ref_idx: f32
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self { Dielectric { ref_idx } }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) = if ray.direction().dot(&hit.normal) > 0.0 {
            let cosine = self.ref_idx * ray.direction().dot(&hit.normal) / ray.direction().magnitude();
            (-hit.normal, self.ref_idx, cosine)
        } else {
            let cosine = -ray.direction().dot(&hit.normal) / ray.direction().magnitude();
            (hit.normal, 1.0 / self.ref_idx, cosine)
        };
        if let Some(refracted) = refract(&ray.direction(), &outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.ref_idx);
            if rand::thread_rng().gen::<f32>() >= reflect_prob {
                let scattered = Ray::new(hit.p, refracted, ray.time());
                return Some((scattered, attenuation))
            }
        }
        let reflected = reflect(&ray.direction(), &hit.normal);
        let scattered = Ray::new(hit.p, reflected, ray.time());
        Some((scattered, attenuation))
    }
}