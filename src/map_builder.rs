use crate::prelude::*;
const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(2, SCREEN_WIDTH - 12),
                rng.range(2, SCREEN_HEIGHT - 12),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            // checking if room overlaps with existing ones
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let index = map_index(p.x, p.y);
                        self.map.tiles[index] = TileType::Floor;
                        
                        //building walls
                        let index_left = map_index(p.x - 1, p.y);
                        if self.map.tiles[index_left] == TileType::Empty {
                            self.map.tiles[index] = TileType::Wall;
                        }

                        let index_up = map_index(p.x, p.y - 1);
                        if self.map.tiles[index_up] == TileType::Empty {
                            self.map.tiles[index_up] = TileType::Wall;
                        }

                        let index_right = map_index(p.x + 1, p.y);
                        if self.map.tiles[index_right] == TileType::Empty {
                            self.map.tiles[index_right] = TileType::Wall;
                        }

                        let index_bottom = map_index(p.x, p.y + 1);
                        if self.map.tiles[index_bottom] == TileType::Empty {
                            self.map.tiles[index_bottom] = TileType::Wall;
                        }
                    }
                });
                self.rooms.push(room)
            }
        }
    }

    fn create_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};

        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(index) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[index as usize] = TileType::Floor;
            
                // building walls
                if let Some(index_left) = self.map.try_index(Point::new(x - 1, y)) {
                    if self.map.tiles[index_left as usize] == TileType::Empty {
                        self.map.tiles[index_left as usize] = TileType::Wall;
                    }
                }

                if let Some(index_right) = self.map.try_index(Point::new(x + 1, y)) {
                    if self.map.tiles[index_right as usize] == TileType::Empty {
                        self.map.tiles[index_right as usize] = TileType::Wall;
                    }
                }
            }        
        }
    }

    fn create_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};

        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(index) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[index as usize] = TileType::Floor;

                // building walls
                if let Some(index_top) = self.map.try_index(Point::new(x, y - 1)) {
                    if self.map.tiles[index_top as usize] == TileType::Empty {
                        self.map.tiles[index_top as usize] = TileType::Wall;
                    }
                } 

                if let Some(index_bottom) = self.map.try_index(Point::new(x, y + 1)) {
                    if self.map.tiles[index_bottom as usize] == TileType::Empty {
                        self.map.tiles[index_bottom as usize] = TileType::Wall;
                    }
                } 
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();

        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.create_horizontal_tunnel(prev.x, new.x, prev.y);
                self.create_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.create_vertical_tunnel(prev.y, new.y, prev.x);
                self.create_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        mb.fill(TileType::Empty);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb
    }
}
