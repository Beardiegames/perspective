use std::collections::HashMap;

use bevy::prelude::*;
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
                .with_system(tile_swap_system)
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

fn tile_swap_system(
    mut commands: Commands,
    noise: Res<PerlinNoise>,
    mut tiles: ResMut<Tiles>,
) {
    let prefabs = tiles.prefabs.clone();
    let layout = tiles.layout.clone();
    
    for (hex, tile) in &mut tiles.grid {
        tile.spawn_tile(&mut commands, &prefabs, &layout);
    }
}

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

#[derive(Component, Clone)]
pub struct Tile {
    hex: Hex,
    material: Handle<StandardMaterial>,
    shape: TileShape,
    entity: Option<Entity>,
}

impl Tile {
    pub fn new(hex: Hex, material: Handle<StandardMaterial>, prefabs: &TileBrefabs) -> Self {
        Tile {
            hex,
            material,
            shape: prefabs.floor.clone(),
            entity: None,
        }
    }
    
    fn set_shape(&mut self, commands: &mut Commands, prefabs: &TileBrefabs) {

    }
    
    fn spawn_tile(&mut self, commands: &mut Commands, prefabs: &TileBrefabs, layout: &HexLayout) {
        
        if self.entity.is_none() {
            let pos = layout.hex_to_world_pos(self.hex);
            
            let bundle = PbrBundle {
                mesh: self.shape.mesh(),
                material: self.material.clone(),
                transform: Transform::from_xyz(
	               pos.x as f32, 
	               0.0, 
	               pos.y as f32
	               ),
                ..default()
            };
            
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

#[derive(Clone, PartialEq)]
pub enum TileShape {
    Floor(Handle<Mesh>),
    Slope1(Handle<Mesh>),
    Slope2(Handle<Mesh>),
    Slope3(Handle<Mesh>),
    Slope4(Handle<Mesh>),
}

impl TileShape {
    pub fn mesh(&self) -> Handle<Mesh> {
        match &self {
            Self::Floor(m) => m.clone(),
            Self::Slope1(m) => m.clone(),
            Self::Slope2(m) => m.clone(),
            Self::Slope3(m) => m.clone(),
            Self::Slope4(m) => m.clone(),
        }
    }
}

#[derive(Resource, Clone)]
pub struct TileBrefabs {
    floor: TileShape,
    slope1: TileShape,
    slope2: TileShape,
    slope3: TileShape,
    slope4: TileShape,
}

impl TileBrefabs {
    fn from_load(asset_server: &Res<AssetServer>) -> Self {
        TileBrefabs {        
            floor: TileShape::Floor(asset_server.load("models/tiles/full.obj")),
            slope1: TileShape::Slope1(asset_server.load("models/tiles/slope-1.obj")),
            slope2: TileShape::Slope2(asset_server.load("models/tiles/slope-2.obj")),
            slope3: TileShape::Slope3(asset_server.load("models/tiles/slope-3.obj")),
            slope4: TileShape::Slope4(asset_server.load("models/tiles/slope-4.obj")),
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

    let grid = shapes::hexagon(Hex::ZERO, 5)
        .map(|hex| {
            let material = materials.add(Color::WHITE.into());
            let tile = Tile::new(hex, material, &prefabs);
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