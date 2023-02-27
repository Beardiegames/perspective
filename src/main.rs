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
        .add_startup_system(setup_tiles)
        .add_startup_system(setup_lights)
        .add_system(bevy::window::close_on_esc)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(realtime_tile_spawner)
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

fn realtime_tile_spawner(
    //mut commands: Commands,
    tiles: ResMut<Tiles>,
) {
    let some_tilepos = tiles.layout.world_pos_to_hex(Vec2::new(0.0, 0.0));
}

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


fn setup_lights(
    mut commands: Commands,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    //asset_server: Res<AssetServer>,
) {
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
}

#[derive(Resource)]
pub struct TileMeshes {
    floor: Handle<Mesh>,
}

impl TileMeshes {
    fn from_load(asset_server: Res<AssetServer>) -> Self {
        TileMeshes {        
            floor: asset_server.load("models/tiles/tile-floor.obj"),
        }
    }
}

#[derive(Resource)]
pub struct Tiles {
    layout: HexLayout,
    meshes: TileMeshes,
    material: Handle<StandardMaterial>,
    grid: HashMap<Hex, Entity>,
}

fn setup_tiles(
    mut commands: Commands, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    let layout = HexLayout {
        orientation: HexOrientation::pointy(),
        origin: Vec2::ZERO,
        hex_size: Vec2::splat(1.0),
    };
    
    let meshes = TileMeshes::from_load(asset_server);
    let material = materials.add(Color::WHITE.into());

    let grid = shapes::hexagon(Hex::ZERO, 10)
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn(PbrBundle {
	               mesh: meshes.floor.clone(),
	               material: material.clone(),
	               transform: Transform::from_xyz(pos.x, 0.0, pos.y),
	               ..default()
	               })
                .id();
            (hex, id)
        })
        .collect();
        
    commands.insert_resource(Tiles {
            layout,
            meshes,
            material,
            grid,
    });
}
