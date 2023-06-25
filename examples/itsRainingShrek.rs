use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_fixed_timer};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

const SHREK_SIZE: f32 = 40.;
const SHREK_SIZE_DEVIDER: f32 = 15.0;
const SHREK_SPAWN_X_NORMALIZED_RANGE: f32 = 800.;
const DEVIDED_SHREK_X_POSITION: i32 = (SHREK_SPAWN_X_NORMALIZED_RANGE / SHREK_SIZE) as i32;

#[derive(Component)]
pub struct EncounterTimer{
    timer: Timer
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(
            slow_spawn
                .in_schedule(CoreSchedule::FixedUpdate)
                .run_if(on_fixed_timer(Duration::from_millis(150))),
        )
        .add_system(shrek_death_deteciton)
        .run();
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

fn slow_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    let shrek_x_pos = (rng.gen_range(0..DEVIDED_SHREK_X_POSITION) as f32) * SHREK_SIZE
        - SHREK_SPAWN_X_NORMALIZED_RANGE / 2.;

    println!("{}", shrek_x_pos);

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(SpriteBundle {
            texture: asset_server.load("mike.png"),
            ..default()
        })
        .insert(Collider::ball(300.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform {
            translation: Vec3 {
                x: shrek_x_pos,
                y: 300.0,
                z: 0.0,
            },
            rotation: Quat::IDENTITY,
            scale: Vec3 {
                x: 1. / SHREK_SIZE_DEVIDER,
                y: 1. / SHREK_SIZE_DEVIDER,
                z: 1.,
            },
        }))
        .insert(Velocity {
            linvel: Vec2::new(0.0, -200.0),
            angvel: 0.,
        })
        .insert(GravityScale(1.))
        .insert(EncounterTimer{ timer: Timer::from_seconds(5., TimerMode::Once)});
}

fn shrek_death_deteciton(
    mut commands: Commands,
    mut shreki: Query<(&mut EncounterTimer, Entity)>, time: Res<Time>){
    for (mut timer, entity) in &mut shreki {
        timer.timer.tick(time.delta());

        if timer.timer.just_finished(){
            commands.entity(entity).despawn()
        }

    }
}