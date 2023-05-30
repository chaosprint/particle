use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        primitives::{quad, cube, sphere, sphere_radius},
        transform::{lookat_target, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable, make_sphere},
    prelude::*,
};

use components::particle_life;

const LIFE_TIME: f32 = 5.;
#[main]
pub async fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), vec3(5.0, 5.0, 2.0))
        .with(lookat_target(), vec3(0., 0., 0.))
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with(scale(), Vec3::ONE * 5.0)
        .spawn();

    run_async(async move{
        loop {
            sleep(0.1).await;
            for _ in 0..300 {
                let pos = vec3(random::<f32>(), random::<f32>(), random::<f32>()) *0.3;
                let size = random::<f32>() * 0.015;
                let id = Entity::new()
                .with_merge(make_transformable())
                .with_default(cube())
                // .with_merge(make_sphere())
                // .with(sphere_radius(), size)
                .with(scale(),  random::<Vec3>() * size * 2.0)
                .with(translation(), pos)
                .with(color(), vec4(1.0, 1.0, 1.0, 1.0))
                .with(particle_life(), 0.0)
                .spawn();
            }
        }
    });
    
    query(particle_life()).each_frame(move |particles|{
        for (particle, life) in particles {
            entity::mutate_component(particle, translation(), |xyz| {
                *xyz += vec3(
                    (random::<f32>() - 0.5) * 0.1, // Random movement in x
                    (random::<f32>() - 0.5) * 0.1, // Random movement in y
                    random::<f32>() * 0.2,
                );
            });
            if life >= LIFE_TIME {
                entity::despawn(particle);
            } else {
                // Update the life of the particle.
                entity::set_component(particle, particle_life(), life + 0.2);

                // Update the color of the particle.
                entity::mutate_component(particle, color(), |xyzw| {
                    *xyzw += vec4(0.00, -0.05, -0.1, -0.00); // Gradually change color to red
                });

                // Update the size of the particle.
                entity::mutate_component(particle, scale(), |size| {
                    *size *= 1.001;
                });
            }
        }
    });
}