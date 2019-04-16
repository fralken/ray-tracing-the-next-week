mod ray;
mod hitable;
mod medium;
mod texture;
mod perlin;
mod material;
mod sphere;
mod rect;
mod cube;
mod translate;
mod rotate;
mod camera;
mod aabb;
mod bvh;

use std::f32;
use std::sync::Arc;
use nalgebra::Vector3;
use rand::Rng;
use image;
use rayon::prelude::*;
use crate::ray::Ray;
use crate::texture::{ConstantTexture, CheckerTexture, NoiseTexture, ImageTexture};
use crate::material::{Lambertian, Metal, Dielectric, DiffuseLight};
use crate::hitable::{Hitable, HitableList, FlipNormals};
use crate::medium::ConstantMedium;
use crate::sphere::{Sphere, MovingSphere};
use crate::rect::{AARect, Plane};
use crate::cube::Cube;
use crate::translate::Translate;
use crate::rotate::{Rotate, Axis};
use crate::camera::Camera;
use crate::bvh::BVHNode;

fn random_scene() -> Box<Hitable> {
    let mut rng = rand::thread_rng();
    let origin = Vector3::new(4.0, 0.2, 0.0);
    let mut world: Vec<Arc<Hitable>> = Vec::new();
    let checker = CheckerTexture::new(ConstantTexture::new(0.2, 0.3, 0.1), ConstantTexture::new(0.9, 0.9, 0.9));
    world.push(Arc::new(Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(checker))));
    for a in -10..10 {
        for b in -10..10 {
            let choose_material = rng.gen::<f32>();
            let center = Vector3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
            if (center - origin).magnitude() > 0.9 {
                if choose_material < 0.8 { // diffuse
                    world.push(Arc::new(
                        MovingSphere::new(center, center + Vector3::new(0.0, 0.5 * rng.gen::<f32>(), 0.0),0.0,1.0,0.2,
                                    Lambertian::new(ConstantTexture::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>())))));
                } else if choose_material < 0.95 { // metal
                    world.push(Arc::new(
                        Sphere::new(center, 0.2,
                                    Metal::new(Vector3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>())), 0.5 * rng.gen::<f32>()))));
                } else { // glass
                    world.push(Arc::new(
                        Sphere::new(center, 0.2, Dielectric::new(1.5))));
                }
            }
        }
    }
    world.push(Arc::new(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5))));
    world.push(Arc::new(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(ConstantTexture::new(0.4, 0.2, 0.1)))));
    world.push(Arc::new(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0))));
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
    world.push(
        Translate::new(
            Rotate::new(Axis::Y,
                        Cube::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(165.0, 165.0, 165.0), white.clone()),
                        -18.0),
            Vector3::new(130.0, 0.0, 65.0)));
    world.push(
        Translate::new(
            Rotate::new(Axis::Y,
                        Cube::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(165.0, 330.0, 165.0), white),
                        15.0),
            Vector3::new(265.0, 0.0, 295.0)));
    Box::new(world)
}

fn cornell_smoke() -> Box<Hitable> {
    let red = Lambertian::new(ConstantTexture::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(ConstantTexture::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(ConstantTexture::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(ConstantTexture::new(7.0, 7.0, 7.0));
    let mut world = HitableList::default();
    world.push(FlipNormals::new(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.push(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, red));
    world.push(AARect::new(Plane::ZX, 127.0, 432.0, 113.0, 443.0, 554.0, light));
    world.push(FlipNormals::new(AARect::new(Plane::ZX, 0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    world.push(AARect::new(Plane::ZX, 0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
    world.push(FlipNormals::new(AARect::new(Plane::XY, 0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    let box1 =
        Translate::new(
            Rotate::new(Axis::Y,
                        Cube::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(165.0, 165.0, 165.0), white.clone()),
                        -18.0),
            Vector3::new(130.0, 0.0, 65.0));
    let box2 =
        Translate::new(
            Rotate::new(Axis::Y,
                        Cube::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(165.0, 330.0, 165.0), white),
                        15.0),
            Vector3::new(265.0, 0.0, 295.0));
    world.push(ConstantMedium::new(box1, 0.01, ConstantTexture::new(1.0, 1.0, 1.0)));
    world.push(ConstantMedium::new(box2, 0.01, ConstantTexture::new(0.0, 0.0, 0.0)));
    Box::new(world)
}

fn final_scene() -> Box<Hitable> {
    let mut rng = rand::thread_rng();
    let white = Lambertian::new(ConstantTexture::new(0.73, 0.73, 0.73));
    let ground = Lambertian::new(ConstantTexture::new(0.48, 0.83, 0.53));
    let mut world = HitableList::default();
    let mut box_list1: Vec<Arc<Hitable>> = Vec::new();
    let nb = 20;
    for i in 0..nb {
        for j in 0..20 {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (rng.gen::<f32>() + 0.01);
            let z1 = z0 + w;
            box_list1.push(Arc::new(Cube::new(Vector3::new(x0, y0, z0), Vector3::new(x1, y1, z1), ground.clone())));
        }
    }
    world.push(BVHNode::new(&mut box_list1, 0.0, 1.0));
    let light = DiffuseLight::new(ConstantTexture::new(7.0, 7.0, 7.0));
    world.push(AARect::new(Plane::ZX, 147.0, 412.0, 123.0, 423.0, 554.0, light));
    let center = Vector3::new(400.0, 400.0, 200.0);
    world.push(MovingSphere::new(center, center + Vector3::new(30.0, 0.0, 0.0), 0.0, 1.0, 50.0, Lambertian::new(ConstantTexture::new(0.7, 0.3, 0.1))));
    world.push(Sphere::new(Vector3::new(260.0, 150.0, 45.0), 50.0, Dielectric::new(1.5)));
    world.push(Sphere::new(Vector3::new(0.0, 150.0, 145.0), 50.0, Metal::new(Vector3::new(0.8, 0.8, 0.9), 10.0)));
    let boundary = Sphere::new(Vector3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    world.push(boundary.clone());
    world.push(ConstantMedium::new(boundary, 0.2, ConstantTexture::new(0.2, 0.4, 0.9)));
    let boundary = Sphere::new(Vector3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    world.push(ConstantMedium::new(boundary, 0.0001, ConstantTexture::new(1.0, 1.0, 1.0)));
    let image = image::open("earthmap.png").expect("image not found").to_rgb();
    let (nx, ny) = image.dimensions();
    let data = image.into_raw();
    let texture = ImageTexture::new(data, nx, ny);
    world.push(Sphere::new(Vector3::new(400.0, 200.0, 400.0), 100.0, Lambertian::new(texture)));
    world.push(Sphere::new(Vector3::new(220.0, 280.0, 300.0), 80.0, Lambertian::new(NoiseTexture::new(0.1))));
    let mut box_list2: Vec<Arc<Hitable>> = Vec::new();
    let ns = 1000;
    for _ in 0..ns {
        box_list2.push(Arc::new(Sphere::new(Vector3::new(165.0 * rng.gen::<f32>(), 165.0 * rng.gen::<f32>(), 165.0 * rng.gen::<f32>()), 10.0, white.clone())));
    }
    world.push(
        Translate::new(
            Rotate::new(Axis::Y,
                        BVHNode::new(&mut box_list2, 0.0, 0.1),
                        15.0),
            Vector3::new(-100.0, 270.0, 395.0))
    );
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
    let nx = 800;
    let ny = 800;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let world = final_scene();
    let look_from = Vector3::new(478.0, 278.0, -600.0);
    let look_at = Vector3::new(278.0, 278.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.0;
    let cam = Camera::new(
        look_from, look_at, Vector3::new(0.0, 1.0, 0.0),
        40.0, nx as f32 / ny as f32, aperture, focus_dist, 0.0, 1.0);
    let image =
        (0..ny).into_par_iter().rev()
            .flat_map(|y|
                (0..nx).flat_map(|x| {
                    let col: Vector3<f32> = (0..ns).map(|_| {
                        let mut rng = rand::thread_rng();
                        let u = (x as f32 + rng.gen::<f32>()) / nx as f32;
                        let v = (y as f32 + rng.gen::<f32>()) / ny as f32;
                        let ray = cam.get_ray(u, v);
                        color(&ray, &world, 0)
                    }).sum();
                    col.iter().map(|c|
                        (255.99 * (c / ns as f32).sqrt().max(0.0).min(1.0)) as u8
                    ).collect::<Vec<u8>>()
                }).collect::<Vec<u8>>()
            ).collect::<Vec<u8>>();
    for col in image.chunks(3) {
       println!("{} {} {}", col[0], col[1], col[2]);
    }
}
