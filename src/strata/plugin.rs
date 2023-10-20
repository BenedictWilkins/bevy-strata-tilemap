use bevy::core_pipeline::core_2d::Transparent2d;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::PrepareAssetSet;
use bevy::render::render_phase::AddRenderCommand;
use bevy::render::render_resource::SpecializedRenderPipelines;
use bevy::render::{Render, RenderApp, RenderSet};
use bevy::sprite::Mesh2dHandle;

use bevy::render::render_resource::{FilterMode, SamplerDescriptor};
use bevy::render::texture::ImageSampler;
use bevy::utils::HashSet;

use crate::extract::extract_fast_tilemap;
use crate::map::apply_map_transforms;
use crate::pipeline::MapPipeline;
use crate::prepare::prepare_fast_tilemap;
use crate::queue::{queue_fast_tilemap, DrawMap};
use crate::shader::{SHADER_CODE, SHADER_HANDLE};

use super::prelude::Chunk;

/// Plugin for fast strata tile.
/// Add this to you app and then spawn one or multiple maps use [`crate::map_builder::MapBuilder`].
#[derive(Default)]
pub struct StrataTileMapPlugin<T: States> {
    /// some intialisation of this plugin will occur OnExit of this game state.
    loading_game_state: T,
}

impl<T: States> StrataTileMapPlugin<T> {
    pub fn new(loading_game_state: T) -> Self {
        return StrataTileMapPlugin {
            loading_game_state: loading_game_state,
        };
    }
}

impl<T: States> Plugin for StrataTileMapPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(self.loading_game_state.clone()),
            configure_loaded_atlases,
        );
        app.add_systems(Update, apply_map_transforms);

        let mut shaders = app.world.resource_mut::<Assets<Shader>>();
        // TODO set this path properly...
        shaders.set_untracked(SHADER_HANDLE, Shader::from_wgsl(SHADER_CODE, ""));

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(err) => panic!("Failed to initialised render app: {:?}", err),
        };

        render_app
            .add_render_command::<Transparent2d, DrawMap>()
            .add_systems(ExtractSchedule, extract_fast_tilemap)
            .add_systems(
                Render,
                (
                    prepare_fast_tilemap
                        .in_set(RenderSet::Prepare)
                        .after(PrepareAssetSet::PreAssetPrepare),
                    queue_fast_tilemap.in_set(RenderSet::Queue),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };
        render_app
            .init_resource::<MapPipeline>()
            .init_resource::<SpecializedRenderPipelines<MapPipeline>>();
    }
}

/// this is a system taken from bevy_fast_tilemap but adapted to use stratas asset loading strategy.
pub fn configure_loaded_atlases(chunks: Query<&Chunk>, mut images: ResMut<Assets<Image>>) {
    let atlases: HashSet<&Handle<Image>> =
        chunks.iter().map(|chunk| &chunk.atlas_texture).collect();

    for atlas in atlases.iter() {
        // Set some sampling options for the atlas texture for nicer looks, such as avoiding "grid lines" when zooming out or mushy edges.
        if let Some(atlas) = images.get_mut(&atlas) {
            atlas.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
                // min_filter of linear gives undesired grid lines when zooming out
                min_filter: FilterMode::Nearest,
                // mag_filter of linear gives mushy edges on tiles in closeup which is usually not what we want
                mag_filter: FilterMode::Nearest,
                mipmap_filter: FilterMode::Linear,
                ..default()
            });
            if let Some(ref mut view_descriptor) = atlas.texture_view_descriptor {
                view_descriptor.mip_level_count = Some(4);
            }
        }
    }
}
