# dungoxide
Library for random dungeon generations.
## Overview
This project is a Dungeon Generator, designed to create random dungeon layouts for games such as roguelikes or dungeon crawlers. It provides a flexible API using the builder pattern and it provides dungeon builder trait to allow generic generation algorithm implementation. It allows developers to customize dungeon generation parameters easily.

## Purpose
The primary purpose of this project is to generate procedurally generated dungeons that can be integrated into games or used for testing algorithms related to pathfinding, game AI, or procedural content generation.

## Features
- **Customize Dungeon Size:** Set the width and height of the dungeon.
- **Configure Room Sizes:** Define minimum and maximum room sizes.
- **Select algorithms:** Choose between dungeon algorithms:
  - **Binary partition:** Recursively splits dungeon into sub-rooms
  - **Room placement algorithm:** Randomly placing rooms into dungeon using Kruskal algorithm
- **Error handling:** Validate configurations and provides descriptive errors for invalid parameters

## How It Works

### DungeonBuilder API
Central part of the project is a **DungeonBuilder** trait, which allows to define dungeon building procedure.
It provides DungeonBuildConfig and implementor of the trait should return **Dungeon** which represents a 2d map of tiles.

```Rust
pub trait DungeonBuilder {
    fn build(self, build_config: DungeonBuildConfig) -> Result<Dungeon, DungeonBuildError>;
}
```

```Rust
#[repr(u32)]
pub enum TileType {
    Door = 1,
    Wall = 4,
    Floor = 5,
}

pub struct Dungeon {
    pub map: Vec<Vec<TileType>>,
}
```
**TileType** is represented by u32 so it is easier to map to tile in the rendering engine like bevy.

### DungeonConfigBuilder<BuilderAlgorithm>

Another important part of the library is DungeonConfigBuilder - it allows to configure all necessary parameters that are required to build dungeon and it allows to configure BuilderAlgorithm which is defined as parameter with trait constraints.

### Algorithms

#### Partition (BSP) Algorithm
- Process:
  - Recursively splits the dungeon area into smaller rectangles (leaf nodes) using Binary Space Partitioning (BSP).
  - Places rooms within these leaf nodes.
  - Connects rooms using corridors between the centers of rooms in sibling leaf nodes.
- Advantages:
  - Creates a more structured dungeon layout.
  - Rooms are well-distributed across the dungeon.

#### Room Placement Algorithm
- Process:
  - Randomly places rooms on the map without overlapping.
  - Uses a connectivity graph to connect rooms using corridors.
  - Employs Kruskal's algorithm to generate a Minimum Spanning Tree (MST) for connecting rooms efficiently.
- Advantages:
  - Creates a more organic and random dungeon layout.
  - Can result in complex and non-linear dungeon designs.

## How to use it

```Rust
let dungeon = DungeonConfigBuilder::new()
        .dungeon_size(DungeonSize { width: w, height: h })
        .room_size(RoomSize { min_room_size: min_size, max_room_size: max_size })
        .build_algorithm(BinaryPartitionBuilder)
        .should_place_doors(false)
        .build()
        .expect("Failed to build dungeon");
let map = dungeon.map;
```

Map is a 2d array which can be used in your favority game engine to render your tiles of choice.
**DungeonConfigBuilder** provides default values, but you can configure - dungeon size, room min and max size. Select build algorithm and set if you wnat to place doors or not.
While building your dungeon - different error can happen, so you should handle errors, they are represented by Enum

```Rust
#[Error(desc = "Dungeon generation error", fmt = debug)]
pub enum DungeonBuildError {
    #[error(desc = "Size of map is not valid", fmt = display)]
    InvalidSize(String),
    #[error(desc = "Room min max configs are invalid", fmt = display)]
    InvalidRoomSize(String),
    #[error(desc = "Dungeon map size should not be less than room size", fmt = display)]
    RoomTooLargeForDungeon,
    #[error(desc = "No room was created, check configurations or try one more time", fmt = display)]
    NoRoomsCreated,
    #[error(desc = "Provide build algorithm for dungeon generation", fmt = display)]
    NoBuildAlgorithmProvided
}
```

# Examples
In the examples project there is a bevy a project which uses dungoxide to build random dungeons, you can play with it by configuring builder differently with different algorithms.

<img src="https://github.com/olegtyshcneko/dungoxide/blob/bdc481687b0b2b35bbf0d18d8968a87d2d52a231/readme/binary_partition_example.png?raw=true" width="320" height="320" /> <img src="https://github.com/olegtyshcneko/dungoxide/blob/bdc481687b0b2b35bbf0d18d8968a87d2d52a231/readme/room_placement_example.png?raw=true" width="320" height="320" />

# Acknowledgments
> This project is done as capstone project for Ukranian Rust Summer bootcamp. I want to thank all mentors and organizers for having this great opportunity to learn Rust and meet new people, you did a great job bringing us together!

> Also, I wanted to acknowledge [@mstdungeon] (https://github.com/redsled84), his implementation for room placement algorithm using Kurskal algorithm really helped me to implement my work. Best wishes to him!
