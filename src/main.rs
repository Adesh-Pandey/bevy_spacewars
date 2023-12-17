//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                shoot_bullets,
                sprite_movement,
                update_bullets,
                remove_bullets,
            ),
        )
        .run();
}

#[derive(Component)]
struct Player {
    health: i32,
    coord: (f32, f32),
}

#[derive(Component)]
struct Bullet {
    speed: f32,
    coord: (f32, f32),
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player_spaceship.png"),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(0.2, 0.2, 0.),
                ..default()
            },

            ..default()
        },
        Player {
            health: 100,
            coord: (0., 0.),
        },
    ));
}

fn sprite_movement(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Player, &mut Transform)>,
) {
    let speed = 100. * time.delta_seconds();
    for (mut player, mut transform) in &mut sprite_position {
        if keyboard.pressed(KeyCode::Up) && transform.translation.y < 300. {
            transform.translation.y += speed;
        } else if keyboard.pressed(KeyCode::Down) && transform.translation.y > -300. {
            transform.translation.y -= speed;
        } else if keyboard.pressed(KeyCode::Right) && transform.translation.x < 600. {
            transform.translation.x += speed;
        } else if keyboard.pressed(KeyCode::Left) && transform.translation.x > -600. {
            transform.translation.x -= speed;
        }

        player.coord = (transform.translation.x, transform.translation.y);
    }
}

fn shoot_bullets(
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    player_query: Query<&Player>,
    asset_server: Res<AssetServer>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for player in &mut player_query.iter() {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("bullet.png"),
                    transform: Transform {
                        translation: Vec3::new(player.coord.0, player.coord.1, 0.),
                        scale: Vec3::new(0.08, 0.08, 0.),
                        ..default()
                    },

                    ..default()
                },
                Bullet {
                    speed: 100.,
                    coord: player.coord,
                },
            ));
        }
    }
}

fn update_bullets(time: Res<Time>, mut bullets: Query<(&mut Bullet, &mut Transform)>) {
    let dt = time.delta_seconds();
    for (mut bullet, mut transform) in &mut bullets {
        transform.translation.y += bullet.speed * dt;
        bullet.coord = (transform.translation.x, transform.translation.y);
    }
    println!("bullets: {}", bullets.iter().len());
}

fn remove_bullets(mut commands: Commands, bullets: Query<(Entity, &Bullet)>) {
    for (entity, bullet) in &mut bullets.iter() {
        if bullet.coord.1 > 300. {
            commands.entity(entity).despawn();
        }
    }
}
