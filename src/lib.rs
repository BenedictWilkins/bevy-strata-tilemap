//! GPU-accelerated tilemap functionality for bevy.
//! Aims at rendering tilemaps with lightning speed by using just a single quad per map (layer)
//! and offloading the actual rendering to GPU.
//! This should be faster than most other bevy tilemap implementations as of this writing.
//!
//! ## Features
//!
//! - Very high rendering performance (hundreds of fps, largely independent of map size)
//! - Tilemaps can be very large or have many "layers"
//! - Rectangular and isometric (axonometric) tile maps.
//! - Tile overlaps either by "dominance" rule or by perspective
//! - Optional custom mesh for which the map serves as a texture
//!
//! ## How it works
//!
//! The principle is probably not new but nonetheless quite helpful: The whole tilemap (-layer) is
//! rendered as a single quad and a shader cares for rendering the correct tiles at the correct
//! position.

pub mod bundle;
pub mod extract;
pub mod map;
pub mod map_builder;
pub mod map_uniform;
pub mod pipeline;
pub mod plugin;
pub mod prepare;
pub mod queue;
pub mod shader;
pub mod tile_projection;

// new strata code.
pub mod strata;

pub mod prelude {
    pub use crate::bundle::MapBundle;
    pub use crate::map::{Map, MapIndexer, MapReadyEvent, MeshManagedByMap};
    pub use crate::plugin::FastTileMapPlugin;
    pub use crate::tile_projection::{TileProjection, AXONOMETRIC, IDENTITY};
}

pub use crate::bundle::MapBundle;
pub use crate::map::{Map, MapIndexer, MapReadyEvent, MeshManagedByMap};
pub use crate::plugin::FastTileMapPlugin;
pub use crate::tile_projection::{TileProjection, AXONOMETRIC, IDENTITY};
