#![allow(unused)]

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";
const TIME_STEP: f32 = 1. / 60. ;
//Entity, Component, System, Resource

// < Resources >
pub struct Materials {
	player_materials: Handle<ColorMaterial>,
}
struct WinSize {
	w: f32,
	h: f32
}
// </ Reources >

// < Components>
struct Player;
struct PlayerSpeed(f32);
impl Default for PlayerSpeed {
	fn default() -> Self {
		Self(100.)
	}
}
struct RigidBody;
// </Components>


fn main() {
	App::build()
		.insert_resource(ClearColor(Color::rgb(0.03, 0.01, 0.01)))
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
			SystemStage::single(player_spawn.system()))
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
		player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into())
	});
	commands.insert_resource(WinSize {
		w: window.width(),
		h: window.height()
	});
	

	//position window
	let mut window = windows.get_primary_mut().unwrap();
	window.set_position(IVec2::new(500, 500));
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<WinSize>) {
	//spawn a sprite
	let bottom = -win_size.h / 2.;
	commands.spawn_bundle(SpriteBundle {
		material: materials.player_materials.clone(),
		transform: Transform {
			translation: Vec3::new(0., bottom+25., 10.),
			scale: Vec3::new(0.5, 0.5, 1.),
			..Default::default()
		},
 		// sprite: Sprite::new(Vec2::new(100., 50.)),
		..Default::default()
	})
	.insert(Player)
	.insert(PlayerSpeed::default());
}

fn player_movement(
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&PlayerSpeed, &mut Transform, With<Player>)>
	){
	if let Ok((speed, mut transform, _)) = query.single_mut() {
		let dirx  = if keyboard_input.pressed(KeyCode::Left) {
			-1.
		} else if keyboard_input.pressed(KeyCode::Right) {
			1.
		} else {
			0.
		};
		let diry  = if keyboard_input.pressed(KeyCode::Down) {
			-1.
		} else if keyboard_input.pressed(KeyCode::Up) {
			1.
		} else {
			0.
		};
		transform.translation += Vec3::new(dirx * speed.0 * TIME_STEP, diry * speed.0 * TIME_STEP, 0.);
	}
}

fn collision(mut query: Query<(With<RigidBody>)>) {
	if let Ok((rigidbody)) = query.single_mut() {
		println!("{:?}", rigidbody);
	}
}