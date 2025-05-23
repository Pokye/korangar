use std::sync::Arc;

use cgmath::{Deg, Point3};
use ragnarok_formats::map::{GroundData, GroundTile, WaterSettings};
use wgpu::{Device, Queue};

use super::{GROUND_TILE_SIZE, create_index_buffer, create_vertex_buffer};
use crate::graphics::{Texture, WaterVertex};
use crate::loaders::{ImageType, TextureLoader};
use crate::world::WaterPlane;

pub fn generate_water_plane(
    device: &Device,
    queue: &Queue,
    resource_file: &str,
    texture_loader: &TextureLoader,
    ground_data: &GroundData,
    water_settings: Option<&WaterSettings>,
) -> Option<WaterPlane> {
    let water_settings = water_settings?;

    let water_level = -water_settings.water_level.unwrap_or(0.0);
    let water_type = water_settings.water_type.unwrap_or(1);
    let wave_height = water_settings.wave_height.unwrap_or(1.0);
    let wave_speed = Deg(water_settings.wave_speed.unwrap_or(2.0));
    let wave_pitch = Deg(water_settings.wave_pitch.unwrap_or(50.0));
    let texture_cycling_interval = water_settings.texture_cycling_interval.unwrap_or(3);

    let max_water_height = water_level + wave_height;

    let (water_vertices, water_indices) = generate_vertices(&ground_data.ground_tiles, ground_data.width, water_level, max_water_height);

    if water_vertices.is_empty() {
        return None;
    }

    let vertex_buffer = Arc::new(create_vertex_buffer(
        device,
        queue,
        resource_file,
        "map water vertices",
        &water_vertices,
    ));

    let index_buffer = Arc::new(create_index_buffer(
        device,
        queue,
        resource_file,
        "map water indices",
        &water_indices,
    ));

    let water_opacity = match water_type {
        4 | 6 => 1.0,
        _ => 144.0 / 255.0,
    };

    let texture_repeat = match water_type {
        4 | 6 => 16.0,
        _ => 4.0,
    };

    let textures: Vec<Arc<Texture>> = (0..32)
        .map(|index| format!("워터\\water{}{:02}.jpg", water_type, index))
        .map(|path| {
            texture_loader
                .get_or_load(&path, ImageType::Color)
                .expect("Can't load water texture")
        })
        .collect();

    Some(WaterPlane::new(
        water_opacity,
        wave_height,
        wave_speed,
        wave_pitch,
        texture_cycling_interval,
        texture_repeat,
        textures,
        vertex_buffer,
        index_buffer,
    ))
}

fn generate_vertices(ground_tiles: &[GroundTile], width: i32, water_level: f32, max_water_height: f32) -> (Vec<WaterVertex>, Vec<u32>) {
    let mut vertices = Vec::new();

    let indices = ground_tiles
        .iter()
        .enumerate()
        // We only generated vertices if the lowest point of the tile is submerged.
        .filter(|(_, current_tile)| current_tile.lowest_point() < max_water_height)
        .flat_map(|(index, _)| {
            let grid_u = index as i32 % width;
            let grid_v = index as i32 / width;

            let south_west = Point3::new(grid_u as f32 * GROUND_TILE_SIZE, water_level, grid_v as f32 * GROUND_TILE_SIZE);
            let south_east = Point3::new(
                (grid_u + 1) as f32 * GROUND_TILE_SIZE,
                water_level,
                grid_v as f32 * GROUND_TILE_SIZE,
            );
            let north_west = Point3::new(
                grid_u as f32 * GROUND_TILE_SIZE,
                water_level,
                (grid_v + 1) as f32 * GROUND_TILE_SIZE,
            );
            let north_east = Point3::new(
                (grid_u + 1) as f32 * GROUND_TILE_SIZE,
                water_level,
                (grid_v + 1) as f32 * GROUND_TILE_SIZE,
            );

            let index = vertices.len() as u32;

            vertices.push(WaterVertex::new(south_west, grid_u, grid_v));
            vertices.push(WaterVertex::new(south_east, grid_u, grid_v));
            vertices.push(WaterVertex::new(north_west, grid_u, grid_v));
            vertices.push(WaterVertex::new(north_east, grid_u, grid_v));

            [index, index + 1, index + 2, index + 1, index + 3, index + 2]
        })
        .collect();

    (vertices, indices)
}

pub trait GroundTileExt {
    fn lowest_point(&self) -> f32;
}

impl GroundTileExt for GroundTile {
    fn lowest_point(&self) -> f32 {
        [
            -self.southeast_corner_height,
            -self.southwest_corner_height,
            -self.northeast_corner_height,
            -self.northwest_corner_height,
        ]
        .into_iter()
        .reduce(f32::min)
        .unwrap()
    }
}
