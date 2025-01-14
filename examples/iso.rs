//! Simple example for illustrating axonometrically projected tilemaps.
//! To keep the math simple instead of strictly isometric, we stick to a projection
//! where each tile ends up a diamond shape that is twice as wide as high.

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{uvec2, vec2};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_fast_tilemap::{
    FastTileMapPlugin, Map, MapBundle, MapIndexer, MeshManagedByMap, AXONOMETRIC,
};

mod mouse_controls_camera;
use mouse_controls_camera::MouseControlsCameraPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Fast Tilemap example"),
                    resolution: (1820., 920.).into(),
                    // disable vsync so we can see the raw FPS speed
                    present_mode: PresentMode::Immediate,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            MouseControlsCameraPlugin::default(),
            FastTileMapPlugin::default(),
        ))
        .add_systems(Startup, startup)
        .add_systems(Update, highlight_hovered)
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Note that tile index 0 is used to draw tiles that are outside
    // the logical map (but inside the rectangular map bounding box).
    // In iso.png we chose a dotted outline to make this visible,
    // in practice you might prefer a transparent tile here or one
    // that can serve as some sort of background to your map.
    let map = Map::builder(
        // Map size
        uvec2(23, 57),
        // Tile atlas texture
        asset_server.load("iso.png"),
        // Tile size
        vec2(40., 20.),
    )
    .with_projection(AXONOMETRIC)
    // Build the map is to provide an initializer callback here.
    .build_and_initialize(&mut images, reset_map);

    commands
        .spawn(MapBundle::new(map))
        // Have the map manage our mesh so it always has the right size
        .insert(MeshManagedByMap);
} // startup

/// Fill the map with a chessboard pattern.
fn reset_map(m: &mut MapIndexer) {
    for y in 0..m.size().y {
        for x in 0..m.size().x {
            m.set(x, y, (((x + y) % 2) + 1) as u16);
        }
    }
} // reset_map

/// Highlight the currently hovered tile red, reset all other tiles
fn highlight_hovered(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut camera_query: Query<(&GlobalTransform, &Camera), With<OrthographicProjection>>,
    maps: Query<&Map>,
    mut images: ResMut<Assets<Image>>,
) {
    for event in cursor_moved_events.iter() {
        for map in maps.iter() {
            for (global, camera) in camera_query.iter_mut() {
                // Translate viewport coordinates to world coordinates
                if let Some(world) = camera
                    .viewport_to_world(global, event.position)
                    .map(|ray| ray.origin.truncate())
                {
                    // Modifying the map requires that the underlying texture be synchronized to
                    // the GPU again so you want to avoid to do this every frame if your map is
                    // very large. The transfer cost does not depend on how much you change, so
                    // you may as well generate the whole thing (of course consider the actual
                    // generation time).
                    //
                    // Note that this technically does *not* modify the `Map` component, but
                    // teh underlying texture which is a plain old Image Asset.
                    if let Ok(mut m) = map.get_mut(&mut *images) {
                        // The map can convert between world coordinates and map coordinates
                        let coord = map.world_to_map(world);
                        println!("Map coordinate: {:?}", coord);

                        let coord = coord
                            .as_uvec2()
                            .clamp(uvec2(0, 0), map.map_size() - uvec2(1, 1));

                        reset_map(&mut m);
                        m.set_uvec(coord, 3u16);
                    }
                } // if Some(world)
            } // for (global, camera)
        } // for map
    } // for event
} // highlight_hovered
