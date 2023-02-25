use std::collections::HashMap;

use bevy::prelude::*;
// use bevy::render::mesh::Indices;
// use bevy::render::render_resource::PrimitiveTopology;
use bevy::time::FixedTimestep;

use bevy_obj::*;
use hexx::shapes;
use hexx::*;


pub fn main() {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 1.0,
            ..default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "I am a window!".to_string(),
                width: 960.,
                height: 540.,
                mode: WindowMode::Windowed,
                ..default()
            },
            ..default()
        }))
        .add_plugin(ObjPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(animate_light),
        )
        .run();
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, -17.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn animate_light(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DynamicLight>>,
) {
    for mut transform in &mut query {
        let sec = time.elapsed().as_millis() as f32 / 1000.0;
        transform.translation = Vec3::new(sec.sin() * 2.0, 2.0, sec.cos() * 2.0);
    }
}

#[derive(Component)]
pub struct DynamicLight;

// fn animate_rings(
//     mut commands: Commands,
//     map: Res<Map>,
//     mut highlighted_hexes: Local<HighlightedHexes>,
// ) {
//     // Clear highlighted hexes materials
//     for entity in highlighted_hexes
//         .hexes
//         .iter()
//         .filter_map(|h| map.entities.get(h))
//     {
//         commands
//             .entity(*entity)
//             .insert(map.default_material.clone());
//     }
//     highlighted_hexes.ring += 1;
//     if highlighted_hexes.ring > MAP_RADIUS {
//         highlighted_hexes.ring = 0;
//     }
//     highlighted_hexes.hexes = Hex::ZERO.ring(highlighted_hexes.ring);
//     // Draw a ring
//     for h in &highlighted_hexes.hexes {
//         if let Some(e) = map.entities.get(h) {
//             commands.entity(*e).insert(map.highlighted_material.clone());
//         }
//     }
// }


fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

	let tile = asset_server.load("models/tile_basic.obj");
	let default_material = materials.add(Color::WHITE.into());
	
	let grid = HexLayout {
            orientation: HexOrientation::pointy(),
            origin: Vec2::ZERO,
            hex_size: Vec2::splat(1.0),
    };
	
	let tiles: HashMap<Hex, Entity> = shapes::hexagon(Hex::ZERO, 10)
        .map(|hex| {
            let pos = grid.hex_to_world_pos(hex);
            let id = commands
                .spawn(PbrBundle {
	               mesh: tile.clone(),
	               material: default_material.clone(),
	               transform: Transform::from_xyz(pos.x, 0.0, pos.y),
	               ..default()
	               })
                .id();
            (hex, id)
        })
        .collect();
    
    // directional 'sun' light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 10.0, 0.0),
            rotation: Quat::from_rotation_x(3.14),
            ..default()
        },
        ..default()
    });
    
    // red point light
    let point_light_id = commands.spawn(
                PointLightBundle {
                // transform: Transform::from_xyz(5.0, 8.0, 2.0),
                transform: Transform::from_xyz(1.0, 2.0, 0.0),
                point_light: PointLight {
                    intensity: 600.0, // 1600.0 lumens - roughly a 100W non-halogen incandescent bulb
                    color: Color::RED,
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            }
        ).id();
    
    commands.entity(point_light_id)
        .insert(DynamicLight);
        
        // .with_children(|builder| {
        //     builder.spawn(PbrBundle {
        //         mesh: meshes.add(Mesh::from(shape::UVSphere {
        //             radius: 0.1,
        //             ..default()
        //         })),
        //         material: materials.add(StandardMaterial {
        //             base_color: Color::RED,
        //             emissive: Color::rgba_linear(100.0, 0.0, 0.0, 0.0),
        //             ..default()
        //         }),
        //         ..default()
        //     });
        // });
}
