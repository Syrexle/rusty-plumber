use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = azzlePlaySound)]
    fn azzle_play_sound(name: &str);
}
use bevy::window::{PresentMode, WindowResolution};

const PLAYER_SIZE: Vec2 = Vec2::new(34.0, 44.0);
const GRAVITY: f32 = -1850.0;
const MOVE_SPEED: f32 = 370.0;
const ACCEL: f32 = 2600.0;
const FRICTION: f32 = 2100.0;
const JUMP_SPEED: f32 = 720.0;
const APPLE_BANK_VALUE: u32 = 10;
const POWERUP_COST: u32 = 2;
const LEVEL_COUNT: usize = 2;

#[derive(Component)]
struct Player;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider {
    size: Vec2,
}

#[derive(Component)]
struct Platform;

#[derive(Component)]
struct Collectible;

#[derive(Component)]
struct Shop;

#[derive(Component)]
struct Hazard;

#[derive(Component)]
struct Goal;

#[derive(Component)]
struct Checkpoint {
    position: Vec3,
}

#[derive(Component)]
struct Enemy {
    left: f32,
    right: f32,
    speed: f32,
}

#[derive(Component)]
struct LevelEntity;

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct Banner;

#[derive(Resource)]
struct Game {
    level: usize,
    lives: i32,
    score: u32,
    bank_coins: u32,
    frog_coins: u32,
    speed_boots: bool,
    spring_legs: bool,
    shield_charm: bool,
    shop_open: bool,
    shop_message: String,
    checkpoint: Vec3,
    mode: GameMode,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum GameMode {
    Playing,
    Won,
    Lost,
}

#[derive(Clone, Copy)]
struct RectSpec {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Clone, Copy)]
struct EnemySpec {
    x: f32,
    y: f32,
    left: f32,
    right: f32,
    speed: f32,
}

#[derive(Clone, Copy)]
struct LevelSpec {
    spawn: Vec3,
    platforms: &'static [RectSpec],
    coins: &'static [(f32, f32)],
    enemies: &'static [EnemySpec],
    hazards: &'static [RectSpec],
    checkpoint: Vec3,
    goal: Vec3,
    shop: Vec3,
    width: f32,
}

const LEVELS: [LevelSpec; LEVEL_COUNT] = [
    LevelSpec {
        spawn: Vec3::new(-530.0, -180.0, 10.0),
        platforms: &[
            RectSpec {
                x: 0.0,
                y: -250.0,
                w: 1200.0,
                h: 48.0,
            },
            RectSpec {
                x: -230.0,
                y: -115.0,
                w: 150.0,
                h: 30.0,
            },
            RectSpec {
                x: 15.0,
                y: 20.0,
                w: 170.0,
                h: 30.0,
            },
            RectSpec {
                x: 290.0,
                y: -65.0,
                w: 160.0,
                h: 30.0,
            },
            RectSpec {
                x: 485.0,
                y: 80.0,
                w: 150.0,
                h: 30.0,
            },
        ],
        coins: &[
            (-310.0, -70.0),
            (-230.0, -65.0),
            (-15.0, 70.0),
            (55.0, 70.0),
            (285.0, -15.0),
            (485.0, 130.0),
            (610.0, -175.0),
        ],
        enemies: &[
            EnemySpec {
                x: 30.0,
                y: -205.0,
                left: -105.0,
                right: 200.0,
                speed: 95.0,
            },
            EnemySpec {
                x: 385.0,
                y: -205.0,
                left: 290.0,
                right: 520.0,
                speed: 130.0,
            },
        ],
        hazards: &[
            RectSpec {
                x: -360.0,
                y: -212.0,
                w: 70.0,
                h: 28.0,
            },
            RectSpec {
                x: 205.0,
                y: -212.0,
                w: 75.0,
                h: 28.0,
            },
        ],
        checkpoint: Vec3::new(185.0, -190.0, 10.0),
        goal: Vec3::new(665.0, -180.0, 10.0),
        shop: Vec3::new(-455.0, -182.0, 10.0),
        width: 1200.0,
    },
    LevelSpec {
        spawn: Vec3::new(-520.0, -80.0, 10.0),
        platforms: &[
            RectSpec {
                x: -270.0,
                y: -250.0,
                w: 720.0,
                h: 48.0,
            },
            RectSpec {
                x: 470.0,
                y: -250.0,
                w: 360.0,
                h: 48.0,
            },
            RectSpec {
                x: -445.0,
                y: -70.0,
                w: 135.0,
                h: 30.0,
            },
            RectSpec {
                x: -165.0,
                y: 25.0,
                w: 130.0,
                h: 30.0,
            },
            RectSpec {
                x: 95.0,
                y: -50.0,
                w: 150.0,
                h: 30.0,
            },
            RectSpec {
                x: 330.0,
                y: 65.0,
                w: 150.0,
                h: 30.0,
            },
            RectSpec {
                x: 565.0,
                y: -35.0,
                w: 150.0,
                h: 30.0,
            },
        ],
        coins: &[
            (-445.0, -20.0),
            (-165.0, 75.0),
            (-95.0, 75.0),
            (95.0, 0.0),
            (330.0, 115.0),
            (565.0, 15.0),
            (665.0, -175.0),
        ],
        enemies: &[
            EnemySpec {
                x: -130.0,
                y: -205.0,
                left: -330.0,
                right: 40.0,
                speed: 120.0,
            },
            EnemySpec {
                x: 420.0,
                y: -205.0,
                left: 320.0,
                right: 610.0,
                speed: 145.0,
            },
            EnemySpec {
                x: 92.0,
                y: -5.0,
                left: 35.0,
                right: 155.0,
                speed: 70.0,
            },
        ],
        hazards: &[
            RectSpec {
                x: 205.0,
                y: -240.0,
                w: 170.0,
                h: 38.0,
            },
            RectSpec {
                x: -20.0,
                y: -212.0,
                w: 70.0,
                h: 28.0,
            },
        ],
        checkpoint: Vec3::new(310.0, 115.0, 10.0),
        goal: Vec3::new(715.0, -180.0, 10.0),
        shop: Vec3::new(-570.0, -182.0, 10.0),
        width: 1350.0,
    },
];

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.08, 0.1, 0.19)))
        .insert_resource(Game {
            level: 0,
            lives: 3,
            score: 0,
            bank_coins: 0,
            frog_coins: 0,
            speed_boots: false,
            spring_legs: false,
            shield_charm: false,
            shop_open: false,
            shop_message: "Find the piggy shop. E opens it.".to_string(),
            checkpoint: LEVELS[0].spawn,
            mode: GameMode::Playing,
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rusty Plumber".into(),
                        resolution: WindowResolution::new(960.0, 540.0),
                        canvas: Some("#bevy-canvas".to_string()),
                        fit_canvas_to_parent: true,
                        present_mode: PresentMode::AutoVsync,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                restart_input,
                player_input,
                apply_velocity,
                enemy_ai,
                resolve_player_platforms.after(apply_velocity),
                collect_items,
                shop_interaction,
                enemy_player_contact,
                hazard_goal_checkpoint,
                camera_follow,
                update_hud,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, game: Res<Game>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 28.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(14.0),
            top: Val::Px(10.0),
            ..default()
        }),
        Hud,
    ));
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 36.0,
                color: Color::srgb(1.0, 0.92, 0.45),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Percent(25.0),
            top: Val::Percent(42.0),
            ..default()
        }),
        Banner,
    ));
    spawn_level(&mut commands, &game, &asset_server);
}

fn spawn_level(commands: &mut Commands, game: &Game, asset_server: &AssetServer) {
    let level = LEVELS[game.level];
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pixel_adventure/derived/background_blue.png"),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(level.width + 500.0, 900.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -20.0),
            ..default()
        },
        LevelEntity,
    ));
    for i in 0..18 {
        let x = -650.0 + i as f32 * 95.0;
        let y = 185.0 + (i % 3) as f32 * 25.0;
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("pixel_adventure/Free/Other/Dust Particle.png"),
                sprite: Sprite {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.35),
                    custom_size: Some(Vec2::new(58.0, 18.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, -15.0),
                ..default()
            },
            LevelEntity,
        ));
    }
    for p in level.platforms {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("pixel_adventure/derived/terrain_tile.png"),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(p.w, p.h)),
                    ..default()
                },
                transform: Transform::from_xyz(p.x, p.y, 0.0),
                ..default()
            },
            Platform,
            Collider {
                size: Vec2::new(p.w, p.h),
            },
            LevelEntity,
        ));
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("pixel_adventure/derived/terrain_grass.png"),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(p.w, 10.0)),
                    ..default()
                },
                transform: Transform::from_xyz(p.x, p.y + p.h / 2.0 + 5.0, 1.0),
                ..default()
            },
            LevelEntity,
        ));
    }
    for h in level.hazards {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("pixel_adventure/derived/hazard_spike.png"),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(h.w, h.h)),
                    ..default()
                },
                transform: Transform::from_xyz(h.x, h.y, 2.0),
                ..default()
            },
            Hazard,
            Collider {
                size: Vec2::new(h.w, h.h),
            },
            LevelEntity,
        ));
    }
    for (x, y) in level.coins {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("pixel_adventure/derived/fruit.png"),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(28.0, 28.0)),
                    ..default()
                },
                transform: Transform::from_xyz(*x, *y, 4.0),
                ..default()
            },
            Collectible,
            Collider {
                size: Vec2::new(22.0, 22.0),
            },
            LevelEntity,
        ));
    }
    for e in level.enemies {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("pixel_adventure/derived/enemy_rock.png"),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(42.0, 42.0)),
                    ..default()
                },
                transform: Transform::from_xyz(e.x, e.y, 5.0),
                ..default()
            },
            Enemy {
                left: e.left,
                right: e.right,
                speed: e.speed,
            },
            Velocity(Vec2::new(e.speed, 0.0)),
            Collider {
                size: Vec2::new(34.0, 34.0),
            },
            LevelEntity,
        ));
    }
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pixel_adventure/derived/checkpoint.png"),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(46.0, 82.0)),
                ..default()
            },
            transform: Transform::from_xyz(level.checkpoint.x, level.checkpoint.y, 3.0),
            ..default()
        },
        Checkpoint {
            position: level.checkpoint,
        },
        Collider {
            size: Vec2::new(32.0, 100.0),
        },
        LevelEntity,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pixel_adventure/Free/Items/Boxes/Box3/Idle.png"),
            sprite: Sprite {
                color: Color::srgb(1.0, 0.88, 0.58),
                custom_size: Some(Vec2::new(62.0, 52.0)),
                ..default()
            },
            transform: Transform::from_xyz(level.shop.x, level.shop.y, 3.0),
            ..default()
        },
        Shop,
        Collider {
            size: Vec2::new(74.0, 70.0),
        },
        LevelEntity,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pixel_adventure/derived/goal.png"),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(64.0, 96.0)),
                ..default()
            },
            transform: Transform::from_xyz(level.goal.x, level.goal.y, 3.0),
            ..default()
        },
        Goal,
        Collider {
            size: Vec2::new(54.0, 125.0),
        },
        LevelEntity,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pixel_adventure/derived/player_idle.png"),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_translation(game.checkpoint),
            ..default()
        },
        Player,
        Velocity(Vec2::ZERO),
        Collider { size: PLAYER_SIZE },
        LevelEntity,
    ));
}

fn restart_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    entities: Query<Entity, With<LevelEntity>>,
) {
    if keys.just_pressed(KeyCode::KeyR) && game.mode != GameMode::Playing {
        for e in entities.iter() {
            commands.entity(e).despawn_recursive();
        }
        game.level = 0;
        game.lives = 3;
        game.score = 0;
        game.bank_coins = 0;
        game.frog_coins = 0;
        game.speed_boots = false;
        game.spring_legs = false;
        game.shield_charm = false;
        game.shop_open = false;
        game.shop_message = "Find the piggy shop. E opens it.".to_string();
        game.checkpoint = LEVELS[0].spawn;
        game.mode = GameMode::Playing;
        spawn_level(&mut commands, &game, &asset_server);
    }
}

fn player_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    game: Res<Game>,
    mut query: Query<(&mut Velocity, &Transform, &Collider), With<Player>>,
    platforms: Query<(&Transform, &Collider), (With<Platform>, Without<Player>)>,
) {
    if game.mode != GameMode::Playing {
        return;
    }
    let Ok((mut velocity, transform, collider)) = query.get_single_mut() else {
        return;
    };
    let dt = time.delta_seconds();
    let left = keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA);
    let right = keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD);
    let move_speed = if game.speed_boots {
        MOVE_SPEED * 1.25
    } else {
        MOVE_SPEED
    };
    let target = if left && !right {
        -move_speed
    } else if right && !left {
        move_speed
    } else {
        0.0
    };
    if target != 0.0 {
        velocity.x += (target - velocity.x).clamp(-ACCEL * dt, ACCEL * dt);
    } else {
        let drop = FRICTION * dt;
        velocity.x = if velocity.x.abs() <= drop {
            0.0
        } else {
            velocity.x - drop * velocity.x.signum()
        };
    }
    let grounded = is_grounded(transform.translation, collider.size, &platforms);
    let jump_pressed = keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::ArrowUp)
        || keys.just_pressed(KeyCode::KeyW);
    if grounded && jump_pressed {
        velocity.y = if game.spring_legs {
            JUMP_SPEED * 1.18
        } else {
            JUMP_SPEED
        };
        play("jump");
    }
    if velocity.y > 0.0
        && !(keys.pressed(KeyCode::Space)
            || keys.pressed(KeyCode::ArrowUp)
            || keys.pressed(KeyCode::KeyW))
    {
        velocity.y *= 0.93;
    }
}

fn apply_velocity(
    time: Res<Time>,
    game: Res<Game>,
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    if game.mode != GameMode::Playing {
        return;
    }
    let dt = time.delta_seconds();
    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.y += GRAVITY * dt;
        velocity.y = velocity.y.max(-1050.0);
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}

fn enemy_ai(
    time: Res<Time>,
    game: Res<Game>,
    mut query: Query<(&mut Transform, &mut Velocity, &Enemy)>,
) {
    if game.mode != GameMode::Playing {
        return;
    }
    for (mut transform, mut velocity, enemy) in query.iter_mut() {
        if transform.translation.x < enemy.left {
            transform.translation.x = enemy.left;
            velocity.x = enemy.speed.abs();
        }
        if transform.translation.x > enemy.right {
            transform.translation.x = enemy.right;
            velocity.x = -enemy.speed.abs();
        }
        transform.scale.x = velocity.x.signum().max(0.1);
        velocity.y = 0.0;
        transform.translation.y += (time.elapsed_seconds() * 8.0 + enemy.left).sin() * 0.02;
    }
}

fn resolve_player_platforms(
    mut query: Query<(&mut Transform, &mut Velocity, &Collider), With<Player>>,
    platforms: Query<(&Transform, &Collider), (With<Platform>, Without<Player>)>,
) {
    let Ok((mut transform, mut velocity, collider)) = query.get_single_mut() else {
        return;
    };
    for (pt, pc) in platforms.iter() {
        if let Some(overlap) = overlap(
            transform.translation,
            collider.size,
            pt.translation,
            pc.size,
        ) {
            if overlap.x < overlap.y {
                transform.translation.x += overlap.x
                    * if transform.translation.x > pt.translation.x {
                        1.0
                    } else {
                        -1.0
                    };
                velocity.x = 0.0;
            } else {
                transform.translation.y += overlap.y
                    * if transform.translation.y > pt.translation.y {
                        1.0
                    } else {
                        -1.0
                    };
                velocity.y = 0.0;
            }
        }
    }
}

fn collect_items(
    mut commands: Commands,
    mut game: ResMut<Game>,
    player: Query<(&Transform, &Collider), With<Player>>,
    coins: Query<(Entity, &Transform, &Collider), With<Collectible>>,
) {
    if game.mode != GameMode::Playing {
        return;
    }
    let Ok((pt, pc)) = player.get_single() else {
        return;
    };
    for (entity, ct, cc) in coins.iter() {
        if overlap(pt.translation, pc.size, ct.translation, cc.size).is_some() {
            commands.entity(entity).despawn_recursive();
            game.score += 100;
            game.bank_coins += APPLE_BANK_VALUE;
            game.frog_coins += 1;
            game.shop_message = format!("+{} piggy bank coins, +1 frog coin.", APPLE_BANK_VALUE);
            play("coin");
        }
    }
}

fn shop_interaction(
    keys: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    player: Query<(&Transform, &Collider), With<Player>>,
    shops: Query<(&Transform, &Collider), With<Shop>>,
) {
    if game.mode != GameMode::Playing {
        return;
    }
    let Ok((pt, pc)) = player.get_single() else {
        return;
    };
    let near_shop = shops.iter().any(|(st, sc)| {
        overlap(
            pt.translation,
            pc.size + Vec2::splat(34.0),
            st.translation,
            sc.size,
        )
        .is_some()
    });

    if !near_shop {
        if game.shop_open {
            game.shop_open = false;
            game.shop_message = "Shop closed. Come back with frog coins.".to_string();
        }
        return;
    }

    if keys.just_pressed(KeyCode::KeyE) || keys.just_pressed(KeyCode::Enter) {
        game.shop_open = !game.shop_open;
        game.shop_message = if game.shop_open {
            "Piggy Shop open: press 1 Speed Boots, 2 Spring Legs, 3 Shield Charm. Each costs 2 frog coins.".to_string()
        } else {
            "Piggy Shop closed.".to_string()
        };
        play("checkpoint");
    }

    if !game.shop_open {
        game.shop_message = "Piggy Shop nearby: press E to open.".to_string();
        return;
    }

    let choice = if keys.just_pressed(KeyCode::Digit1) {
        Some(1)
    } else if keys.just_pressed(KeyCode::Digit2) {
        Some(2)
    } else if keys.just_pressed(KeyCode::Digit3) {
        Some(3)
    } else {
        None
    };

    if let Some(choice) = choice {
        if game.frog_coins < POWERUP_COST {
            game.shop_message = format!(
                "Need {} frog coins per power-up. Collect more apples.",
                POWERUP_COST
            );
            play("hurt");
            return;
        }
        let already_owned = match choice {
            1 => game.speed_boots,
            2 => game.spring_legs,
            3 => game.shield_charm,
            _ => false,
        };
        if already_owned {
            game.shop_message = "You already own that power-up.".to_string();
            return;
        }
        game.frog_coins -= POWERUP_COST;
        match choice {
            1 => {
                game.speed_boots = true;
                game.shop_message = "Bought Speed Boots: move 25% faster.".to_string();
            }
            2 => {
                game.spring_legs = true;
                game.shop_message = "Bought Spring Legs: jump 18% higher.".to_string();
            }
            3 => {
                game.shield_charm = true;
                game.shop_message = "Bought Shield Charm: blocks your next hit.".to_string();
            }
            _ => {}
        }
        play("coin");
    }
}

fn enemy_player_contact(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut player: Query<(&mut Transform, &mut Velocity, &Collider), With<Player>>,
    enemies: Query<(Entity, &Transform, &Collider), (With<Enemy>, Without<Player>)>,
) {
    if game.mode != GameMode::Playing {
        return;
    }
    let Ok((mut pt, mut pv, pc)) = player.get_single_mut() else {
        return;
    };
    for (entity, et, ec) in enemies.iter() {
        if overlap(pt.translation, pc.size, et.translation, ec.size).is_some() {
            let player_bottom = pt.translation.y - pc.size.y / 2.0;
            let enemy_top = et.translation.y + ec.size.y / 2.0;
            if pv.y < 0.0 && player_bottom > enemy_top - 16.0 {
                commands.entity(entity).despawn_recursive();
                pv.y = JUMP_SPEED * 0.62;
                game.score += 250;
                play("stomp");
            } else {
                damage_player(&mut game, &mut pt, &mut pv);
            }
        }
    }
}

fn hazard_goal_checkpoint(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    level_entities: Query<Entity, With<LevelEntity>>,
    mut player: Query<(&mut Transform, &mut Velocity, &Collider), With<Player>>,
    hazards: Query<(&Transform, &Collider), (With<Hazard>, Without<Player>)>,
    goals: Query<(&Transform, &Collider), (With<Goal>, Without<Player>)>,
    checkpoints: Query<(&Transform, &Collider, &Checkpoint), Without<Player>>,
) {
    if game.mode != GameMode::Playing {
        return;
    }
    let Ok((mut pt, mut pv, pc)) = player.get_single_mut() else {
        return;
    };
    if pt.translation.y < -520.0 {
        damage_player(&mut game, &mut pt, &mut pv);
        return;
    }
    for (ht, hc) in hazards.iter() {
        if overlap(pt.translation, pc.size, ht.translation, hc.size).is_some() {
            damage_player(&mut game, &mut pt, &mut pv);
            return;
        }
    }
    for (ct, cc, checkpoint) in checkpoints.iter() {
        if overlap(pt.translation, pc.size, ct.translation, cc.size).is_some()
            && game.checkpoint != checkpoint.position
        {
            game.checkpoint = checkpoint.position;
            game.score += 50;
            play("checkpoint");
        }
    }
    for (gt, gc) in goals.iter() {
        if overlap(pt.translation, pc.size, gt.translation, gc.size).is_some() {
            game.score += 1000;
            if game.level + 1 < LEVEL_COUNT {
                for e in level_entities.iter() {
                    commands.entity(e).despawn_recursive();
                }
                game.level += 1;
                game.checkpoint = LEVELS[game.level].spawn;
                spawn_level(&mut commands, &game, &asset_server);
            } else {
                game.mode = GameMode::Won;
                play("win");
            }
            return;
        }
    }
}

fn damage_player(game: &mut Game, transform: &mut Transform, velocity: &mut Velocity) {
    if game.shield_charm {
        game.shield_charm = false;
        game.shop_message = "Shield Charm blocked the hit.".to_string();
        transform.translation = game.checkpoint;
        velocity.0 = Vec2::ZERO;
        play("checkpoint");
        return;
    }
    game.lives -= 1;
    play("hurt");
    if game.lives <= 0 {
        game.mode = GameMode::Lost;
        velocity.0 = Vec2::ZERO;
    } else {
        transform.translation = game.checkpoint;
        velocity.0 = Vec2::ZERO;
    }
}

fn camera_follow(
    game: Res<Game>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let Ok(pt) = player.get_single() else {
        return;
    };
    let Ok(mut ct) = camera.get_single_mut() else {
        return;
    };
    let half = LEVELS[game.level].width / 2.0;
    let target_x = pt.translation.x.clamp(-half + 480.0, half - 480.0);
    ct.translation.x = ct.translation.x + (target_x - ct.translation.x) * 0.12;
    ct.translation.y = ct.translation.y
        + ((pt.translation.y + 80.0).clamp(-60.0, 165.0) - ct.translation.y) * 0.08;
}

fn update_hud(
    game: Res<Game>,
    mut hud: Query<&mut Text, With<Hud>>,
    mut banner: Query<&mut Text, (With<Banner>, Without<Hud>)>,
) {
    if let Ok(mut text) = hud.get_single_mut() {
        let owned = format!(
            "{}{}{}",
            if game.speed_boots { " SpeedBoots" } else { "" },
            if game.spring_legs { " SpringLegs" } else { "" },
            if game.shield_charm { " Shield" } else { "" }
        );
        text.sections[0].value = format!(
            "Level {} / {}  Score {}  Lives {}  Piggy Bank {}  Frog Coins {}{}\nMove A/D or ←/→  Jump Space/W/↑  Shop E/Enter then 1/2/3  {}",
            game.level + 1,
            LEVEL_COUNT,
            game.score,
            game.lives.max(0),
            game.bank_coins,
            game.frog_coins,
            owned,
            game.shop_message
        );
    }
    if let Ok(mut text) = banner.get_single_mut() {
        text.sections[0].value = match game.mode {
            GameMode::Playing => "".to_string(),
            GameMode::Won => format!("YOU WIN! Final score: {}   Press R to restart", game.score),
            GameMode::Lost => "GAME OVER   Press R to restart".to_string(),
        };
    }
}

fn is_grounded(
    pos: Vec3,
    size: Vec2,
    platforms: &Query<(&Transform, &Collider), (With<Platform>, Without<Player>)>,
) -> bool {
    let probe = Vec3::new(pos.x, pos.y - 3.0, pos.z);
    platforms
        .iter()
        .any(|(pt, pc)| overlap(probe, size, pt.translation, pc.size).is_some())
}

fn overlap(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> Option<Vec2> {
    let dx = b_pos.x - a_pos.x;
    let px = (a_size.x + b_size.x) / 2.0 - dx.abs();
    if px <= 0.0 {
        return None;
    }
    let dy = b_pos.y - a_pos.y;
    let py = (a_size.y + b_size.y) / 2.0 - dy.abs();
    if py <= 0.0 {
        return None;
    }
    Some(Vec2::new(px, py))
}

fn play(name: &str) {
    #[cfg(target_arch = "wasm32")]
    azzle_play_sound(name);

    #[cfg(not(target_arch = "wasm32"))]
    let _ = name;
}
