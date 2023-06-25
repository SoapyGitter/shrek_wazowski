use std::{time::Duration};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

const SHREK_SIZE_DEVIDER: f32 = 50.0;

#[derive(Component)]
pub struct EncounterTimer {
    timer: Timer,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(shreksplosion.run_if(run_once_in(Duration::from_millis(2000))))
        .add_system(shrek_death_deteciton)
        .run();
}

fn run_once_in(duration: Duration) -> impl FnMut(Res<Time>) -> bool + Clone {
    let mut timer = Timer::new(duration, TimerMode::Once);
    move |time: Res<Time>| {
        timer.tick(time.delta());
        timer.just_finished()
    }
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    let platform_angle_rad: f32 = 2.;
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Quat::from_axis_angle(Vec3::Z, platform_angle_rad.to_radians()),
            scale: Vec3 {
                x: 1.,
                y: 1.,
                z: 1.,
            },
        }));
}

fn get_img_name(num: u8) -> String {
    if num == 0{
        return "mike.png".to_string();
    }else if num == 1 {
        return "wasowski.png".to_string();
    }
    "baby.png".to_string()
}

fn shreksplosion(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    let mut i = 0;
    loop {
        let x_pos = (rng.gen_range(0..40) - 20) as f32;
        let y_pos = (rng.gen_range(0..40) - 20) as f32;
        if i <= 100 {
            commands
                .spawn(RigidBody::Dynamic)
                .insert(SpriteBundle {
                    texture: asset_server.load(get_img_name(rng.gen_range(0..4))),
                    ..default()
                })
                .insert(TransformBundle::from(Transform {
                    translation: Vec3 {
                        x: x_pos,
                        y: y_pos,
                        z: 0.0,
                    },
                    rotation: Quat::IDENTITY,
                    scale: Vec3 {
                        x: 1. / SHREK_SIZE_DEVIDER,
                        y: 1. / SHREK_SIZE_DEVIDER,
                        z: 1.,
                    },
                }))
                .insert(Collider::ball(5.0))
                .insert(Restitution::coefficient(0.7))
                .insert(GravityScale(0.))
                .insert(Velocity {
                    linvel: Vec2::new(x_pos , y_pos ),
                    angvel: 0.,
                })
                .insert(EncounterTimer {
                    timer: Timer::from_seconds(20., TimerMode::Once),
                });
            i += 1;
        } else {
            break;
        }
    }
}

fn shrek_death_deteciton(
    mut commands: Commands,
    mut shreki: Query<(&mut EncounterTimer, Entity, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut timer, entity, mut transform) in &mut shreki {
        timer.timer.tick(time.delta());
        transform.scale.x += time.delta_seconds() / 100.;
        transform.scale.y += time.delta_seconds() / 100.;

        if timer.timer.just_finished() {
            commands.entity(entity).despawn()
        }
    }
}
