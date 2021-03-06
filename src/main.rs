// WARNING this code is a mess and don't ask me why. I made it in last 9 hrs with three days of experience with bevy
// submarine and mines images by my partner Seebass22
// this code is an bad example of how ECS should be used and if you find something useful (which I highly doubt), feel free to use it in your own games

#![allow(unused)]

use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use rand::prelude::*;

const PLAYER_SPRITE: &str = "submarine.png";
const MINE: &str = "mine.png";
//Entity, Component, System, Resource

// < Resources >
pub struct Materials {
    player_materials: Handle<ColorMaterial>,
    mine_materials: Handle<ColorMaterial>,
}
struct WinSize {
    w: f32,
    h: f32,
}
// </ Reources >

// < Components>
struct Player;
struct PlayerSpeed(f32, f32);
impl Default for PlayerSpeed {
    fn default() -> Self {
        Self(0., -200.)
    }
}
impl PlayerSpeed {
    fn set(&mut self, speedx: f32, speedy: f32) {
        self.0 = speedx;
        self.1 = speedy;
    }
}
struct Colliding(bool);
struct Wall(f32, f32, bool);
impl Default for Wall {
    fn default() -> Self {
        Self(0., 0., false)
    }
}
struct RigidBody;
// </Components>

fn main() {
    let mut app = App::build();
    app.insert_resource(ClearColor(Color::rgb(0.078125, 0.203125, 0.390625)))
        .insert_resource(WindowDescriptor {
            title: "Danger dive".to_string(),
            width: 640.0,
            height: 480.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system())
        .add_system(collision.system());

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let mut window = windows.get_primary_mut().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // create the main resources
    commands.insert_resource(Materials {
        player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        mine_materials: materials.add(asset_server.load(MINE).into()),
    });
    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    //position window
    let mut window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(500, 500));

    // adding the mines
    let mut rng = rand::thread_rng();
    let random: f32 = rng.gen();
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load(MINE).into()),
            transform: Transform {
                translation: Vec3::new(
                    window.height() / 2.,
                    random * (-1. * (-window.height() / 2. + 100.) + window.height() / 2.)
                        - 1. * window.height() / 2.
                        + 100.,
                    5.,
                ),
                scale: Vec3::new(1.0, 1.0, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody)
        .insert(Wall(0., random, true));
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<WinSize>) {
    //spawn a sprite
    let bottom = -win_size.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_materials.clone(),
            transform: Transform {
                translation: Vec3::new(-100., bottom + 25., 10.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerSpeed::default())
        .insert(Colliding(false));
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    win_size: Res<WinSize>,
    mut query: Query<(&mut PlayerSpeed, &mut Transform, With<Player>)>,
) {
    if let Ok((mut speed, mut transform, _)) = query.single_mut() {
        let bottom = -1. * win_size.h / 2. + 100.;
        let dirx = 0.;
        let diry = if keyboard_input.pressed(KeyCode::Down) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Up) {
            1.
        } else {
            0.
        };
        speed.1 += 5. * diry;
        speed.1 -= 1.;
        if speed.1 > 150. {
            speed.1 = 150.;
        }
        if speed.1 * -1. > 150. {
            speed.1 = -150.;
        }

        transform.translation += Vec3::new(
            speed.0 * time.delta_seconds(),
            speed.1 * time.delta_seconds(),
            0.,
        );

        if transform.translation.y > win_size.h / 2. {
            speed.1 *= -1.;
        }
        if transform.translation.y < bottom {
            speed.1 *= -1.;
            speed.1 /= 2.;
        }
    }
}

fn collision(
    mut queries: QuerySet<(
        Query<(&Wall, &mut Transform)>,
        Query<(&PlayerSpeed, &mut Transform, With<Player>)>,
    )>,
    win_size: Res<WinSize>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let bottom = -win_size.h / 2.;
    let mut speed = (0., 0.);
    let (mut player_x, mut player_y) = (0., 0.);
    if let Ok((mut speed, mut player_transform, _)) = queries.q1_mut().single_mut() {
        if player_transform.translation.y < bottom + 100. {
            player_transform.translation.y = bottom + 100.;
        }
        player_x = player_transform.translation.x;
        player_y = player_transform.translation.y;
    }
    for wall_query in queries.q0_mut().iter_mut() {
        let (Wall(x, y, damagble), mut transform) = wall_query;

        let x = transform.translation.x;
        let y = transform.translation.y;

        let collision = collide(
            Vec3::new(player_x, player_y, 10.),
            Vec2::new(180., 55.),
            Vec3::new(x, y, 10.),
            Vec2::new(50., 50.),
        );

        if let Some(collision) = collision {
            if (*damagble) {
                println!("deadb");
            }
            use std::process;
            process::exit(0x0100);

            println!("deadb");
            speed.0 = 1.;
        } else {
            speed.0 = 0.;
        }

        let mut rng = rand::thread_rng();
        let random: f32 = rng.gen();
        transform.translation.x -= 4.;
        if transform.translation.x < win_size.w / 2. * -1. {
            transform.translation.x = win_size.w / 2.;
            transform.translation.y = random * (-1. * (-win_size.h / 2. + 100.) + win_size.h / 2.)
                - 1. * win_size.h / 2.
                + 100.;
        }
    }
}
