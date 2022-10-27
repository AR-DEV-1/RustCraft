use crate::services::asset::material::chunk::ChunkMaterial;
use crate::services::asset::AssetService;
use crate::{
    default, info, shape, Aabb, App, Assets, ChunkData, Color, Commands, Handle, Mesh, Mut,
    PbrBundle, Plugin, RawChunkData, RerenderChunkFlag, ResMut, StandardMaterial,
    TextureAtlasSprite, Transform, Vec3,
};
use bevy::prelude::MaterialMeshBundle;
use fnv::{FnvBuildHasher, FnvHashMap};
use nalgebra::Vector3;
use rustcraft_protocol::constants::CHUNK_SIZE;
use rustcraft_protocol::protocol::clientbound::chunk_update::PartialChunkUpdate;
use std::collections::HashMap;

pub mod data;
pub mod lookup;
pub mod systems;

pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.world
            .resource_scope(|world, mut materials: Mut<Assets<StandardMaterial>>| {
                world.insert_resource(ChunkService::new(&mut *materials));
            });
    }
}

pub struct ChunkService {
    pub chunks: HashMap<Vector3<i32>, ChunkData, FnvBuildHasher>,
    default_material: Handle<StandardMaterial>,
}

impl ChunkService {
    pub fn new(materials: &mut Assets<StandardMaterial>) -> ChunkService {
        let default_material = materials.add(Color::rgb(0.3, 0.3, 0.3).into());

        ChunkService {
            chunks: FnvHashMap::default(),
            default_material,
        }
    }

    /// Loads a chunk from network into the game by creating an entity and ChunkData entry
    pub fn load_chunk(
        &mut self,
        position: Vector3<i32>,
        data: &PartialChunkUpdate,
        commands: &mut Commands,
        asset_service: &AssetService,
        materials: &mut ResMut<Assets<ChunkMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        let entity = commands
            .spawn_bundle(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 0.0 })),
                material: asset_service.texture_atlas_material.clone(),
                transform: Transform::from_translation(Vec3::new(
                    (position.x * CHUNK_SIZE as i32) as f32,
                    (position.y * CHUNK_SIZE as i32) as f32,
                    (position.z * CHUNK_SIZE as i32) as f32,
                )),
                ..default()
            })
            .insert(RerenderChunkFlag { chunk: position })
            //TODO: Remove once bevy has fixed its shitty AABB generation
            .insert(Aabb::from_min_max(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(16.0, 16.0, 16.0),
            ))
            .id();

        self.chunks
            .insert(position, ChunkData::new(data.data, entity, position));
    }

    /// Creates a new chunk from data
    pub fn create_chunk(
        &mut self,
        position: Vector3<i32>,
        data: RawChunkData,
        commands: &mut Commands,
        asset_service: &AssetService,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) {
        let entity = commands
            .spawn_bundle(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 0.0 })),
                material: asset_service.texture_atlas_material.clone(),
                transform: Transform::from_translation(Vec3::new(
                    (position.x * CHUNK_SIZE as i32) as f32,
                    (position.y * CHUNK_SIZE as i32) as f32,
                    (position.z * CHUNK_SIZE as i32) as f32,
                )),
                ..default()
            })
            .insert(RerenderChunkFlag { chunk: position })
            //TODO: Remove once bevy has fixed its shitty AABB generation
            .insert(Aabb::from_min_max(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(16.0, 16.0, 16.0),
            ))
            .id();

        let chunk = ChunkData::new(data, entity, position);

        self.chunks.insert(position, chunk);
    }
}
