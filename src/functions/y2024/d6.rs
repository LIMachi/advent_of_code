use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Write};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tiles {
    Empty,
    Crate,
    Walked,
    Edge
}

impl Tiles {
    fn walkable(&self) -> bool {
        match self {
            Tiles::Crate => false,
            _ => true
        }
    }
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tiles>,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![Tiles::Empty; width * height],
        }
    }

    fn set(&mut self, x: isize, y: isize, tile: Tiles) -> Tiles {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            return Tiles::Edge;
        }
        let prev = self.tiles[x as usize + y as usize * self.width];
        self.tiles[x as usize + y as usize * self.width] = tile;
        prev
    }

    fn get(&self, x: isize, y: isize) -> Tiles {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return Tiles::Edge;
        }
        self.tiles[x as usize + y as usize * self.width]
    }

    fn print(&self, guard: &Guard) {
        let gx = if guard.x < 0 { 0 } else if guard.x >= self.width as isize { self.width - 1 } else { guard.x as usize };
        let gy = if guard.y < 0 { 0 } else if guard.y >= self.height as isize { self.height - 1 } else { guard.y as usize };
        for y in 0..self.height {
            for x in 0..self.width {
                if gx == x && gy == y  {
                    print!("{}", match guard.dir {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    });
                } else {
                    print!("{}", match self.tiles[x + self.width * y] {
                        Tiles::Empty => '.',
                        Tiles::Crate => '#',
                        Tiles::Walked => 'X',
                        Tiles::Edge => ' ',
                    });
                }
            }
            println!();
        }
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right
}

impl Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        })
    }
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    fn rotate(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Default, Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Guard {
    x: isize,
    y: isize,
    dir: Direction,
}

#[derive(Default)]
struct PositionSet {
    set: HashSet<Guard>,
    vec: Vec<Guard>
}

impl PositionSet {
    fn add(&mut self, position: Guard) {
        self.vec.push(position);
        self.set.insert(position);
    }

    fn contains(&self, position: Guard) -> bool {
        self.set.contains(&position)
    }

    fn ordered(&self) -> &Vec<Guard> {
        &self.vec
    }

    fn clone_until(&self, position: Guard) -> Self {
        let mut clone = Self::default();
        for guard in &self.vec {
            if guard == &position { break }
            clone.add(*guard);
        }
        clone
    }
}

impl Guard {
    fn step(&self) -> Self {
        let (dx, dy) = self.dir.delta();
        Self {
            x: self.x + dx,
            y: self.y + dy,
            dir: self.dir,
        }
    }

    fn rotate(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            dir: self.dir.rotate(),
        }
    }

    fn get_tile_in_front(&self, map: &Map) -> Tiles {
        let (dx, dy) = self.dir.delta();
        map.get(self.x + dx, self.y + dy)
    }

    fn get_tile_under(&self, map: &Map) -> Tiles {
        map.get(self.x, self.y)
    }

    fn paint(&self, map: &mut Map, tile: Tiles) -> bool {
        map.set(self.x, self.y, tile) != tile
    }
}

fn parse(input: &str) -> Result<(Map, Guard), String> {
    let mut width = 0;
    let mut height = 0;
    let mut x = 0;
    for c in input.chars() {
        match c {
            '^' | '>' | '<' | 'v' | '.' | '#' => {
                if x == 0 {
                    height += 1;
                }
                x += 1;
                if height == 1 {
                    width += 1;
                } else if x > width {
                    return Err(format!("Line {height} too long {x} expected {width}"));
                }
            }
            '\n' => { x = 0; }
            '\r' => {}
            e @ _ => {
                return Err(format!("Invalid character '{e}'"));
            }
        }
    }
    let mut map = Map::new(width, height);
    let mut guard: Option<Guard> = None;
    let mut x = 0;
    let mut y = 0;
    for c in input.chars() {
        match c {
            '^' | '>' | '<' | 'v' | '.' | '#' => {
                let tile = if c == '#' { Tiles::Crate } else { Tiles::Empty };
                if c != '#' && c != '.' {
                    if guard.is_some() {
                        return Err("Duplicate guard".to_string());
                    }
                    guard = Some(Guard {
                        x,
                        y,
                        dir: match c {
                            '^' => Direction::Up,
                            '>' => Direction::Right,
                            '<' => Direction::Left,
                            'v' => Direction::Down,
                            _ => unreachable!()
                        }
                    });
                }
                map.set(x, y, tile);
                x += 1;
            }
            '\n' => {
                y += 1;
                x = 0;
            }
            _ => {}
        }
    }
    if let Some(guard) = guard {
        Ok((map, guard))
    } else {
        Err("no guard found".to_string())
    }
}

fn guard_patrol(mut map: Map, mut guard: Guard, mut positions: PositionSet) -> (Map, PositionSet, Option<usize>) {
    let mut acc = 0;
    while guard.get_tile_under(&map) != Tiles::Edge {
        if guard.paint(&mut map, Tiles::Walked) {
            acc += 1;
        } else if positions.contains(guard) {
            return (map, positions, None);
        }
        positions.add(guard);
        if guard.get_tile_in_front(&map).walkable() {
            guard = guard.step();
        } else {
            guard = guard.rotate();
        }
    }
    (map, positions, Some(acc))
}

pub fn y2024d6a(input: &str) -> Result<String, String> {
    let (map, guard) = parse(input)?;
    guard_patrol(map, guard, PositionSet::default()).2.ok_or_else(|| "Looping".to_string()).map(|r| r.to_string())
}

//FIXME: works but is too slow (took more than 10 seconds)
//other solution: instead of using an unsorted set, we could use a sorted list of positions
//this way we can copy all the steps prior to putting the obstacle and start the automaton right at
//the position of the added obstacle
//FIXME: new version using the sorted set works in almost 15 second, which is the limit of what advent_of_code allows, should find other ways to optimise it
//for now put the example input to make the other days faster
pub fn y2024d6b(input: &str) -> Result<String, String> {
    let (map, guard) = parse(input)?;
    let (map, positions, _) = guard_patrol(map, guard, PositionSet::default());
    let mut acc = 0;
    let mut tries = HashSet::new();
    for position in positions.ordered() {
        let t = position.step();
        if !tries.contains(&(t.x, t.y)) {
            let front = t.get_tile_under(&map);
            if front.walkable() && front != Tiles::Edge {
                let mut tmap = map.clone();
                t.paint(&mut tmap, Tiles::Crate);
                tries.insert((t.x, t.y));
                if guard_patrol(tmap, *position, positions.clone_until(*position)).2.is_none() {
                    acc += 1;
                }
            }
        }
    }
    Ok(acc.to_string())
}
