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
    mut tiles: ResMut<Tiles>,
    noise: Res<PerlinNoise>,
) {    
    let prefabs = tiles.prefabs.clone();
    let layout = tiles.layout.clone();
    
    for i in 0..tiles.grid.len() {
        let hex = tiles.grid[i].hex;
        let my_height = tiles.grid[i].get_height(&noise);
        
        let neighbors = hex.all_neighbors().map(|x| match tiles.tile_at_hex(&x) {
            Some(t) => t.get_height(&noise),
            None => 0.0,
        });
        let tile_shape = TileShape::get_by_neighbors(my_height, neighbors);
        
        tiles.grid[i].set_shape(tile_shape, &prefabs);
        tiles.grid[i].spawn(&mut commands, &layout);
    }
}

fn tile_transform_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut tile_queue: Query<(&mut Transform, &Tile)>,
    noise: Res<PerlinNoise>,
) {    
    for (mut trans, tile) in &mut tile_queue {
        let height = tile.get_height(&noise);
        
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
    prefab: TilePrefab,
    entity: Option<Entity>,
}

impl Tile {
    pub fn new(hex: Hex, start_tile: TilePrefab, material: Handle<StandardMaterial>) -> Self {
        Tile {
            hex,
            material,
            prefab: start_tile,
            entity: None,
        }
    }
    
    pub fn get_height(&self, noise: &PerlinNoise) -> f32 {
        let point = [self.hex.x() as f64 / 10.0, self.hex.y() as f64 / 10.0];
        noise.perlin.get(point) as f32
    }
    
    fn set_shape(&mut self, to: TileShape, prefabs: &TileBrefabs) -> bool {
        if self.prefab.shape != to {
            if let Some(p) = prefabs.by_shape(&to) {
                self.prefab = p.clone();
                return true;
            }
        }
        false
    }
    
    fn despawn(&mut self, commands: &mut Commands) {
        if let Some(e) = self.entity {
            commands.entity(e).despawn();
        }
    }
    
    fn spawn(&mut self, commands: &mut Commands, layout: &HexLayout) {
        
        if self.entity.is_none() {
            let pos = layout.hex_to_world_pos(self.hex);
            
            let bundle = PbrBundle {
                mesh: self.prefab.mesh.clone(),
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
    Floor,
    Slope1,
    Slope2,
    Slope3,
    Slope4,
}

impl TileShape {
    fn get_by_neighbors(my_height: f32, n_heights: [f32; 6]) -> Self {
        let mut sum = 0;
        let mut down = true;
        
        for v in n_heights {
            if v > my_height {
                if down {
                    down = false;
                    sum = 0;
                }
                sum += 1;
            } else {
                down = true;
            }
        }
        
        if n_heights[5] > my_height && n_heights[0] > my_height {
            sum += 1;
        }
        
        match sum {
            1 => Self::Slope1,
            2 => Self::Slope2,
            3 => Self::Slope3,
            4 => Self::Slope4, 
            _ => Self::Floor,
        }
    }
}

#[derive(Clone)]
pub struct TilePrefab {
    pub shape: TileShape,
    pub mesh: Handle<Mesh>,
}

impl PartialEq for TilePrefab {
    fn eq(&self, other: &Self) -> bool {
        self.shape == other.shape
    }
}

impl TilePrefab {
    pub fn new(shape: TileShape, mesh: Handle<Mesh>) -> Self {
        TilePrefab { shape, mesh }
    }
}

#[derive(Resource, Clone)]
pub struct TileBrefabs (Vec<TilePrefab>);
pub type TilePrefabBuilder = TileBrefabs;

impl TileBrefabs {
    pub fn setup() -> TilePrefabBuilder {
        TileBrefabs(Vec::new())
    }
    
    pub fn add_prefab(mut self, prefab: TilePrefab) -> TilePrefabBuilder {
        self.0.push(prefab);
        self
    }
    
    pub fn build(self) -> TileBrefabs { self }
    
    pub fn first(&self) -> Option<&TilePrefab> {
        match self.0.len() > 0 {
            true => Some(&self.0[0]),
            false => None,
        }
    }
    
    pub fn by_shape(&self, shape: &TileShape) -> Option<&TilePrefab> {
        self.0.iter().find(|x| x.shape == *shape)
    }
    
    pub fn by_mesh(&self, mesh: &Handle<Mesh>) -> Option<&TilePrefab> {
        self.0.iter().find(|x| x.mesh == *mesh)
    }
}

#[derive(Resource)]
pub struct Tiles {
    layout: HexLayout, 
    prefabs: TileBrefabs, 
    pub grid: Vec<Tile>,
}

impl Tiles {
    pub fn tile_at_hex(&self, hex: &Hex) -> Option<&Tile> {
        self.grid.iter().find(|x| x.hex == *hex)
    }
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
    
    let default_prefab = TilePrefab::new(TileShape::Floor, asset_server.load("models/tiles/full.obj"));
    let prefabs = TileBrefabs::setup()
        .add_prefab(default_prefab.clone()) 
        .add_prefab(TilePrefab::new(TileShape::Slope1, asset_server.load("models/tiles/slope-1.obj"))) 
        .add_prefab(TilePrefab::new(TileShape::Slope2, asset_server.load("models/tiles/slope-2.obj"))) 
        .add_prefab(TilePrefab::new(TileShape::Slope3, asset_server.load("models/tiles/slope-3.obj"))) 
        .add_prefab(TilePrefab::new(TileShape::Slope4, asset_server.load("models/tiles/slope-4.obj"))) 
        .build();

    let grid = shapes::hexagon(Hex::ZERO, 5)
        .map(|hex| {
            let material = materials.add(Color::WHITE.into());
            let tile = Tile::new(hex, default_prefab.clone(), material);
            tile
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