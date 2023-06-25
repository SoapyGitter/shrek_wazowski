use bevy::prelude::*;
use bevy::time::common_conditions::on_fixed_timer;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

const RADIUS: f32 = 700.0;
const MAX_ANGLE: f32 = 360.;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(
            slow_spawn.in_schedule(CoreSchedule::FixedUpdate)
                .run_if(on_fixed_timer(Duration::from_millis(2000))),
        )
        .add_system(sprite_movement)
        .run();
}

/// player component
#[derive(Component)]
struct Shrek {
    angle: f32,
    is_ready_to_spin: bool,
    is_ready_to_rotate: bool,
    counter: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

}

fn slow_spawn(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
    println!("{}", time.elapsed_seconds());
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("mike.png"),
                ..default()
            },
            Shrek {
                angle: 0.,
                is_ready_to_spin: false,
                counter: 0.001,
                is_ready_to_rotate: false,
            },
        ))
        .insert(TransformBundle::from(Transform {
            translation: Vec3 {
                x: (RADIUS - time.elapsed_seconds()).sin() * RADIUS,
                y: (RADIUS - time.elapsed_seconds()).cos() * RADIUS,
                z: 0.,
            },
            scale: Vec3 {
                x: 0.3,
                y: 0.3,
                z: 1.,
            },
            rotation: Quat::IDENTITY,
        }))
        .insert(Velocity {
            linvel: Vec2::new(1.0, 2.0),
            angvel: 0.,
        })
        .insert(GravityScale(0.1))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled());
}

// fn translate(mut from: Vec3, to: Vec3, force: f32, min_distance: f32){
//     while from.distance(to) < min_distance {
//         from.x += force;
//         from.y += force;
//     }
// }

fn sprite_movement(mut sprite_position: Query<(&mut Transform, &mut Shrek)>, time: Res<Time>) {
    for (mut transform, mut shrek) in &mut sprite_position {
        let transition_y = shrek.angle.cos() * RADIUS;
        let transition_x = shrek.angle.sin() * RADIUS;

        let to: Vec3 = Vec3 {
            x: transition_x,
            y: transition_y,
            z: 0.,
        };
        let force: f32 = 100.;

        if shrek.is_ready_to_spin && transform.translation.distance(to) > 10. {
            transform.translation.x += (to.x - transform.translation.x) / force;
            transform.translation.y += (to.y - transform.translation.y) / force;
        }

        shrek.angle = if shrek.angle >= MAX_ANGLE {
            0.
        } else {
            shrek.angle + 0.01
        };

        if shrek.is_ready_to_spin {
            transform.rotate_z(shrek.angle);
        }

        if !shrek.is_ready_to_spin {
            shrek.counter += 0.01;
        }

        if shrek.counter >= 2. {
            shrek.is_ready_to_rotate = true;
        }

        if shrek.counter >= 4. {
            shrek.is_ready_to_spin = true;
        }
    }
}
