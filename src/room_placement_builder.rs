use std::cmp::{min, max};
use rand::Rng;
use crate::dungeon::{
    Dungeon, DungeonBuilder, DungeonBuildConfig, DungeonBuildError,
    TileType
};

pub struct RoomPlacementBuilder;

impl DungeonBuilder for RoomPlacementBuilder {
    fn build(self, build_config: DungeonBuildConfig) -> Result<Dungeon, DungeonBuildError> {
        let width = build_config.dungeon_size.width;
        let height = build_config.dungeon_size.height;
        let room_min_size = build_config.room_size.min_room_size;
        let room_max_size = build_config.room_size.max_room_size;

        let mut map = vec![vec![TileType::Wall; width]; height];
        let mut rng = rand::thread_rng();
        let mut rooms = Vec::new();

        let max_rooms = (width * height) / (room_min_size * room_max_size);

        for _ in 0..max_rooms {
            let next_room_w = rng.gen_range(room_min_size..=room_max_size);
            let next_room_h = rng.gen_range(room_min_size..=room_max_size);

            if next_room_w >= width - 2 || next_room_h >= height - 2 {
                continue;
            }

            let x = rng.gen_range(1..(width - next_room_w - 1));
            let y = rng.gen_range(1..(height - next_room_h - 1));

            let next_room = Room::new(x, y, next_room_w, next_room_h);

            if rooms.iter().all(|r| !next_room.intersects(r)) {
                for i in next_room.x..(next_room.x + next_room.width) {
                    for j in next_room.y..(next_room.y + next_room.height) {
                        map[j][i] = TileType::Floor;
                    }
                }
                rooms.push(next_room);
            }
        }

        if rooms.is_empty() {
            return Err(DungeonBuildError::NoRoomsCreated);
        }

        let mut edges = Vec::new();

        for i in 0..rooms.len() {
            for j in (i + 1)..rooms.len() {
                let distance = (
                    (rooms[i].center_x as isize - rooms[j].center_x as isize).pow(2) +
                    (rooms[i].center_y as isize - rooms[j].center_y as isize).pow(2)
                ) as f64;
                edges.push(((i, j), distance.sqrt()));
            }
        }

        edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Kruskal algorithm to find minimum spanning tree
        let mut union_find = UnionFind::new(rooms.len());
        let mut corridors = Vec::new();

        for ((i, j), _) in &edges {
            if union_find.find(*i) != union_find.find(*j) {
                union_find.union(*i, *j);

                create_corridor(&mut map, &rooms[*i], &rooms[*j]);
                corridors.push((*i, *j));

                if union_find.count() == 1 {
                    break;
                }
            }
        }

        let extra_corridors = rng.gen_range(0..2);
        let mut added = 0;
        for ((i, j), _) in &edges {
            if !corridors.contains(&(*i, *j)) && !corridors.contains(&(*j, *i)) {
                create_corridor(&mut map, &rooms[*i], &rooms[*j]);
                added += 1;
                if added >= extra_corridors {
                    break;
                }
            }
        }

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
    height: usize,
    center_x: usize,
    center_y: usize,
}

impl Room {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        let center_x = x + width / 2;
        let center_y = y + height / 2;
        Room {
            x,
            y,
            width,
            height,
            center_x,
            center_y,
        }
    }

    pub fn intersects(&self, other: &Room) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

/// Disjoint union to do search using Kruskal algorithm
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
            size
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let xroot = self.find(x);
        let yroot = self.find(y);

        if xroot != yroot {
            match self.rank[xroot].cmp(&self.rank[yroot]) {
                std::cmp::Ordering::Less => {
                    self.parent[xroot] = yroot;
                },
                std::cmp::Ordering::Greater => {
                    self.parent[yroot] = xroot;
                },
                std::cmp::Ordering::Equal => {
                    self.parent[yroot] = xroot;
                    self.rank[xroot] += 1;
                },
            }
            self.size -= 1;
        }
    }

    pub fn count(&self) -> usize {
        self.size
    }
}

fn create_corridor(map: &mut [Vec<TileType>], room1: &Room, room2: &Room) {
    let mut rng = rand::thread_rng();

    let (x1, y1) = (room1.center_x, room1.center_y);
    let (x2, y2) = (room2.center_x, room2.center_y);

    if rng.gen_bool(0.5) {
        for x in min(x1, x2)..=max(x1, x2) {
            map[y1][x] = TileType::Floor;
        }
        for y in min(y1, y2)..=max(y1, y2) {
            map[y][x2] = TileType::Floor;
        }
    } else {
        for y in min(y1, y2)..=max(y1, y2) {
            map[y][x1] = TileType::Floor;
        }
        for x in min(x1, x2)..=max(x1, x2) {
            map[y2][x] = TileType::Floor;
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
