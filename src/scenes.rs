use std::f32;
use std::sync::Arc;

use rand::prelude::*;

use crate::camera::ViewInfo;
use crate::material::*;
use crate::texture::*;
use crate::*;

pub fn random(nx: i32, ny: i32) -> (Vec<Arc<dyn Hittable>>, Camera) {
    (
        {
            let n = 500;
            let mut scene: Vec<Arc<dyn Hittable>> = Vec::with_capacity(n + 1);

            let checker = CheckerTexture::new(
                ConstantTexture::new(0.2, 0.3, 0.1),
                ConstantTexture::new(0.9, 0.9, 0.9),
            );
            scene.push(Arc::new(Sphere::new(
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
                                scene.push(Arc::new(MovingSphere::new(
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
                                scene.push(Arc::new(Sphere::new(
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
                                scene.push(Arc::new(Sphere::new(
                                    &center,
                                    0.2,
                                    material::Dielectric::new(1.5),
                                )));
                            }
                        }
                    });
                }
            }

            scene.push(Arc::new(Sphere::new(
                &Vector3::new(0.0, 1.0, 0.0),
                1.0,
                material::Dielectric::new(1.5),
            )));

            scene.push(Arc::new(Sphere::new(
                &Vector3::new(-4.0, 1.0, 0.0),
                1.0,
                material::Lambertian::new(texture::ConstantTexture::new(0.4, 0.2, 0.1)),
            )));
            scene.push(Arc::new(Sphere::new(
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
                &ViewInfo::new(20.0, nx as f32 / ny as f32, aperture, focus_dist),
                0.0,
                1.0,
            )
        },
    )
}

pub fn two_spheres() -> Vec<Arc<dyn Hittable>> {
    let checker = CheckerTexture::new(
        ConstantTexture::new(0.2, 0.3, 0.1),
        ConstantTexture::new(0.9, 0.9, 0.9),
    );

    vec![
        Arc::new(Sphere::new(
            &Vector3::new(0.0, -10.0, 0.0),
            10.0,
            Lambertian::new(checker.clone()),
        )),
        Arc::new(Sphere::new(
            &Vector3::new(0.0, 10.0, 0.0),
            10.0,
            Lambertian::new(checker),
        )),
    ]
}

pub fn two_perlin_sphere(nx: u32, ny: u32) -> (Vec<Arc<dyn Hittable>>, Camera) {
    (
        {
            let noise = NoiseTexture::new(10.0);

            vec![
                Arc::new(Sphere::new(
                    &Vector3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    Lambertian::new(noise.clone()),
                )),
                Arc::new(Sphere::new(
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
                &ViewInfo::new(20.0, nx as f32 / ny as f32, aperture, focus_dist),
                0.0,
                1.0,
            )
        },
    )
}

pub fn earth() -> (Vec<Arc<dyn Hittable>>, Camera) {
    (
        vec![Arc::new(Sphere::new(
            &Vector3::new(0.0, 0.0, 0.0),
            1.0,
            Lambertian::new(ImageTexture::open("resource/earth.jpg")),
        ))],
        Camera::new(
            &Vector3::new(0.0, 0.0, 10.0),
            &Vector3::zero(),
            &Vector3::up(),
            &ViewInfo::new(
                2.0 * f32::atan(1.2 / 10.0) * 180.0 / f32::consts::PI,
                1200.0 / 800.0,
                0.0,
                10.0,
            ),
            0.0,
            0.0,
        ),
    )
}

pub fn earth_other_half() -> (Vec<Arc<dyn Hittable>>, Camera) {
    (
        vec![Arc::new(Sphere::new(
            &Vector3::new(0.0, 0.0, 0.0),
            1.0,
            Lambertian::new(ImageTexture::open("resource/earth.jpg")),
        ))],
        Camera::new(
            &Vector3::new(0.0, 0.0, -10.0),
            &Vector3::zero(),
            &Vector3::up(),
            &ViewInfo::new(
                2.0 * f32::atan(1.2 / 10.0) * 180.0 / f32::consts::PI,
                1200.0 / 800.0,
                0.0,
                10.0,
            ),
            0.0,
            0.0,
        ),
    )
}

pub fn simple_light(nx: i32, ny: i32) -> (Vec<Arc<dyn Hittable>>, Camera) {
    (
        {
            let noise = Arc::new(Lambertian::new(NoiseTexture::new(4.0)));
            let light = Arc::new(DiffuseLight::new(ConstantTexture::new(4.0, 4.0, 4.0)));
            let obj_list: Vec<Arc<dyn Hittable>> = vec![
                Arc::new(Sphere::new::<Lambertian, Arc<Lambertian>>(
                    &Vector3::new(0.0, -1000.0, 0.0),
                    1000.0,
                    noise.clone(),
                )),
                Arc::new(Sphere::new::<Lambertian, Arc<Lambertian>>(
                    &Vector3::new(0.0, 2.0, 0.0),
                    2.0,
                    noise,
                )),
                Arc::new(Sphere::new::<DiffuseLight, Arc<DiffuseLight>>(
                    &Vector3::new(0.0, 7.0, 0.0),
                    2.0,
                    light.clone(),
                )),
                Arc::new(XyRect::new::<DiffuseLight, Arc<DiffuseLight>>(
                    (3.0, 5.0),
                    (1.0, 3.0),
                    -2.0,
                    light,
                )),
            ];
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
                &ViewInfo::new(
                    2.0 * 180.0 / f32::consts::PI * f32::atan(5.0 / distance),
                    nx as f32 / ny as f32,
                    0.0,
                    distance,
                ),
                0.0,
                0.0,
            )
        },
    )
}

pub fn cornell_box(nx: i32, ny: i32) -> (Vec<Arc<dyn Hittable>>, Camera) {
    (
        {
            let red_material = Lambertian::new(ConstantTexture::new(0.65, 0.05, 0.05));
            let white_material = Arc::new(Lambertian::new(ConstantTexture::new(0.73, 0.73, 0.73)));
            let green_material = Lambertian::new(ConstantTexture::new(0.12, 0.45, 0.15));
            let light_material = DiffuseLight::new(ConstantTexture::new(15.0, 15.0, 15.0));

            let list: Vec<Arc<dyn Hittable>> = vec![
                Arc::new(FlipNormals::new(YzRect::new(
                    (0.0, 555.0),
                    (0.0, 555.0),
                    555.0,
                    green_material,
                ))),
                Arc::new(YzRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red_material)),
                Arc::new(ZxRect::new(
                    (227.0, 332.0),
                    (213.0, 343.0),
                    554.0,
                    light_material,
                )),
                Arc::new(FlipNormals::new(
                    ZxRect::new::<Lambertian, Arc<Lambertian>>(
                        (0.0, 555.0),
                        (0.0, 555.0),
                        555.0,
                        white_material.clone(),
                    ),
                )),
                Arc::new(ZxRect::new::<Lambertian, Arc<Lambertian>>(
                    (0.0, 555.0),
                    (0.0, 555.0),
                    0.0,
                    white_material.clone(),
                )),
                Arc::new(FlipNormals::new(
                    XyRect::new::<Lambertian, Arc<Lambertian>>(
                        (0.0, 555.0),
                        (0.0, 555.0),
                        555.0,
                        white_material,
                    ),
                )),
            ];
            list
        },
        {
            let look_from = Vector3::new(278.0, 278.0, -800.0);
            let look_at = Vector3::new(278.0, 278.0, 0.0);
            Camera::new(
                &look_from,
                &look_at,
                &Vector3::up(),
                &ViewInfo::new(40.0, nx as f32 / ny as f32, 0.0, 10.0),
                0.0,
                1.0,
            )
        },
    )
}
