#![allow(unused)]

use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

const PLAYER_SPRITE: &str = "submarine.png";
const SPRITES: &str = "wall.png";
//Entity, Component, System, Resource

// < Resources >
pub struct Materials {
    player_materials: Handle<ColorMaterial>,
    sprites_materials: Handle<ColorMaterial>,
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
        Self(100., 100.)
    }
}
impl PlayerSpeed {
	fn set(&mut self, speedx: f32, speedy: f32) {
		self.0 = speedx;
        self.1 = speedy;
	}
}
struct Colliding(bool);
struct Wall(f32, f32);
impl Default for Wall {
    fn default() -> Self {
        Self(0., 0.)
    }
}
struct RigidBody;
// </Components>

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.078125, 0.203125, 0.390625)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders".to_string(),
            width: 590.0,
            height: 670.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system())
        .add_system(collision.system())
        .run();
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut windows: ResMut<Windows>
) {
	let mut window = windows.get_primary_mut().unwrap();

	// camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	// create the main resources
	commands.insert_resource(Materials {
		player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into()),
		sprites_materials: materials.add(asset_server.load(SPRITES).into())
	});
	commands.insert_resource(WinSize {
		w: window.width(),
		h: window.height()
	});
	

	//position window
	let mut window = windows.get_primary_mut().unwrap();
	window.set_position(IVec2::new(500, 500));


	// adding the walls
	commands
		.spawn_bundle(SpriteBundle {
		material: materials.add(asset_server.load(SPRITES).into()),
		transform: Transform {
					translation: Vec3::new(0., 0., 1.),
                    scale: Vec3::new(0.15, 0.2, 1.),
					..Default::default()
				},
		..Default::default()
	})
	.insert(RigidBody)
	.insert(Wall::default());

    // walls collision box
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1., 0.7, 0.7).into()),
            // transform: Transform {
            //     translation: Vec3::new(0., bottom + 25., 10.),
            //     scale: Vec3::new(0.5, 0.5, 1.),
            //     ..Default::default()
            // },
            sprite: Sprite::new(Vec2::new(100., 100.)),
            ..Default::default()
        });
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<WinSize>) {
    //spawn a sprite
    let bottom = -win_size.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_materials.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + 25., 10.),
                scale: Vec3::new(0.5, 0.5, 1.),
                ..Default::default()
            },
            //sprite: Sprite::new(Vec2::new(100., 50.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerSpeed::default())
        .insert(Colliding(false));

}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerSpeed, &mut Transform, With<Player>)>,
) {
    if let Ok((speed, mut transform, _)) = query.single_mut() {
        let dirx = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };
        let diry = if keyboard_input.pressed(KeyCode::Down) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Up) {
            1.
        } else {
            0.
        };
        transform.translation += Vec3::new(
            dirx * speed.0 * time.delta_seconds(),
            diry * speed.1 * time.delta_seconds(),
            0.,
        );
    }
}

fn collision(
	time: Res<Time>,
	mut wall_query: Query<(&Wall, With<RigidBody>)>,
	mut player_query: Query<(&mut PlayerSpeed, &mut Transform, &Sprite, With<Colliding>, With<Player>)>
	) {

	if let Ok((Wall(x, y),rigidbody)) = wall_query.single_mut() {
		if let Ok((mut speed, mut transform, mut colliding, sprite, _)) = player_query.single_mut() {
			let mut pos = transform.translation;
			// if pos.x > *x && pos.y > *y {
			// 	if pos.x < *x + 100. && pos.y < *y + 100.{
			// 		colliding = true;
			// 		pos.x = 0.;
			// 	} else {
			// 		colliding = false;
			// 	}
			// } else {
			// 	colliding = false;
			// }
            let collision = collide(
                transform.translation,
                Vec2::new(60., 40.),
                //sprite.size(),
                Vec3::new(0., 0., 10.),
                Vec2::new(100., 100.),
            );
            if let Some(collision) = collision {
                match collision {
                        Collision::Left  => speed.0 *= if speed.0 > 0.0 { -1. } else { 1. },
                        Collision::Right => speed.0 *= if speed.0 < 0.0 {  1. } else {-1. },
                        Collision::Top   => speed.1 *= if speed.1 < 0.0 {  1. } else {-1. },
                        Collision::Bottom=> speed.1 *= if speed.1 > 0.0 { -1. } else { 1. },
                    }
            } else {
                speed.0 = 100.;
                speed.1 = 100.;
             }
			//println!("{}", colliding);
		}
	}
}
