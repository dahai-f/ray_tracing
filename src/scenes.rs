use std::f32;
use std::sync::Arc;

use rand::prelude::*;

use crate::material::*;
use crate::texture::*;
use crate::*;

pub fn random(nx: i32, ny: i32) -> (Vec<Box<Hittable>>, Camera) {
    (
        {
            let n = 500;
            let mut scene: Vec<Box<Hittable>> = Vec::with_capacity(n + 1);

            let checker = CheckerTexture::new(
                ConstantTexture::new(0.2, 0.3, 0.1),
                ConstantTexture::new(0.9, 0.9, 0.9),
            );
            scene.push(Box::new(Sphere::new(
                &Vector3::new(0.0, -1000.0, 0.0),
                1000.0,
                Lambertian::new(checker),
            )));
            for a in -11..11 {
                for b in -11..11 {
                    Random::with_rng(|rng| {
                        let choose_mat = rng.gen::<f32>();
                        let center = Vector3::new(
                            a as f32 + 0.9 * rng.gen::<f32>(),
                            0.2,
                            b as f32 + 0.9 * rng.gen::<f32>(),
                        );
                        if (center - Vector3::new(4.0, 0.2, 0.0)).squared_length() > 0.9 * 0.9 {
                            if choose_mat < 0.8 {
                                // diffuse
                                scene.push(Box::new(MovingSphere::new(
                                    &center,
                                    &(center + Vector3::new(0.0, 0.5 * rng.gen::<f32>(), 0.0)),
                                    0.0,
                                    1.0,
                                    0.2,
                                    Lambertian::new(texture::ConstantTexture::new(
                                        rng.gen::<f32>() * rng.gen::<f32>(),
                                        rng.gen::<f32>() * rng.gen::<f32>(),
                                        rng.gen::<f32>() * rng.gen::<f32>(),
                                    )),
                                )));
                            } else if choose_mat < 0.95 {
                                // metal
                                scene.push(Box::new(Sphere::new(
                                    &center,
                                    0.2,
                                    Metal::new(
                                        &Vector3::new(
                                            0.5 * (1.0 + rng.gen::<f32>()),
                                            0.5 * (1.0 + rng.gen::<f32>()),
                                            0.5 * (1.0 + rng.gen::<f32>()),
                                        ),
                                        0.5 * rng.gen::<f32>(),
                                    ),
                                )));
                            } else {
                                // dielectric
                                scene.push(Box::new(Sphere::new(
                                    &center,
                                    0.2,
                                    material::Dielectric::new(1.5),
                                )));
                            }
                        }
                    });
                }
            }

            scene.push(Box::new(Sphere::new(
                &Vector3::new(0.0, 1.0, 0.0),
                1.0,
                material::Dielectric::new(1.5),
            )));

            scene.push(Box::new(Sphere::new(
                &Vector3::new(-4.0, 1.0, 0.0),
                1.0,
                material::Lambertian::new(texture::ConstantTexture::new(0.4, 0.2, 0.1)),
            )));
            scene.push(Box::new(Sphere::new(
                &Vector3::new(4.0, 1.0, 0.0),
                1.0,
                material::Metal::new(&Vector3::new(0.7, 0.6, 0.5), 0.0),
            )));

            scene
        },
        {
            let look_from = Vector3::new(13.0, 2.0, 3.0);
            let look_at = Vector3::new(0.0, 0.0, 0.0);
            let focus_dist = 10.0;
            let aperture = 0.0;

            Camera::new(
                &look_from,
                &look_at,
                &Vector3::new(0.0, 1.0, 0.0),
                20.0,
                nx as f32 / ny as f32,
                aperture,
                focus_dist,
                0.0,
                1.0,
            )
        },
    )
}

pub fn two_spheres() -> Vec<Box<Hittable>> {
    let checker = CheckerTexture::new(
        ConstantTexture::new(0.2, 0.3, 0.1),
        ConstantTexture::new(0.9, 0.9, 0.9),
    );

    vec![
        Box::new(Sphere::new(
            &Vector3::new(0.0, -10.0, 0.0),
            10.0,
            Lambertian::new(checker.clone()),
        )),
        Box::new(Sphere::new(
            &Vector3::new(0.0, 10.0, 0.0),
            10.0,
            Lambertian::new(checker),
        )),
    ]
}

pub fn two_perlin_sphere(nx: u32, ny: u32) -> (Vec<Box<Hittable>>, Camera) {
    (
        {
            let noise = NoiseTexture::new(10.0);

            vec![
                Box::new(Sphere::new(
                    &Vector3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    Lambertian::new(noise.clone()),
                )),
                Box::new(Sphere::new(
                    &Vector3::new(0.0, 2.0, 0.0),
                    2.0,
                    Lambertian::new(noise),
                )),
            ]
        },
        {
            let look_from = Vector3::new(13.0, 2.0, 3.0);
            let look_at = Vector3::new(0.0, 0.0, 0.0);
            let focus_dist = 10.0;
            let aperture = 0.0;

            Camera::new(
                &look_from,
                &look_at,
                &Vector3::new(0.0, 1.0, 0.0),
                20.0,
                nx as f32 / ny as f32,
                aperture,
                focus_dist,
                0.0,
                1.0,
            )
        },
    )
}

pub fn earth() -> (Vec<Box<Hittable>>, Camera) {
    (
        vec![Box::new(Sphere::new(
            &Vector3::new(0.0, 0.0, 0.0),
            1.0,
            Lambertian::new(ImageTexture::open("resource/earth.jpg")),
        ))],
        Camera::new(
            &Vector3::new(0.0, 0.0, 10.0),
            &Vector3::zero(),
            &Vector3::up(),
            2.0 * f32::atan(1.2 / 10.0) * 180.0 / f32::consts::PI,
            1200.0 / 800.0,
            0.0,
            10.0,
            0.0,
            0.0,
        ),
    )
}

pub fn earth_other_half() -> (Vec<Box<Hittable>>, Camera) {
    (
        vec![Box::new(Sphere::new(
            &Vector3::new(0.0, 0.0, 0.0),
            1.0,
            Lambertian::new(ImageTexture::open("resource/earth.jpg")),
        ))],
        Camera::new(
            &Vector3::new(0.0, 0.0, -10.0),
            &Vector3::zero(),
            &Vector3::up(),
            2.0 * f32::atan(1.2 / 10.0) * 180.0 / f32::consts::PI,
            1200.0 / 800.0,
            0.0,
            10.0,
            0.0,
            0.0,
        ),
    )
}

pub fn simple_light(nx: i32, ny: i32) -> (Vec<Box<Hittable>>, Camera) {
    (
        {
            let noise = Arc::new(Lambertian::new(NoiseTexture::new(4.0)));
            let light = Arc::new(DiffuseLight::new(ConstantTexture::new(4.0, 4.0, 4.0)));
            let mut obj_list: Vec<Box<Hittable>> = Vec::with_capacity(4);
            obj_list.push(Box::new(Sphere::new::<Lambertian, Arc<Lambertian>>(
                &Vector3::new(0.0, -1000.0, 0.0),
                1000.0,
                noise.clone(),
            )));
            obj_list.push(Box::new(Sphere::new::<Lambertian, Arc<Lambertian>>(
                &Vector3::new(0.0, 2.0, 0.0),
                2.0,
                noise.clone(),
            )));
            obj_list.push(Box::new(Sphere::new::<DiffuseLight, Arc<DiffuseLight>>(
                &Vector3::new(0.0, 7.0, 0.0),
                2.0,
                light.clone(),
            )));
            obj_list.push(Box::new(XyRect::new::<DiffuseLight, Arc<DiffuseLight>>(
                (3.0, 5.0),
                (1.0, 3.0),
                -2.0,
                light.clone(),
            )));
            obj_list
        },
        {
            let look_from = Vector3::new(15.0, 3.0, 0.0);
            let look_at = Vector3::new(0.0, 2.0, 0.0);
            let distance = Vector3::distance(&look_from, &look_at);
            Camera::new(
                &look_from,
                &look_at,
                &Vector3::up(),
                2.0 * 180.0 / f32::consts::PI * f32::atan(5.0 / distance),
                nx as f32 / ny as f32,
                0.0,
                distance,
                0.0,
                0.0,
            )
        },
    )
}
