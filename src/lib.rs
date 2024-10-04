mod binary_partition_builder;
mod room_placement_builder;

pub mod dungeon;
pub use binary_partition_builder::BinaryPartitionBuilder;
pub use room_placement_builder::RoomPlacementBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use super::dungeon::*;

    #[test]
    fn test_default_build() {
        let dungeon = DungeonConfigBuilder::new()
            .build_algorithm(BinaryPartitionBuilder).build().expect("Failed to build dungeon");
        assert_eq!(dungeon.map.len(), 32);
        assert_eq!(dungeon.map[0].len(), 32);
    }

    #[test]
    fn test_custom_size() {
        let dungeon = DungeonConfigBuilder::new()
            .build_algorithm(BinaryPartitionBuilder)
            .dungeon_size(DungeonSize { width: 60, height: 40 })
            .build()
            .expect("Failed to build dungeon");
        assert_eq!(dungeon.map.len(), 40);
        assert_eq!(dungeon.map[0].len(), 60);
    }

    #[test]
    fn test_min_room_size_greater_than_max_room_size() {
        let result = DungeonConfigBuilder::new()
            .build_algorithm(BinaryPartitionBuilder)
            .room_size(RoomSize { min_room_size: 10, max_room_size: 5 })
            .build();

        assert!(matches!(
            result,
            Err(DungeonBuildError::InvalidRoomSize(_))
        ));
    }

    #[test]
    fn test_zero_width() {
        let result = DungeonConfigBuilder::new()
            .build_algorithm(BinaryPartitionBuilder)
            .dungeon_size(DungeonSize { width: 0, height: 50 })
            .build();

        assert!(matches!(
            result,
            Err(DungeonBuildError::InvalidSize(_))
        ));
    }

    #[test]
    fn test_zero_height() {
        let result = DungeonConfigBuilder::new()
            .build_algorithm(BinaryPartitionBuilder)
            .dungeon_size(DungeonSize { width: 50, height: 0 })
            .build();

        assert!(matches!(
            result,
            Err(DungeonBuildError::InvalidSize(_))
        ));
    }

    #[test]
    fn test_zero_min_room_size() {
        let result = DungeonConfigBuilder::new()
            .build_algorithm(BinaryPartitionBuilder)
            .room_size(RoomSize { min_room_size: 0, max_room_size: 5 })
            .build();

        assert!(matches!(
            result,
            Err(DungeonBuildError::InvalidRoomSize(_))
        ));
    }

    #[test]
    fn test_zero_max_room_size() {
        let result = DungeonConfigBuilder::new()
            .build_algorithm(BinaryPartitionBuilder)
            .room_size(RoomSize { min_room_size: 1, max_room_size: 0 })
            .build();

        assert!(matches!(
            result,
            Err(DungeonBuildError::InvalidRoomSize(_))
        ));
    }

    #[test]
    fn test_room_too_large_for_dungeon() {
        let result = DungeonConfigBuilder::new()
            .build_algorithm(BinaryPartitionBuilder)
            .dungeon_size(DungeonSize { width: 10, height: 10 })
            .room_size(RoomSize { min_room_size: 15, max_room_size: 16 })
            .build();

        assert!(matches!(
            result,
            Err(DungeonBuildError::RoomTooLargeForDungeon)
        ));
    }

    #[test]
    fn test_no_rooms_created_partition() {
        let result = DungeonConfigBuilder::new()
            .build_algorithm(BinaryPartitionBuilder)
            .dungeon_size(DungeonSize { width: 1, height: 1 })
            .room_size(RoomSize { min_room_size: 1, max_room_size: 2 })
            .build();
        assert!(matches!(result, Err(DungeonBuildError::NoRoomsCreated)));
    }

    #[test]
    fn test_no_rooms_created_room_placement() {
        let result = DungeonConfigBuilder::new()
            .build_algorithm(RoomPlacementBuilder)
            .dungeon_size(DungeonSize { width: 5, height: 5 })
            .room_size(RoomSize { min_room_size: 4, max_room_size: 5 })
            .build();
        assert!(matches!(result, Err(DungeonBuildError::NoRoomsCreated)));
    }

    #[test]
    fn test_disable_doors() {
        let dungeon = DungeonConfigBuilder::new()
            .build_algorithm(BinaryPartitionBuilder)
            .should_place_doors(false)
            .build()
            .expect("Failed to build dungeon");

        let door_count = dungeon
            .map
            .iter()
            .flatten()
            .filter(|&&tile| tile == TileType::Door)
            .count();
        assert_eq!(door_count, 0);
    }

    #[test]
    fn test_room_placement_algorithm() {
        let dungeon = DungeonConfigBuilder::new()
            .build_algorithm(RoomPlacementBuilder)
            .build()
            .expect("Failed to build dungeon");
        assert!(!dungeon.map.is_empty());
    }
}
