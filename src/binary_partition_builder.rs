use std::cmp::{min, max};
use rand::Rng;
use crate::dungeon::{
    Dungeon, DungeonBuilder, DungeonBuildConfig, DungeonBuildError,
    TileType
};

pub struct BinaryPartitionBuilder;

impl DungeonBuilder for BinaryPartitionBuilder {
    fn build(self, build_config: DungeonBuildConfig) -> Result<Dungeon, DungeonBuildError> {
        let width = build_config.dungeon_size.width;
        let height = build_config.dungeon_size.height;
        let room_min_size = build_config.room_size.min_room_size;
        let room_max_size = build_config.room_size.max_room_size;

        let mut map = vec![vec![TileType::Wall; width]; height];
        let mut root_node = RoomsPartition::new(Room {
            x: 0,
            y: 0,
            width,
            height,
        });

        root_node.partition_tree(room_min_size, room_max_size);

        let mut rooms = Vec::new();
        root_node.create_rooms(&mut rooms, room_min_size, room_max_size);

        if rooms.is_empty() {
            return Err(DungeonBuildError::NoRoomsCreated);
        }

        for room in &rooms {
            for y in room.y..(room.y + room.height) {
                for x in room.x..(room.x + room.width) {
                    map[y][x] = TileType::Floor;
                }
            }
        }

        root_node.connect_rooms(&mut map);

        if build_config.should_place_doors {
            place_doors(&mut map);
        }

        Ok(Dungeon { map })
    }
}

#[derive(Debug, Copy, Clone)]
struct Room {
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

impl Room {
    pub fn center(&self) -> (usize, usize) {
        (
            self.x + self.width / 2,
            self.y + self.height / 2
        )
    }
}

struct RoomsPartition {
    root_room: Room,
    room: Option<Room>,
    left: Option<Box<RoomsPartition>>,
    right: Option<Box<RoomsPartition>>,
}

impl RoomsPartition {
    pub fn new(root_room: Room) -> Self {
        Self {
            root_room,
            room: None,
            left: None,
            right: None
        }
    }

    pub fn split(&mut self, min_size: usize) -> bool {
        if self.left.is_some() || self.right.is_some() {
            return false;
        }

        let mut rng = rand::thread_rng();

        let should_split_horizontally = if self.root_room.width >= self.root_room.height {
            false
        } else if self.root_room.height >= self.root_room.width {
            true
        } else {
            rng.gen_bool(0.5)
        };

        let max_size = if should_split_horizontally {
            self.root_room.height - min_size
        } else {
            self.root_room.width - min_size
        };

        if max_size <= min_size {
            return false;
        }

        let split = rng.gen_range(min_size..max_size);

        if should_split_horizontally {
            let left_split = Box::new(RoomsPartition::new(Room {
                x: self.root_room.x,
                y: self.root_room.y,
                width: self.root_room.width,
                height: split,
            }));
            let right_split = Box::new(RoomsPartition::new(Room {
                x: self.root_room.x,
                y: self.root_room.y + split,
                width: self.root_room.width,
                height: self.root_room.height - split,
            }));
            self.left = Some(left_split);
            self.right = Some(right_split);

            return true;
        }

        let left_split = Box::new(RoomsPartition::new(Room {
            x: self.root_room.x,
            y: self.root_room.y,
            width: split,
            height: self.root_room.height,
        }));

        let right_split = Box::new(RoomsPartition::new(Room {
            x: self.root_room.x + split,
            y: self.root_room.y,
            width: self.root_room.width - split,
            height: self.root_room.height,
        }));

        self.left = Some(left_split);
        self.right = Some(right_split);

        true
    }

    pub fn partition_tree(&mut self, min_size: usize, max_size: usize) {
        let can_split = self.root_room.width > max_size
        || self.root_room.height > max_size
        || rand::thread_rng().gen_bool(0.5);

        if !can_split {
            return;
        }

        if self.split(min_size) {
            if let Some(ref mut left) = self.left {
                left.partition_tree(min_size, max_size);
            }
            if let Some(ref mut right) = self.right {
                right.partition_tree(min_size, max_size);
            }
        }
    }

    pub fn create_rooms(&mut self, rooms: &mut Vec<Room>, min_size: usize, max_size: usize) {
        let is_left_or_right = self.left.is_some() || self.right.is_some();

        if !is_left_or_right {
            let mut rng = rand::thread_rng();

            let (w_min, h_min) = (min_size, min_size);
            let (w_max, h_max) = (min(self.root_room.width - 1, max_size), min(self.root_room.height - 1, max_size));

            if w_min > w_max || h_min > h_max {
                return;
            }

            let room_w =
                rng.gen_range(w_min..=w_max);
            let room_h =
                rng.gen_range(h_min..=h_max);

            if room_w > self.root_room.x + self.root_room.width
            || room_h > self.root_room.y + self.root_room.height {
                return;
            }

            let room_x =
                rng.gen_range(self.root_room.x..=(self.root_room.x + self.root_room.width - room_w));
            let room_y =
                rng.gen_range(self.root_room.y..=(self.root_room.y + self.root_room.height - room_h));

            let room = Room {
                x: room_x,
                y: room_y,
                width: room_w,
                height: room_h,
            };
            self.room = Some(room);
            rooms.push(room);

            return;
        }

        if let Some(ref mut left) = self.left {
            left.create_rooms(rooms, min_size, max_size);
        }

        if let Some(ref mut right) = self.right {
            right.create_rooms(rooms, min_size, max_size);
        }
    }

    pub fn connect_rooms(&self, map: &mut Vec<Vec<TileType>>) {
        if let Some(ref left) = self.left {
            left.connect_rooms(map);
        }

        if let Some(ref right) = self.right {
            right.connect_rooms(map);
        }

        if self.left.is_some() && self.right.is_some() {
            let left_center = self.left.as_ref().unwrap().get_room_center();
            let right_center = self.right.as_ref().unwrap().get_room_center();

            if let (Some((left_x, left_y)), Some((right_x, right_y))) = (left_center, right_center) {
                apply_corridors(map, left_x, left_y, right_x, right_y);
            }
        }
    }

    pub fn get_room_center(&self) -> Option<(usize, usize)> {
        if let Some(ref room) = self.room {
            return Some(room.center());
        }

        self.left
            .as_ref()
            .and_then(|l| l.get_room_center())
            .or_else(|| self.right.as_ref().and_then(|r| r.get_room_center()))
    }
}

fn apply_corridors(
    map: &mut [Vec<TileType>],
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
) {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(0.5) {
        for x in min(x1, x2)..=max(x1, x2) {
            if map[y1][x] == TileType::Wall {
                map[y1][x] = TileType::Floor;
            }
        }
        for y in min(y1, y2)..=max(y1, y2) {
            if map[y][x2] == TileType::Wall {
                map[y][x2] = TileType::Floor;
            }
        }
    } else {
        for y in min(y1, y2)..=max(y1, y2) {
            if map[y][x1] == TileType::Wall {
                map[y][x1] = TileType::Floor;
            }
        }
        for x in min(x1, x2)..=max(x1, x2) {
            if map[y2][x] == TileType::Wall {
                map[y2][x] = TileType::Floor;
            }
        }
    }
}

/// algo to place doors outside of rooms
/// this algo doesn't work correctly, but I didn't have time to fix it
fn place_doors(map: &mut [Vec<TileType>]) {
    let height = map.len();
    let width = map[0].len();

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            if map[y][x] == TileType::Wall {
                let adjacent_floors = [
                    map[y - 1][x],
                    map[y + 1][x],
                    map[y][x - 1],
                    map[y][x + 1],
                ];
                let floor_count = adjacent_floors
                    .iter()
                    .filter(|&&tile| tile == TileType::Floor)
                    .count();

                if floor_count >= 2 {
                    map[y][x] = TileType::Door;
                }
            }
        }
    }
}
