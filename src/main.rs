use std::collections::HashMap;

use bevy::prelude::*;
use bevy::asset::LoadState;
use bevy::reflect::GetPath;
use bevy::render::renderer::RenderQueue;
// use bevy::render::mesh::Indices;
// use bevy::render::render_resource::PrimitiveTopology;
use bevy::time::FixedTimestep;
use bevy_obj::*;
use hexx::shapes;
use hexx::*;
use noise::*;


pub fn main() {
    let window = WindowDescriptor {
        title: "I am a window!".to_string(),
        width: 960.,
        height: 540.,
        mode: WindowMode::Windowed,
        ..default()
    };
    
    App::new()
        .insert_resource(AmbientLight {
            brightness: 1.0,
            ..default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window,
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
                .with_system(tile_transform_system)
                //.with_system(tile_material_system)
                .with_system(animate_light),
        )
        .run();
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, -20.0).looking_at(Vec3::ZERO, Vec3::Y),
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

fn tile_transform_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut tile_queue: Query<(&mut Transform, &Tile)>,
    noise: Res<PerlinNoise>,
) {    
    for (mut trans, tile) in &mut tile_queue {
        let point = [tile.hex.x() as f64 / 10.0, tile.hex.y() as f64 / 10.0];
        let height = noise.perlin.get(point) as f32;
        
        trans.translation = Vec3::new(
            trans.translation.x, 
            height.round() * 0.5,
            trans.translation.z, 
        ); 
        
        if let Some(m) = materials.get_mut(&tile.material) {
            m.base_color = Color::Rgba { red: height, green: height, blue: 0.5, alpha: 1.0 }
        }
    }
}

// fn tile_material_system(
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     tiles: ResMut<Tiles>,
// ) {
//     let perlin = tiles.perlin;
    
//     for (hex, tile) in &tiles.grid {
//         let point = [hex.x() as f64 / 10.0, hex.y() as f64 / 10.0];
//         let height = perlin.get(point).round() as f32;// * 0.5;
        
//         if let Some(m) = materials.get_mut(&tile.material) {
//             m.base_color = Color::Rgba { red: height, green: height, blue: 0.5, alpha: 1.0 }
//         }
//     }
// }

// fn realtime_tile_spawner(
//     mut commands: Commands,
//     tiles: ResMut<Tiles>,
// ) {
//     let perlin = tiles.perlin;
     
//     //let some_tilepos = tiles.layout.world_pos_to_hex(Vec2::new(0.0, 0.0));
//     for (hex, entity) in &tiles.grid {
//         let point = [hex.x() as f64, hex.y() as f64];
//         let height = perlin.get(point);
        
//         if let Some(obj) = commands.get_entity(*entity) {
//             obj.
//         }
//     }
// }

#[derive(Clone)]
pub struct TileShape {
    idx: usize,
    entity: Entity,
}

impl TileShape {
    
}

#[derive(Component, Clone)]
pub struct Tile {
    hex: Hex,
    material: Handle<StandardMaterial>,
    shape: TileShape,
}

impl Tile {
    fn set_shape(&mut self, commands: &mut Commands, prefabs: &TileBrefabs) {
        let bundle = PbrBundle {
	       mesh: prefabs.floor.clone(),
	       material: self.material.clone(),
	       transform: Transform::from_xyz(
	           self.hex.x as f32, 
	           0.0, 
	           self.hex.y as f32
	           ),
	       ..default()
	   };
	   
	   let entity = commands
            .spawn(bundle)
            .insert(self.clone())
            .id();
    }
}

#[derive(Resource)]
pub struct PerlinNoise {
    pub perlin: Perlin,
}

#[derive(Resource)]
pub struct TileBrefabs {
    floor: Handle<Mesh>,
    slope1: Handle<Mesh>,
    slope2: Handle<Mesh>,
    slope3: Handle<Mesh>,
    slope4: Handle<Mesh>,
}

impl TileBrefabs {
    fn from_load(asset_server: &Res<AssetServer>) -> Self {
        TileBrefabs {        
            floor: asset_server.load("models/tiles/full.obj"),
            slope1: asset_server.load("models/tiles/slope-1.obj"),
            slope2: asset_server.load("models/tiles/slope-2.obj"),
            slope3: asset_server.load("models/tiles/slope-3.obj"),
            slope4: asset_server.load("models/tiles/slope-4.obj"),
        }
    }
}

#[derive(Resource)]
pub struct Tiles {
    layout: HexLayout, 
    prefabs: TileBrefabs, 
    pub grid: HashMap<Hex, Tile>,
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
    
    let prefabs = TileBrefabs::from_load(&asset_server);
    let mesh = asset_server.load("models/tiles/full.obj");

    let grid = shapes::hexagon(Hex::ZERO, 5)
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let material = materials.add(Color::WHITE.into());
            
            
	       
	       let tile = Tile { 
                hex: hex.clone(),
                material: material.clone(),
            };

            

            (hex, tile)
        })
        .collect();
        
    commands.insert_resource(Tiles {layout, prefabs, grid });
    commands.insert_resource(PerlinNoise{ perlin: Perlin::new(0u32) });
}


fn setup_lights(
    mut commands: Commands,
) {
    // red point light
    let point_light_id = commands.spawn(
                PointLightBundle {
                // transform: Transform::from_xyz(5.0, 8.0, 2.0),
                transform: Transform::from_xyz(1.0, 20.0, 0.0),
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