mod chunk;
mod chunk_builder;
mod plugin;

pub mod prelude {

    pub type Chunk = crate::Map;
    pub type ChunkIndexer<'a> = crate::MapIndexer<'a>;
    pub type ChunkLoading = crate::map::MapLoading;

    pub use crate::strata::chunk::ChunkBundle;
    pub use crate::strata::chunk_builder::ChunkBuilder;
    pub use crate::strata::plugin::StrataTileMapPlugin;
}
