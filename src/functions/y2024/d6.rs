#[derive(Copy, Clone, Eq, PartialEq)]
enum Tiles {
    Empty,
    Crate,
    Walked(Direction),
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
                        Tiles::Walked(d) => match d {
                            Direction::Up => '^',
                            Direction::Down => 'v',
                            Direction::Left => '<',
                            Direction::Right => '>',
                        },
                        Tiles::Edge => ' ',
                    });
                }
            }
            println!();
        }
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right
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

#[derive(Default, Copy, Clone)]
struct Guard {
    x: isize,
    y: isize,
    dir: Direction,
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

fn guard_patrol(map: &Map, mut guard: Guard) -> Option<usize> {
    let mut acc = 0;
    let mut map = map.clone();
    while map.get(guard.x, guard.y) != Tiles::Edge {
        match map.set(guard.x, guard.y, Tiles::Walked(guard.dir)) {
            Tiles::Walked(d) => {
                if d != guard.dir {
                    acc += 1;
                } else {
                    return None;
                }
            },
            _ => {}
        }
        let (dx, dy) = guard.dir.delta();
        if map.get(guard.x + dx, guard.y + dy).walkable() {
            guard.x += dx;
            guard.y += dy;
        } else {
            guard.dir = guard.dir.rotate();
        }
    }
    Some(acc)
}

pub fn y2024d6a(input: &str) -> Result<String, String> {
    let (map, guard) = parse(input)?;
    guard_patrol(&map, guard).ok_or_else(|| "Looping".to_string()).map(|r| r.to_string())
}

pub fn y2024d6b(input: &str) -> Result<String, String> {
    let (mut map, guard) = parse(input)?;
    let mut acc = 0;
    for y in 0..map.height as isize {
        for x in 0..map.width as isize {
            if map.get(x, y) == Tiles::Empty {
                map.set(x, y, Tiles::Crate);
                if guard_patrol(&map, guard).is_none() {
                    acc += 1;
                }
                map.set(x, y, Tiles::Empty);
            }
        }
    }
    Ok(acc.to_string())
}
