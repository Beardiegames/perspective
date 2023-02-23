use std::collections::HashMap;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::time::FixedTimestep;

use bevy_obj::*;
use hexx::shapes;
use hexx::*;


/// World space height of hex columns
const COLUMN_HEIGHT: f32 = 1.0;
/// Map radius
const MAP_RADIUS: u32 = 20;
    

pub fn main() {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 1.0,
            ..default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "I am a window!".to_string(),
                //width: 960.,
                //height: 540.,
                mode: WindowMode::Fullscreen,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_grid)
        .add_system(bevy::window::close_on_esc)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(animate_rings),
        )
        .run();
}

#[derive(Debug, Resource)]
struct Map {
	grid: HexLayout,
    entities: HashMap<Hex, Entity>,
    highlighted_material: Handle<StandardMaterial>,
    default_material: Handle<StandardMaterial>,
}

#[derive(Debug, Default, Resource)]
struct HighlightedHexes {
    ring: u32,
    hexes: Vec<Hex>,
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, -17.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Hex grid setup
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// hex layout settings
	let grid = HexLayout {
            orientation: HexOrientation::pointy(),
            ..default()
            //origin: Vec2::ZERO,
            //hex_size: Vec2::splat(1.0),
    };
    
    // materials
    let default_material = materials.add(Color::WHITE.into());
    let highlighted_material = materials.add(Color::YELLOW.into());
    // mesh
    let mesh = hexagonal_column(&grid);
    let mesh_handle = meshes.add(mesh);

    let entities = shapes::hexagon(Hex::ZERO, MAP_RADIUS)
        .map(|hex| {
            let pos = grid.hex_to_world_pos(hex);
            let id = commands
                .spawn(PbrBundle {
                    transform: Transform::from_xyz(pos.x, 0.0, pos.y).with_scale(Vec3::splat(0.95)),
                    mesh: mesh_handle.clone(),
                    material: default_material.clone(),
                    ..default()
                })
                .id();
            (hex, id)
        })
        .collect();
        
    commands.insert_resource(Map {
    	grid,
        entities,
        highlighted_material,
        default_material,
    });
}

fn animate_rings(
    mut commands: Commands,
    map: Res<Map>,
    mut highlighted_hexes: Local<HighlightedHexes>,
) {
    // Clear highlighted hexes materials
    for entity in highlighted_hexes
        .hexes
        .iter()
        .filter_map(|h| map.entities.get(h))
    {
        commands
            .entity(*entity)
            .insert(map.default_material.clone());
    }
    highlighted_hexes.ring += 1;
    if highlighted_hexes.ring > MAP_RADIUS {
        highlighted_hexes.ring = 0;
    }
    highlighted_hexes.hexes = Hex::ZERO.ring(highlighted_hexes.ring);
    // Draw a ring
    for h in &highlighted_hexes.hexes {
        if let Some(e) = map.entities.get(h) {
            commands.entity(*e).insert(map.highlighted_material.clone());
        }
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_column(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = MeshInfo::partial_hexagonal_column(hex_layout, Hex::ZERO, COLUMN_HEIGHT);
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs.to_vec());
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}

// fn setup(
    // mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // 
    // asset_server: Res<AssetServer>,
// ) {
// 
	// let tile = asset_server.load("models/tile.obj");
	// meshes.add(tile.clone());
// 
	// let xcount = 6;
	// let xhalf = xcount as f32 / 2.0;
	// let zcount = 4;
	// let zhalf = zcount as f32 / 2.0;
	// 
	// for x in 0..xcount {
	// for z in 0..zcount {
// 
		// let offset = match z % 2 == 0 {
			// true => 0.0,
			// false => 0.5,
		// };
	// 
	    // commands.spawn(PbrBundle {
	        // mesh:tile.clone(), // meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
	        // material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
	        // transform: Transform::from_xyz(
	        	// x as f32 - xhalf + offset,
	        	// 0.0,
	        	// z as f32 * 0.75 - zhalf
	        // ),
	        // ..default()
	    // });
	// }}
// }
