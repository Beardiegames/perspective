mod realhex;

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_obj::*;
use hexx::shapes;
use hexx::*;
use realhex::*;
use noise::*;


pub type HexDir = hexx::Direction;

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
                .with_run_criteria(FixedTimestep::step(0.25))
                .with_system(tile_position_system)
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.01))
                //.with_system(tile_swap_system)
                //.with_system(tile_transform_system)
                //.with_system(tile_material_system)
                .with_system(animate_light)
                .with_system(camera_rotation),
        )
        .run();
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 15.0, -30.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut tf in &mut query {
        let sec = time.elapsed().as_millis() as f32 / 3000.0;
        let target = Transform::from_xyz(sec.sin() * 20.0, 10.0, sec.cos() * 20.0)
            .looking_at(Vec3::new(0.0, -5.0, 0.0), Vec3::Y);
            
        tf.translation = target.translation;
        tf.rotation = target.rotation;
    }
}

#[derive(Component)]
pub struct DynamicLight;

fn animate_light(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DynamicLight>>,
) {
    for mut transform in &mut query {
        let sec = time.elapsed().as_millis() as f32 / 1000.0;
        transform.translation = Vec3::new(sec.sin() * 5.0, 2.5, sec.cos() * 5.0);
    }
}

fn tile_position_system(
    mut map: ResMut<Map>,
) { 
    //map.world_position += RealHex::new(0.0, 0.1);
}

// fn tile_transform_system(
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut tile_queue: Query<(&mut Transform, &Tile)>,
//     map: Res<Map>,
//     noise: Res<PerlinNoise>,
// ) {    
//     let world_pos = &map.world_position;
        
//     for (mut t, tile) in &mut tile_queue {
//         let offset = match tile.prefab.shape == TileShape::Floor { true => -1.0, false => 0.0 };
//         let height = tile.height(world_pos, &noise) + offset;
        
//         t.translation = Vec3::new(t.translation.x, height * 0.5, t.translation.z); 
//         t.rotation = Quat::from_rotation_y(tile.rotation());
        
//         if let Some(m) = materials.get_mut(&tile.material) {
//             m.base_color = Color::Rgba { red: height/2.0 + 0.5, green: height/2.0 +0.5, blue: 0.5, alpha: 1.0 }
//         }
//     }
// }

#[derive(Component, Clone)]
pub struct Tile {
    pub hex: Hex,
    pub entity: Option<Entity>,
}

impl Tile {

    pub fn new(hex: Hex) -> Self {
        Tile {
            hex,
            entity: None,
        }
    }
    
    pub fn height(&self, world_pos: &Hex, noise: &PerlinNoise) -> f32 {
        let real_hex = Hex::new(
            world_pos.x() + self.hex.x(),
            world_pos.y() + self.hex.y()
        );
        let point = [real_hex.x() as f64 / 10.0, real_hex.y() as f64 / 10.0];
        (noise.perlin.get(point) as f32 * 3.0).round()
    }

    pub fn despawn(&mut self, commands: &mut Commands) {
        if let Some(e) = self.entity {
            commands.entity(e).despawn();
            self.entity = None;
        }
        
    }
    
    pub fn spawn(&mut self, commands: &mut Commands, bundle: PbrBundle) {
        if self.entity.is_none() {
            self.entity = Some(
                commands
                .spawn(bundle)
                .insert(self.clone())
                .id()
            )
        }
    }
}

#[derive(Resource)]
pub struct PerlinNoise {
    pub perlin: Perlin,
}

impl PerlinNoise {
    pub fn height(&self, pos: &RealHex) -> f32 {
        let point = [pos.x as f64 / 10.0, pos.y as f64 / 10.0];
        (self.perlin.get(point) as f32 * 3.0).round() * 0.5
    }
}

#[derive(Resource, Clone)]
pub struct Prefab {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl Prefab {
    pub fn instance(&self, commands: &mut Commands, transform: Transform) -> PbrBundle {
        PbrBundle {
            mesh: self.mesh.clone(),
            material: self.material.clone(),
            transform,
            ..default()
        }
    }
}

#[derive(Resource, Clone)]
pub struct SceneryPrefabs {
    pub grass_tile: Prefab,
}

impl SceneryPrefabs {
    pub fn load(asset_server: &AssetServer, materials: &mut Assets<StandardMaterial>) -> SceneryPrefabs {
        SceneryPrefabs {
            grass_tile: Prefab {
                mesh: asset_server.load("models/tile_basic.obj"),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    base_color_texture: Some(asset_server.load("models/GrassTile.png")),
                    reflectance: 0.0,
                    metallic: 0.0,
                    ..Default::default()
                })
            },
        }
    }
}

#[derive(Resource)]
pub struct Map {
    layout: HexLayout, 
    scenery: SceneryPrefabs, 
    pub grid: Vec<Tile>,
    pub world_position: RealHex, 
}

impl Map {
    pub fn tile_at_hex(&self, hex: &Hex) -> Option<&Tile> {
        self.grid.iter().find(|x| x.hex == *hex)
    }
}

fn setup_tiles(
    mut commands: Commands, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    let noise = PerlinNoise{ perlin: Perlin::new(0u32) };
    
    let layout = HexLayout {
        orientation: HexOrientation::pointy(),
        origin: Vec2::ZERO,
        hex_size: Vec2::splat(1.0),
    };

    let scenery = SceneryPrefabs::load(&asset_server, &mut materials);

    let grid = shapes::hexagon(Hex::ZERO, 10)
        .map(|hex| {
            let mut tile = Tile::new(hex);
            let pos = layout.hex_to_world_pos(hex);
            let dist = Vec2::ZERO.distance(pos) * 0.1;
            // println!("dist ({:?}) = {}", hex, dist);
            
            let transform = Transform {
                translation: Vec3::new(
                    pos.x as f32, 
                    noise.height(&RealHex::from(&hex)) - dist, 
                    pos.y as f32
                ),
                ..Default::default()
            };
            
            let instance = scenery.grass_tile.instance(&mut commands, transform);  
            // if let Some(m) = materials.get_mut(&instance.material) {
            //     let mut c = Color::WHITE.as_rgba_f32();
            //         c[0] /= dist;
            //         c[1] /= dist;
            //         c[2] /= dist;
                    
            //     m.base_color = c.into();
            // }
                      
            tile.spawn(&mut commands, instance);
            tile
        })
        .collect();
        
    commands.insert_resource(Map {layout, scenery, grid, world_position: RealHex::default() });
    commands.insert_resource(noise);
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
                    intensity: 1200.0, // 1600.0 lumens - roughly a 100W non-halogen incandescent bulb
                    color: Color::WHITE,
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            }
        ).id();
    
    commands.entity(point_light_id)
        .insert(DynamicLight);
}