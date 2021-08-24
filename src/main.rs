#![allow(unused)]

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";
const SPRITES: &str = "assets.png";
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
struct PlayerSpeed(f32);
impl Default for PlayerSpeed {
    fn default() -> Self {
        Self(100.)
    }
}
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
					translation: Vec3::new(350., -200., 1.),
					..Default::default()
				},
		..Default::default()
	})
	.insert(RigidBody)
	.insert(Wall::default());
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
            // sprite: Sprite::new(Vec2::new(100., 50.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerSpeed::default());
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
            diry * speed.0 * time.delta_seconds(),
            0.,
        );
    }
}

fn collision(
	mut wall_query: Query<(&Wall, With<RigidBody>)>,
	mut player_query: Query<(&PlayerSpeed, &mut Transform, With<Player>)>
	) {

	if let Ok((Wall(x, y),rigidbody)) = wall_query.single_mut() {
		if let Ok((speed, mut transform, _)) = player_query.single_mut() {
			let pos = transform.translation;
			if pos.x > *x && pos.y > *y {
				if pos.x < *x + 100. && pos.y < *y + 100.{
					println!("colliding");
				}
			} 
		} 
	}
}