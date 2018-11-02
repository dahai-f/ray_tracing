use crate::*;
use rand::prelude::*;

pub fn random() -> Vec<Box<Hittable>> {
    use crate::texture::*;

    let n = 500;
    let mut scene: Vec<Box<Hittable>> = Vec::with_capacity(n + 1);

    let checker = CheckerTexture::new(
        ConstantTexture::new(0.2, 0.3, 0.1),
        ConstantTexture::new(0.9, 0.9, 0.9),
    );
    scene.push(Box::new(Sphere::new(
        &Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(material::Lambertian::new(checker)),
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
                            Box::new(material::Lambertian::new(texture::ConstantTexture::new(
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                            ))),
                        )));
                    } else if choose_mat < 0.95 {
                        // metal
                        scene.push(Box::new(Sphere::new(
                            &center,
                            0.2,
                            Box::new(material::Metal::new(
                                &Vector3::new(
                                    0.5 * (1.0 + rng.gen::<f32>()),
                                    0.5 * (1.0 + rng.gen::<f32>()),
                                    0.5 * (1.0 + rng.gen::<f32>()),
                                ),
                                0.5 * rng.gen::<f32>(),
                            )),
                        )));
                    } else {
                        // dielectric
                        scene.push(Box::new(Sphere::new(
                            &center,
                            0.2,
                            Box::new(material::Dielectric::new(1.5)),
                        )));
                    }
                }
            });
        }
    }

    scene.push(Box::new(Sphere::new(
        &Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(material::Dielectric::new(1.5)),
    )));

    scene.push(Box::new(Sphere::new(
        &Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(material::Lambertian::new(texture::ConstantTexture::new(
            0.4, 0.2, 0.1,
        ))),
    )));
    scene.push(Box::new(Sphere::new(
        &Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(material::Metal::new(&Vector3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    scene
}

pub fn two_spheres() -> Vec<Box<Hittable>> {
    use crate::texture::*;
    let checker = CheckerTexture::new(
        ConstantTexture::new(0.2, 0.3, 0.1),
        ConstantTexture::new(0.9, 0.9, 0.9),
    );

    vec![
        Box::new(Sphere::new(
            &Vector3::new(0.0, -10.0, 0.0),
            10.0,
            Box::new(crate::material::Lambertian::new(checker.clone())),
        )),
        Box::new(Sphere::new(
            &Vector3::new(0.0, 10.0, 0.0),
            10.0,
            Box::new(crate::material::Lambertian::new(checker)),
        )),
    ]
}
