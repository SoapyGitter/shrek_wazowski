use bevy::prelude::*;
use bevy::time::common_conditions::on_fixed_timer;
use bevy_rapier2d::math;
use std::time::Duration;
const SHREK_SPEED: f32 = 5.0;
const SHREK_JUMP_FORCE_LIMIT: f32 = 8.0;
const SHREK_JUMP_FORCE_ADDITION: f32 = 0.5;
#[derive(Component)]
struct Shrek {
    is_dead: bool,
    move_direction: Vec2,
    movement_speed: f32,
    jump_force: f32,
    grounded: bool,
    jumping_up: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input)
        .add_system(
            fixed_update
                .in_schedule(CoreSchedule::FixedUpdate)
                .run_if(on_fixed_timer(Duration::from_millis(20))),
        )
        .add_system(update)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: assets.load("game/background.png"),
            ..default()
        })
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            0.0, 0.0, 0.0,
        )));

    commands
        .spawn(SpriteBundle {
            texture: assets.load("game/bg.png"),
            ..default()
        })
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            0.0, 0.0, 2.0,
        )));
    commands
        .spawn((
            SpriteBundle {
                texture: assets.load("game/mike.png"),
                ..default()
            },
            Shrek {
                is_dead: false,
                move_direction: Vec2 { x: 0., y: 0. },
                movement_speed: SHREK_SPEED,
                jump_force: 0.,
                grounded: true,
                jumping_up: false,
            },
        ))
        .insert(TransformBundle::from(Transform {
            translation: Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            },
            scale: Vec3 {
                x: 1.,
                y: 1.,
                z: 1.,
            },
            rotation: Quat::IDENTITY,
        }));
}

fn update() {}

fn fixed_update(mut shrek_query: Query<(&mut Transform, &mut Shrek)>) {
    let (mut transform, mut shrek) = shrek_query.single_mut();

    if shrek.move_direction.x != 0.0 || shrek.move_direction.y != 0.0 {
        transform.translation.x += shrek.move_direction.x * shrek.movement_speed;
        transform.translation.y += shrek.move_direction.y * shrek.movement_speed;

        if shrek.move_direction.x > 0. {
            transform.scale.x = 1.;
        } else if shrek.move_direction.x < 0. {
            transform.scale.x = -1.;
        }
    }

    if !shrek.grounded && shrek.jumping_up {
        transform.translation.y += shrek.jump_force.powf(2.);
        shrek.jump_force -= SHREK_JUMP_FORCE_ADDITION;

        if shrek.jump_force <= 0. {
            shrek.jump_force = SHREK_JUMP_FORCE_ADDITION;
            shrek.jumping_up = false;
        }
    } else if !shrek.grounded && !shrek.jumping_up {
        transform.translation.y -= shrek.jump_force.powf(2.);
        shrek.jump_force += SHREK_JUMP_FORCE_ADDITION;

        if shrek.jump_force >= SHREK_JUMP_FORCE_LIMIT + SHREK_JUMP_FORCE_ADDITION {
            shrek.grounded = true;
        }
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut shrek_query: Query<&mut Shrek>) {
    let mut shrek = shrek_query.get_single_mut().unwrap();
    if keys.any_pressed([KeyCode::W, KeyCode::A, KeyCode::D, KeyCode::S]) {
        if(shrek.grounded){
            if keys.pressed(KeyCode::W) {
                shrek.move_direction.y = 1.;
            } else if keys.pressed(KeyCode::S) {
                shrek.move_direction.y = -1.;
            } else {
                shrek.move_direction.y = 0.;
            }
        }
        
        if keys.pressed(KeyCode::A) && !keys.pressed(KeyCode::D)
            || keys.just_pressed(KeyCode::A) && keys.pressed(KeyCode::D)
        {
            shrek.move_direction.x = -1.;
        } else if keys.pressed(KeyCode::D) && !keys.pressed(KeyCode::A)
            || keys.pressed(KeyCode::A) && keys.just_pressed(KeyCode::D)
        {
            shrek.move_direction.x = 1.;
        } else if !keys.pressed(KeyCode::D) && !keys.pressed(KeyCode::A) {
            shrek.move_direction.x = 0.;
        }
    } else {
        shrek.move_direction = Vec2 { x: 0., y: 0. };
    }

    if keys.just_pressed(KeyCode::Space) && shrek.grounded {
        shrek.grounded = false;
        shrek.jump_force = SHREK_JUMP_FORCE_LIMIT;
        shrek.jumping_up = true;
    }
}
