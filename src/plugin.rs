use crate::{
    map::{apply_map_transforms, configure_loaded_assets, update_loading_maps, MapReadyEvent},
    pipeline::MapPipeline,
};
use bevy::{
    core_pipeline::core_2d::Transparent2d,
    prelude::*,
    render::{
        render_asset::PrepareAssetSet, render_phase::AddRenderCommand,
        render_resource::SpecializedRenderPipelines, Render, RenderApp, RenderSet,
    },
};

use crate::{
    extract::extract_fast_tilemap,
    prepare::prepare_fast_tilemap,
    queue::{queue_fast_tilemap, DrawMap},
    shader::{SHADER_CODE, SHADER_HANDLE},
};

/// Plugin for fast tilemap.
/// Add this to you app and then spawn one or multiple maps use [`crate::map_builder::MapBuilder`].
#[derive(Default)]
pub struct FastTileMapPlugin;

impl Plugin for FastTileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MapReadyEvent>().add_systems(
            Update,
            (configure_loaded_assets, update_loading_maps).chain(),
        );
        app.add_systems(Update, apply_map_transforms);

        let mut shaders = app.world.resource_mut::<Assets<Shader>>();
        shaders.set_untracked(
            SHADER_HANDLE,
            Shader::from_wgsl(SHADER_CODE, "../assets/tilemap_shader.wgsl"),
        );

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
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
