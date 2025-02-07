use crate::systems::asset::atlas::atlas::TextureAtlas;
use crate::systems::asset::atlas::resource_packs::ResourcePacks;
use crate::systems::asset::material::chunk::ChunkMaterial;
use crate::systems::asset::AssetService;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use crate::systems::ui::loading::LoadingData;
use fnv::FnvBuildHasher;
use image::{DynamicImage, GenericImage};
use std::collections::HashMap;
use std::ffi::OsString;

pub mod atlas;
pub mod index;
pub mod resource_packs;

#[derive(Debug, PartialEq, Eq, Resource)]
pub enum AtlasLoadingStage {
    AwaitingIndex,
    AwaitingPack,
    Done,
}

/// The images that make up a resource pack
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "7b14806a-672b-423b-8d16-4f18afefa463"]
pub struct ResourcePackData {
    images: HashMap<String, DynamicImage, FnvBuildHasher>,
}

impl ResourcePackData {
    pub fn new(images: HashMap<String, DynamicImage, FnvBuildHasher>) -> ResourcePackData {
        ResourcePackData { images }
    }
}

pub fn load_resource_zips(
    packs: Res<Assets<ResourcePacks>>,
    mut service: ResMut<AssetService>,
    server: Res<AssetServer>,
    mut stage: ResMut<AtlasLoadingStage>,
) {
    // Only load zips on change to resource packs
    if *stage != AtlasLoadingStage::AwaitingIndex || packs.len() == 0 {
        return;
    }
    if !packs.is_changed() {
        return;
    }

    let packs = packs.get(&service.resource_packs).unwrap();

    let pack = packs.get_default();

    if !pack.path.extension().unwrap().eq(&OsString::from("pack")) {
        error!(
            "Resource pack {:?} does not end with .pack, aborting load.",
            pack.path
        );
        return;
    }

    service.pack = Some(server.load(pack.path.clone()));

    *stage = AtlasLoadingStage::AwaitingPack;
}

pub fn build_texture_atlas(
    packs: Res<Assets<ResourcePacks>>,
    mut data: ResMut<Assets<ResourcePackData>>,
    mut service: ResMut<AssetService>,
    mut stage: ResMut<AtlasLoadingStage>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut loading: ResMut<LoadingData>,
) {
    if *stage != AtlasLoadingStage::AwaitingPack
        || data.len() == 0
        || *stage == AtlasLoadingStage::Done
    {
        return;
    }

    // Fetch the resources required to build the texture atlas
    let pack = packs.get(&service.resource_packs).unwrap().get_default();
    let textures = data.get_mut(service.pack.as_ref().unwrap());

    let textures = match textures {
        None => return,
        Some(val) => val,
    };

    // Build the texture atlas
    let atlas = TextureAtlas::new(pack, &mut textures.images, &mut images);

    info!("Generated texture atlas");
    service.texture_atlas = Some(atlas);

    // Create a new material
    materials.set(
        &service.opaque_texture_atlas_material,
        ChunkMaterial {
            color: Color::WHITE,
            color_texture: Some(
                images.get_handle(service.texture_atlas.as_ref().unwrap().get_image()),
            ),
            alpha_mode: AlphaMode::Opaque,
        },
    );

    materials.set(
        &service.translucent_texture_atlas_material,
        ChunkMaterial {
            color: Color::WHITE,
            color_texture: Some(
                images.get_handle(service.texture_atlas.as_ref().unwrap().get_image()),
            ),
            alpha_mode: AlphaMode::Blend,
        },
    );

    *stage = AtlasLoadingStage::Done;
    loading.texture_atlas = true;
}
