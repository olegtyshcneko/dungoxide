use justerror::Error;

pub trait DungeonBuilder {
    fn build(self, build_config: DungeonBuildConfig) -> Result<Dungeon, DungeonBuildError>;
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileType {
    Door = 1,
    Wall = 4,
    Floor = 5,
}

#[derive(Debug, Clone)]
pub struct Dungeon {
    pub map: Vec<Vec<TileType>>,
}

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

#[derive(Debug, Copy, Clone)]
pub struct DungeonSize {
    pub width: usize,
    pub height: usize
}

impl DungeonSize {
    pub fn validate(&self) -> Result<(), DungeonBuildError> {
        if self.width == 0 || self.height == 0 {
            return Err(DungeonBuildError::InvalidSize("Width and height must be bigger or equals to 5".to_owned()))
        }

        Ok(())
    }

    pub fn validate_room_size(&self, room_size: &RoomSize) -> Result<(), DungeonBuildError> {
        if room_size.min_room_size > self.width || room_size.min_room_size > self.height {
            return Err(DungeonBuildError::RoomTooLargeForDungeon);
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RoomSize {
    pub min_room_size: usize,
    pub max_room_size: usize
}

impl RoomSize {
    pub fn validate(&self) -> Result<(), DungeonBuildError> {
        if self.min_room_size == 0 || self.max_room_size == 0 {
            return Err(DungeonBuildError::InvalidRoomSize("Room size must be greater than 0".to_owned()));
        }

        if self.min_room_size > self.max_room_size {
            return Err(DungeonBuildError::InvalidRoomSize("Room maximum size must be greater or equal to minimum size".to_owned()));
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DungeonBuildConfig {
    pub dungeon_size: DungeonSize,
    pub room_size: RoomSize,
    pub should_place_doors: bool,
}

#[derive(Debug)]
pub struct DungeonConfigBuilder<BuilderAlgorithm> {
    dungeon_config: DungeonBuildConfig,
    build_algorithm: Option<BuilderAlgorithm>,
}

impl<BuilderAlgorithm: DungeonBuilder> DungeonConfigBuilder<BuilderAlgorithm> {
    pub fn new() -> Self {
        Self {
            dungeon_config: Default::default(),
            build_algorithm: None
        }
    }

    pub fn dungeon_size(mut self, dungeon_size: DungeonSize) -> Self {
        self.dungeon_config.dungeon_size = dungeon_size;
        self
    }

    pub fn room_size(mut self, room_size: RoomSize) -> Self {
        self.dungeon_config.room_size = room_size;
        self
    }

    pub fn build_algorithm(mut self, build_algorithm: BuilderAlgorithm) -> Self {
        self.build_algorithm = Some(build_algorithm);
        self
    }

    pub fn should_place_doors(mut self, should_place_doors: bool) -> Self {
        self.dungeon_config.should_place_doors = should_place_doors;
        self
    }

    pub fn build(self) -> Result<Dungeon, DungeonBuildError> {
        let build_algorithm = self.build_algorithm
            .ok_or(DungeonBuildError::NoBuildAlgorithmProvided)?;

        self.dungeon_config.dungeon_size.validate()?;
        self.dungeon_config.room_size.validate()?;
        self.dungeon_config.dungeon_size.validate_room_size(&self.dungeon_config.room_size)?;

        build_algorithm.build(self.dungeon_config)
    }
}

impl Default for DungeonBuildConfig {
    fn default() -> Self {
        Self {
            dungeon_size: DungeonSize {
                width: 32,
                height: 32
            },
            room_size: RoomSize {
                min_room_size: 5,
                max_room_size: 10
            },
            should_place_doors: false
        }
    }
}

impl<BuilderAlgorithm: DungeonBuilder> Default for DungeonConfigBuilder<BuilderAlgorithm> {
    fn default() -> Self {
        Self::new()
    }
}