mod camera;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use dungoxide::dungeon::{
    Dungeon, DungeonSize, RoomSize, DungeonConfigBuilder, TileType,
};
use dungoxide::{BinaryPartitionBuilder, RoomPlacementBuilder};

const TILES_BASIC: &str = "tiles_basic_colors.png";

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    array_texture_loader: Res<ArrayTextureLoader>
) {
    commands.spawn(Camera2dBundle::default());
    let texture_handle: Handle<Image> = asset_server.load(TILES_BASIC);

    let dungeon = DungeonConfigBuilder::new()
        .dungeon_size(DungeonSize { width: 32, height: 32 })
        .room_size(RoomSize { min_room_size: 5, max_room_size: 5 })
        .build_algorithm(BinaryPartitionBuilder)
        .should_place_doors(false)
        .build()
        .expect("Failed to build dungeon");

    // let dungeon = DungeonConfigBuilder::new()
    //     .dungeon_size(DungeonSize { width: 43, height: 50 })
    //     .room_size(RoomSize { min_room_size: 4, max_room_size: 6 })
    //     .build_algorithm(RoomPlacementBuilder)
    //     .should_place_doors(false)
    //     .build()
    //     .expect("Failed to build dungeon");

    let random_tile_map = dungeon.map;
    let map_size = TilemapSize { x: random_tile_map.len() as u32, y: random_tile_map[0].len() as u32 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y: y };
            let tile_entity = commands.spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(random_tile_map[x as usize][y as usize] as u32),
                ..Default::default()
            }).id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture{
            texture: TilemapTexture::Single(asset_server.load(TILES_BASIC)),
            tile_size,
            ..Default::default()
        })
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Dungoxide dungeon generation example"),
                    ..Default::default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, camera::movement)
        .run()
}
