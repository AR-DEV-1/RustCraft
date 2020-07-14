use crate::block::Block;
use crate::services::chunk_service::chunk::{ChunkBlockData};
use crate::services::settings_service::CHUNK_SIZE;
use nalgebra::{Vector3};
use noise::{NoiseFn, Perlin, Seedable};

// This file is temporary, until we can connect to a server I need to have chunk generation client side

pub struct World {}

impl World {
    pub fn generate_chunk(chunk_pos: Vector3<i32>, blocks: &Vec<Block>) -> Option<ChunkBlockData> {
        let scale = 1.0 / CHUNK_SIZE as f64;
        let mut chunk_nothing = true;

        let noise_map = Perlin::new();
        noise_map.set_seed(0);

        let mut chunk = [[[0 as u32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        let blocks: Vec<Block> = (*blocks).to_vec();

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for y_offset in 0..CHUNK_SIZE {
                    let y = (chunk_pos.y * CHUNK_SIZE as i32) + y_offset as i32;
                    let height_map = noise_map.get([
                        (x as f64 * scale) + chunk_pos.x as f64,
                        (z as f64 * scale) + chunk_pos.z as f64,
                    ]);
                    let height = (height_map * 5.0).round() as i32 + 50;
                    //let height = 52;

                    //Stone
                    if y < height {
                        chunk[x][y_offset][z] = 1;
                        chunk_nothing = false;
                    // Dirt
                    } else if y <= (height + 1) {
                        chunk[x][y_offset][z] = 2;
                        chunk_nothing = false;
                    } else if y == (height + 2) {
                        chunk[x][y_offset][z] = 3;
                        chunk_nothing = false;

                        // Please ignore
                        if rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true  {
                            chunk[x][y_offset][z] = 6;
                        }

                        if rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true &&
                            rand::random::<bool>() == true  {
                            chunk[x][y_offset][z] = 5;
                        }
                    }
                }
            }
        }

        if chunk_nothing {
            None
        } else {
            Some((chunk, blocks))
        }
    }
}
