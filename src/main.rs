mod ray;
mod hitable;
mod texture;
mod perlin;
mod material;
mod sphere;
mod rect;
mod cube;
mod camera;
mod aabb;
mod bvh;

use std::f32;
use std::rc::Rc;
use nalgebra::Vector3;
use rand::Rng;
use image;
use crate::ray::Ray;
use crate::texture::{ConstantTexture, CheckerTexture, NoiseTexture, ImageTexture};
use crate::material::{Lambertian, Metal, Dielectric, DiffuseLight};
use crate::hitable::{Hitable, HitableList, FlipNormals};
use crate::sphere::{Sphere, MovingSphere};
use crate::rect::{AARect, Plane};
use crate::cube::Cube;
use crate::camera::Camera;
use crate::bvh::BVHNode;

fn random_scene() -> Box<Hitable> {
    let mut rng = rand::thread_rng();
    let origin = Vector3::new(4.0, 0.2, 0.0);
    let mut world: Vec<Rc<Hitable>> = Vec::new();
    let checker = CheckerTexture::new(ConstantTexture::new(0.2, 0.3, 0.1), ConstantTexture::new(0.9, 0.9, 0.9));
    world.push(Rc::new(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(checker))));
    for a in -10..10 {
        for b in -10..10 {
            let choose_material = rng.gen::<f32>();
            let center = Vector3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
            if (center - origin).magnitude() > 0.9 {
                if choose_material < 0.8 { // diffuse
                    world.push(Rc::new(
                        MovingSphere::new(center, center + Vector3::new(0.0, 0.5 * rng.gen::<f32>(), 0.0),0.0,1.0,0.2,
                                    Lambertian::new(ConstantTexture::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>())))));
                } else if choose_material < 0.95 { // metal
                    world.push(Rc::new(
                        Sphere::new(center, 0.2,
                                    Metal::new(Vector3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>())), 0.5 * rng.gen::<f32>()))));
                } else { // glass
                    world.push(Rc::new(
                        Sphere::new(center, 0.2, Dielectric::new(1.5))));
                }
            }
        }
    }
    world.push(Rc::new(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5))));
    world.push(Rc::new(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(ConstantTexture::new(0.4, 0.2, 0.1)))));
    world.push(Rc::new(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0))));
    Box::new(BVHNode::new(&mut world, 0.0, 1.0))
}

fn two_spheres() -> Box<Hitable> {
    let checker = CheckerTexture::new(ConstantTexture::new(0.2, 0.3, 0.1), ConstantTexture::new(0.9, 0.9, 0.9));
    let mut world = HitableList::default();
    world.push(Sphere::new(Vector3::new(0.0, -10.0, 0.0), 10.0, Lambertian::new(checker.clone())));
    world.push(Sphere::new(Vector3::new(0.0, 10.0, 0.0), 10.0, Lambertian::new(checker)));
    Box::new(world)
}

fn two_perlin_spheres() -> Box<Hitable> {
    let noise = NoiseTexture::new(4.0);
    let mut world = HitableList::default();
    world.push(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(noise.clone())));
    world.push(Sphere::new(Vector3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(noise)));
    Box::new(world)
}

fn earth() -> Box<Hitable> {
    let image = image::open("earthmap.png").expect("image not found").to_rgb();
    let (nx, ny) = image.dimensions();
    let data = image.into_raw();
    let texture = ImageTexture::new(data, nx, ny);
    let earth = Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0, Lambertian::new(texture));
    Box::new(earth)
}

fn simple_light() -> Box<Hitable> {
    let noise = NoiseTexture::new(4.0);
    let mut world = HitableList::default();
    world.push(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(noise.clone())));
    world.push(Sphere::new(Vector3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(noise)));
    world.push(Sphere::new(Vector3::new(0.0, 7.0, 0.0), 2.0, DiffuseLight::new(ConstantTexture::new(4.0, 4.0, 4.0))));
    world.push(AARect::new(Plane::XY, 3.0, 5.0, 1.0, 3.0, -2.0, DiffuseLight::new(ConstantTexture::new(4.0, 4.0, 4.0))));
    Box::new(world)
}

fn cornell_box() -> Box<Hitable> {
    let red = Lambertian::new(ConstantTexture::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(ConstantTexture::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(ConstantTexture::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(ConstantTexture::new(15.0, 15.0, 15.0));
    let mut world = HitableList::default();
    world.push(FlipNormals::new(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.push(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, red));
    world.push(AARect::new(Plane::ZX, 227.0, 332.0, 213.0, 343.0, 554.0, light));
    world.push(FlipNormals::new(AARect::new(Plane::ZX, 0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.push(AARect::new(Plane::ZX, 0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
    world.push(FlipNormals::new(AARect::new(Plane::XY, 0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.push(Cube::new(Vector3::new(130.0, 0.0, 65.0), Vector3::new(295.0, 165.0, 230.0), white.clone()));
    world.push(Cube::new(Vector3::new(265.0, 0.0, 295.0), Vector3::new(430.0, 330.0, 460.0), white));
    Box::new(world)
}

fn color(ray: &Ray, world: &Box<Hitable>, depth: i32) -> Vector3<f32> {
    if let Some(hit) = world.hit(ray, 0.001, f32::MAX) {
        let emitted = hit.material.emitted(hit.u, hit.v, &hit.p);
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
                return emitted + attenuation.zip_map(&color(&scattered, &world, depth+1), |l, r| l * r);
            }
        }
        emitted
    } else {
        Vector3::zeros()
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let nx = 800;
    let ny = 800;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let world = cornell_box();
    let look_from = Vector3::new(278.0, 278.0, -800.0);
    let look_at = Vector3::new(278.0, 278.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.0;
    let cam = Camera::new(
        look_from, look_at, Vector3::new(0.0, 1.0, 0.0),
        40.0, nx as f32 / ny as f32, aperture, focus_dist, 0.0, 1.0);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let ray = cam.get_ray(u, v);
                col += color(&ray, &world, 0);
            }
            col /= ns as f32;
            for c in col.iter_mut() { *c = nalgebra::clamp(c.sqrt(), 0.0, 1.0); }
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
