use bevy::prelude::*;
use rand::Rng;

pub const PLAYER_SPEED: f32 = 200.;
pub const MINIONS_SPEED: f32 = 75.;
pub const BULLET_SPEED: f32 = 250.;

pub const PLAYER_SIZE: f32 = 65.;
pub const MINIONS_SIZE: f32 = 50.;
pub const BULLET_SIZE: f32 = 10.;

#[derive(Resource)]
struct MyTimer(Timer);

#[derive(Component)]
struct ColorText;

pub struct MinionPlugin;
impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MyTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Update, add_ememies);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(MinionPlugin)
        .add_systems(
            Update,
            (
                shoot_bullets,
                sprite_movement,
                update_bullets,
                remove_bullets,
                update_enemies,
                remove_enemies,
                bullet_enemy_crash,
                enemies_player_crash,
            ),
        )
        .add_systems(Update, add_ememies)
        .run();
}

#[derive(Component)]
struct Player {
    health: i32,
    coord: (f32, f32),
}

struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Bullet {
    speed: f32,
    coord: (f32, f32),
}

#[derive(Component)]
struct Minions {
    speed: f32,
    coord: Position,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player_spaceship.png"),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),

                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
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
    let speed = PLAYER_SPEED * time.delta_seconds();
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
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                        ..default()
                    },
                    ..default()
                },
                Bullet {
                    speed: BULLET_SPEED,
                    coord: player.coord,
                },
            ));
        }
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/shoot.ogg"),
            ..default()
        });
    }
}

fn update_bullets(time: Res<Time>, mut bullets: Query<(&mut Bullet, &mut Transform)>) {
    let dt = time.delta_seconds();
    for (mut bullet, mut transform) in &mut bullets {
        transform.translation.y += bullet.speed * dt;
        bullet.coord = (transform.translation.x, transform.translation.y);
    }
}

fn remove_bullets(mut commands: Commands, bullets: Query<(Entity, &Bullet)>) {
    for (entity, bullet) in &mut bullets.iter() {
        if bullet.coord.1 > 300. {
            commands.entity(entity).despawn();
        }
    }
}

fn add_ememies(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemies: Query<(Entity, &Minions)>,
    mut timer: ResMut<MyTimer>,
) {
    timer.0.tick(time.delta());

    if timer.0.tick(time.delta()).just_finished() && enemies.iter().len() < 15 {
        let mut rng = rand::thread_rng();
        // Generate a random integer between 1 and 100
        let mut random_number: i32 = rng.gen_range(0..=600);

        if rng.gen_bool(1. / 2.) {
            random_number = -1 * random_number;
        }

        let coord = (random_number as f32, 400.);
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("minions.png"),
                transform: Transform {
                    translation: Vec3::new(coord.0, coord.1, 0.),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(MINIONS_SIZE, MINIONS_SIZE)),
                    ..default()
                },
                ..default()
            },
            Minions {
                speed: MINIONS_SPEED,
                coord: Position {
                    x: coord.0,
                    y: coord.1,
                },
            },
        ));
    }
}

fn update_enemies(time: Res<Time>, mut minions: Query<(&mut Minions, &mut Transform)>) {
    let dt = time.delta_seconds();
    for (mut minion, mut transform) in &mut minions {
        transform.translation.y -= minion.speed * dt;
        minion.coord.x = transform.translation.x;
        minion.coord.y = transform.translation.y;
    }
}

fn remove_enemies(mut commands: Commands, minions: Query<(Entity, &Minions)>) {
    for (entity, minion) in &mut minions.iter() {
        if minion.coord.y < -400. {
            commands.entity(entity).despawn();
        }
    }
}

fn bullet_enemy_crash(
    minions: Query<(Entity, &Transform), With<Minions>>,
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    asset_server: Res<AssetServer>,
) {
    for (minion_entity, minion_transform) in &mut minions.iter() {
        let minimum_distance = MINIONS_SIZE / 2. + BULLET_SIZE / 2.;

        for (bullet_entity, bullet_transform) in &mut bullets.iter() {
            let distance = minion_transform
                .translation
                .distance(bullet_transform.translation);

            if distance < minimum_distance {
                commands.entity(minion_entity).despawn();
                commands.entity(bullet_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/minion_kill.ogg"),
                    ..default()
                });
            }
        }
    }
}

fn enemies_player_crash(
    minions: Query<(Entity, &Transform), With<Minions>>,
    mut commands: Commands,
    players: Query<(Entity, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    for (minion_entity, minion_transform) in &mut minions.iter() {
        let minimum_distance = MINIONS_SIZE / 2. + PLAYER_SIZE / 2.;

        for (player_entity, player_transform) in &mut players.iter() {
            let distance = minion_transform
                .translation
                .distance(player_transform.translation);

            if distance < minimum_distance {
                commands.entity(minion_entity).despawn();
                commands.entity(player_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/game_over.ogg"),
                    ..default()
                });

                // wanna spwan big text saying Game Over

                commands.spawn((
                    TextBundle::from_section(
                        "GAME OVER",
                        TextStyle {
                            font: asset_server.load("fonts/RubikBrokenFax-Regular.ttf"),
                            font_size: 100.0,
                            ..default()
                        },
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(0.0),
                        right: Val::Px(0.0),
                        ..default()
                    }),
                    ColorText,
                ));
            }
        }
    }
}
