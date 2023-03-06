//use std::collections::HashMap;

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_obj::*;
use hexx::shapes;
use hexx::*;
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
                .with_system(tile_swap_system)
                .with_system(tile_transform_system)
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
        let target = Transform::from_xyz(sec.sin() * 30.0, 15.0, sec.cos() * 30.0)
            .looking_at(Vec3::ZERO, Vec3::Y);
            
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
    mut tiles: ResMut<Tiles>,
) { 
    tiles.world_position += Hex::new(0, 1);
}

fn tile_swap_system(
    mut commands: Commands,
    mut tiles: ResMut<Tiles>,
    noise: Res<PerlinNoise>,
) {    
    let prefabs = tiles.prefabs.clone();
    let layout = tiles.layout.clone();
    
    for tile in &mut tiles.grid {
        tile.despawn(&mut commands);
    }
    
    for i in 0..tiles.grid.len() {
        let hex = tiles.grid[i].hex;
        //let my_height = tiles.grid[i].get_height(&noise);
        
        // let neighbors = hex.all_neighbors().map(|x| match tiles.tile_at_hex(&x) {
        //     Some(t) => //t.get_height(&noise),
        //     None => 0.0,
        // });
        let tile_shape = TileShape::get_by_neighbors(hex, &tiles, &noise);
        
        tiles.grid[i].set_shape(tile_shape, &prefabs);
        tiles.grid[i].spawn(&mut commands, &layout);
    }
}

fn tile_transform_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut tile_queue: Query<(&mut Transform, &Tile)>,
    tiles: Res<Tiles>,
    noise: Res<PerlinNoise>,
) {    
    let world_pos = &tiles.world_position;
        
    for (mut t, tile) in &mut tile_queue {
        let offset = match tile.prefab.shape == TileShape::Floor { true => -1.0, false => 0.0 };
        let height = tile.height(world_pos, &noise) + offset;
        
        t.translation = Vec3::new(t.translation.x, height * 0.5, t.translation.z); 
        t.rotation = Quat::from_rotation_y(tile.rotation());
        
        if let Some(m) = materials.get_mut(&tile.material) {
            m.base_color = Color::Rgba { red: height/2.0 + 0.5, green: height/2.0 +0.5, blue: 0.5, alpha: 1.0 }
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
    
    pub fn height(&self, world_pos: &Hex, noise: &PerlinNoise) -> f32 {
        let real_hex = Hex::new(
            world_pos.x() + self.hex.x(),
            world_pos.y() + self.hex.y()
        );
        let point = [real_hex.x() as f64 / 10.0, real_hex.y() as f64 / 10.0];
        (noise.perlin.get(point) as f32 * 3.0).round()
    }
    
    // pub fn rotation(&self) -> f32 {
    //     let dir = self.prefab.shape.direction();
    //     println!("dir: {:?}", dir);
    //     dir.angle_pointy()
    // }
    
    // fn set_shape(&mut self, to: TileShape, prefabs: &TileBrefabs) -> bool {
    //     if self.prefab.shape != to {
    //         if let Some(p) = prefabs.by_shape(&to) {
    //             self.prefab.mesh = p.mesh.clone();
    //             self.prefab.shape = to;
    //             return true;
    //         }
    //     }
    //     false
    // }
    
    fn despawn(&mut self, commands: &mut Commands) {
        if let Some(e) = self.entity {
            commands.entity(e).despawn();
        }
        self.entity = None;
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

// #[derive(Clone)]
// pub enum TileShape {
//     Floor,
//     Slope1(HexDir),
//     Slope2(HexDir),
//     Slope3(HexDir),
//     Slope4(HexDir),
// }

// impl TileShape {
//     fn direction(&self) -> HexDir {
//         match self {
//             Self::Slope1(d) => *d,
//             Self::Slope2(d) => *d,
//             Self::Slope3(d) => *d,
//             Self::Slope4(d) => *d, 
//             _ => HexDir::Top,
//         }
//     }
    
//     fn get_by_neighbors(hex: Hex, tiles: &Tiles, noise: &PerlinNoise) -> Self {
//         let world_pos = &tiles.world_position;
        
//         let mut sum = 0;
//         let mut down = true;
//         let mut dir = None;
        
//         let my_height = tiles.tile_at_hex(&hex).unwrap().height(world_pos, noise);
//         let neighbors = hex.all_neighbors().map(|x| tiles.tile_at_hex(&x));
//         let mut first = None;
        
//         for i in 0..neighbors.len() {
//             if let Some(o) = neighbors[i] {
//                 let other_height = o.height(world_pos, noise);
//                 if other_height > my_height {
//                     if down {
//                         down = false;
//                         sum = 0;
//                         if dir.is_none() {
//                             let d = hex.direction_to(o.hex);
//                             //println!("rot ({:?}): {:?}", o.hex - hex, d);
//                             dir = Some(d);
//                         }
//                     }
//                     sum += 1;
//                 } else {
//                     down = true;
//                     if first.is_some() {
//                         dir = None;
//                     }
//                 }
                
//                 if i == 0 { first = Some(other_height); }
//             }
//         }
        
//         if let Some(other_height) = first {
//             if !down && other_height > my_height {
//                 sum += 1;
//             }
//         }
        
//         if dir.is_none() {
//             dir = neighbors[0].map(|x| hex.direction_to(x.hex));
//         }
        
//         match dir {
//             Some(d) => TileShape::from_index(sum, d),
//             None => TileShape::Floor,
//         }
//     }
    
//     pub fn to_index(&self) -> usize {
//         match self {
//             Self::Floor => 0,
//             Self::Slope1(_d) => 1,
//             Self::Slope2(_d) => 2,
//             Self::Slope3(_d) => 3,
//             Self::Slope4(_d) => 4, 
//         }
//     }
    
//     pub fn from_index(idx: usize, dir: HexDir) -> Self {
//         match idx {
//             1 => Self::Slope1(dir),
//             2 => Self::Slope2(dir),
//             3 => Self::Slope3(dir),
//             4 => Self::Slope4(dir), 
//             _ => Self::Floor,
//         }
//     }
//}

// impl PartialEq for TileShape {
//     fn eq(&self, other: &Self) -> bool {
//         self.to_index() == other.to_index()
//     }
// }

#[derive(Clone)]
pub struct TilePrefab {
    //pub shape: TileShape,
    pub mesh: Handle<Mesh>,
}

// impl PartialEq for TilePrefab {
//     fn eq(&self, other: &Self) -> bool {
//         self.shape == other.shape
//     }
// }

// impl TilePrefab {
//     pub fn new(shape: TileShape, mesh: Handle<Mesh>) -> Self {
//         TilePrefab { shape, mesh }
//     }
// }

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
    
    // pub fn by_shape(&self, shape: &TileShape) -> Option<&TilePrefab> {
    //     self.0.iter().find(|x| x.shape == *shape)
    // }
    
    pub fn by_mesh(&self, mesh: &Handle<Mesh>) -> Option<&TilePrefab> {
        self.0.iter().find(|x| x.mesh == *mesh)
    }
}

#[derive(Resource)]
pub struct Tiles {
    layout: HexLayout, 
    prefabs: TileBrefabs, 
    pub grid: Vec<Tile>,
    pub world_position: Hex, 
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
    
    let default_prefab = TilePrefab{ mesh: asset_server.load("models/tiles/full.obj") };
    //let default_prefab = TilePrefab::new(TileShape::Floor, asset_server.load("models/tiles/full.obj"));
    // let prefabs = TileBrefabs::setup()
    //     .add_prefab(default_prefab.clone()) 
    //     .add_prefab(TilePrefab::new(TileShape::Slope1(HexDir::Top), asset_server.load("models/tiles/slope-1.obj"))) 
    //     .add_prefab(TilePrefab::new(TileShape::Slope2(HexDir::TopRight), asset_server.load("models/tiles/slope-2.obj"))) 
    //     .add_prefab(TilePrefab::new(TileShape::Slope3(HexDir::Top), asset_server.load("models/tiles/slope-3.obj"))) 
    //     .add_prefab(TilePrefab::new(TileShape::Slope4(HexDir::TopRight), asset_server.load("models/tiles/slope-4.obj"))) 
    //     .build();

    let grid = shapes::hexagon(Hex::ZERO, 5)
        .map(|hex| {
            let material = materials.add(Color::WHITE.into());
            let tile = Tile::new(hex, default_prefab.clone(), material);
            tile
        })
        .collect();
        
    commands.insert_resource(Tiles {layout, grid, world_position: Hex::ZERO });
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