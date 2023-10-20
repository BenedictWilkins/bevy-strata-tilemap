/// this file is for code specific to the Strata game. It optimise some of the setup using Strata's asset loading strategy.
use bevy::{prelude::*, render::primitives::Aabb, sprite::Mesh2dHandle};

pub mod prelude {

    pub type Chunk = crate::Map;
    //pub type MeshManagedByChunk = bevy_strata_tilemap::MeshManagedByMap;
    pub type ChunkIndexer<'a> = crate::MapIndexer<'a>;
    pub type ChunkLoading = crate::map::MapLoading;

    //pub use super::ChunkBundle;
    //pub use super::StrataTileMapPlugin;
}

use prelude::*;

#[derive(Bundle, Clone, Default)]
pub struct ChunkBundle {
    // these are both present in the origin bevy_fast_tilemap crate, but strata uses a different asset/map loading strategy.
    // they are not required as long as StrataTileMapPlugin is used.
    //mesh_managed_by: MeshManagedByChunk,
    //loading: ChunkLoading,
    pub chunk: Chunk,
    pub transform: Transform,
    mesh: Mesh2dHandle,
    global_transform: GlobalTransform,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
    aabb: Aabb, // this is required to check whether the chunk is visible in in the camera viewport
}

impl ChunkBundle {
    pub fn new(chunk: Chunk, transform: Transform) -> Self {
        Self {
            chunk,
            transform,
            ..default()
        }
    }

    pub fn with_mesh(mut self, meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        // create mesh for chunk
        let mesh = Mesh::from(shape::Quad {
            size: self.chunk.world_size(),
            flip: false,
        });
        // create aabb for chunk
        if let Some(aabb) = mesh.compute_aabb() {
            self.aabb = aabb;
        } else {
            panic!("Failed to create aabb for chunk mesh.");
        }
        // add mesh to assets
        let mesh_handle = Mesh2dHandle(meshes.add(mesh));
        self.mesh = mesh_handle;
        return self;
    }
}

pub fn initialise_chunk_meshes(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    mut chunks: Query<(Entity, &mut Chunk)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, mut chunk) in &mut chunks {
        println!("CHUNK: {:?}", entity);
        assert!(chunk.is_loaded(images.as_ref())); // sanity check, all assets should have loaded by now!

        // update the chunk to its new size/set uniform values -- "update" is such a bad name for this -__-
        if !chunk.update(images.as_ref()) {
            panic!()
        }

        // create mesh for chunk
        let mesh = Mesh::from(shape::Quad {
            size: chunk.world_size(),
            flip: false,
        });
        // create aabb for chunk
        if let Some(aabb) = mesh.compute_aabb() {
            commands.entity(entity).insert(aabb);
        } else {
            warn!("Failed to create Aabb for chunk: {entity:?} mesh.");
        }
        // add mesh to assets
        let mesh_handle = Mesh2dHandle(meshes.add(mesh));
        commands.entity(entity).insert(mesh_handle);
    }
}
