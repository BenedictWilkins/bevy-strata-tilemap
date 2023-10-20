use std::mem::size_of;

use crate::{map_uniform::MapUniform, tile_projection};

use super::prelude::*;
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

pub trait ChunkBuilder {
    fn new(map_size: UVec2, tile_size: Vec2, atlas_texture: Handle<Image>) -> Self;

    fn build<F>(self, images: &mut ResMut<Assets<Image>>, initializer: F) -> Chunk
    where
        F: FnMut(UVec2) -> u16;
}

impl ChunkBuilder for Chunk {
    fn new(map_size: UVec2, tile_size: Vec2, atlas_texture: Handle<Image>) -> Self {
        let mut chunk = Chunk {
            atlas_texture,
            map_uniform: MapUniform {
                map_size,
                tile_size,
                ..default()
            },
            ..default()
        };
        chunk.map_uniform.projection = tile_projection::IDENTITY.projection;
        chunk.map_uniform.tile_anchor_point = tile_projection::IDENTITY.tile_anchor_point;
        return chunk;
    }

    fn build<F>(mut self, images: &mut ResMut<Assets<Image>>, mut initializer: F) -> Chunk
    where
        F: FnMut(UVec2) -> u16, //F: FnOnce(&mut ChunkIndexer),
    {
        let mut map_image = Image::new(
            Extent3d {
                width: self.map_size().x,
                height: self.map_size().y,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            vec![0u8; (self.map_size().x * self.map_size().y) as usize * size_of::<u16>()],
            TextureFormat::R16Uint,
        );
        map_image.texture_descriptor.usage = TextureUsages::STORAGE_BINDING
            | TextureUsages::COPY_DST
            | TextureUsages::TEXTURE_BINDING;
        map_image.texture_descriptor.mip_level_count = 1;

        // initialise values in the chunk
        let sx = self.map_size().x;
        let sy = self.map_size().y;
        let mut _initializer = |m: &mut ChunkIndexer| {
            for y in 0..sy {
                for x in 0..sx {
                    m.set(x, y, initializer(UVec2::new(x, y)));
                }
            }
        };
        _initializer(&mut ChunkIndexer {
            image: &mut map_image,
            size: self.map_uniform.map_size,
        });

        // this should be loaded before building using strata asset loading strategy
        let atlas_texture = images.get(&self.atlas_texture).unwrap();
        self.map_uniform.update_atlas_size(atlas_texture.size());

        self.map_texture = images.add(map_image);
        self.map_uniform.update_inverse_projection();
        self.map_uniform.update_world_size();

        self
    }
}
