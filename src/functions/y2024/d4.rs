pub enum XmasDirection {
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
    Top,
    TopRight
}

impl XmasDirection {
    pub fn to_delta(&self) -> (isize, isize) {
        match self {
            XmasDirection::Right => (1, 0),
            XmasDirection::BottomRight => (1, 1),
            XmasDirection::Bottom => (0, 1),
            XmasDirection::BottomLeft => (-1, 1),
            XmasDirection::Left => (-1, 0),
            XmasDirection::TopLeft => (-1, -1),
            XmasDirection::Top => (0, -1),
            XmasDirection::TopRight => (1, -1)
        }
    }

    pub fn rotate_clockwise(&self) -> Self {
        match self {
            XmasDirection::Right => XmasDirection::BottomRight,
            XmasDirection::BottomRight => XmasDirection::Bottom,
            XmasDirection::Bottom => XmasDirection::BottomLeft,
            XmasDirection::BottomLeft => XmasDirection::Left,
            XmasDirection::Left => XmasDirection::TopLeft,
            XmasDirection::TopLeft => XmasDirection::Top,
            XmasDirection::Top => XmasDirection::TopRight,
            XmasDirection::TopRight => XmasDirection::Right
        }
    }

    pub fn rotate_counter_clockwise(&self) -> Self {
        match self {
            XmasDirection::Right => XmasDirection::TopRight,
            XmasDirection::BottomRight => XmasDirection::Right,
            XmasDirection::Bottom => XmasDirection::BottomRight,
            XmasDirection::BottomLeft => XmasDirection::Bottom,
            XmasDirection::Left => XmasDirection::BottomLeft,
            XmasDirection::TopLeft => XmasDirection::Left,
            XmasDirection::Top => XmasDirection::TopLeft,
            XmasDirection::TopRight => XmasDirection::Top,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            XmasDirection::Right => XmasDirection::Left,
            XmasDirection::BottomRight => XmasDirection::TopLeft,
            XmasDirection::Bottom => XmasDirection::Top,
            XmasDirection::BottomLeft => XmasDirection::TopRight,
            XmasDirection::Left => XmasDirection::Right,
            XmasDirection::TopLeft => XmasDirection::BottomRight,
            XmasDirection::Top => XmasDirection::Bottom,
            XmasDirection::TopRight => XmasDirection::BottomLeft,
        }
    }

    pub fn array() -> [Self;8] {
        [
            XmasDirection::Right,
            XmasDirection::BottomRight,
            XmasDirection::Bottom,
            XmasDirection::BottomLeft,
            XmasDirection::Left,
            XmasDirection::TopLeft,
            XmasDirection::Top,
            XmasDirection::TopRight
        ]
    }

    pub fn corners() -> [Self;4] {
        [
            XmasDirection::BottomRight,
            XmasDirection::BottomLeft,
            XmasDirection::TopLeft,
            XmasDirection::TopRight
        ]
    }

    pub fn cardinals() -> [Self;4] {
        [
            XmasDirection::Right,
            XmasDirection::Bottom,
            XmasDirection::Left,
            XmasDirection::Top
        ]
    }

    pub fn get_grid_delta<'g, T>(&self, grid: &'g Vec<Vec<T>>, start: (usize, usize), steps: usize) -> Option<&'g T> {
        let (dx, dy) = self.to_delta();
        let (dx, dy) = (dx * steps as isize, dy * steps as isize);
        if start.0 as isize + dx >= 0 && start.1 as isize + dy >= 0 {
            grid.get((start.1 as isize + dy) as usize).and_then(|l| l.get((start.0 as isize + dx) as usize))
        } else {
            None
        }
    }
}

pub fn y2024d4a(input: &str) -> Result<String, String> {
    let mut count = 0;
    let grid = input.split("\n").map(|l| {
        l.as_ascii().unwrap().iter().map(|c| match c.to_char() {
            'X' => 0,
            'M' => 1,
            'A' => 2,
            'S' => 3,
            c @ _ => unreachable!("the input should not have this char: {}", c)
        }).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>();
    for y in 0..grid.len(){
        for x in 0..grid[y].len() {
            if grid[y][x] != 0 { continue }
            for d in XmasDirection::array() {
                for i in 1..4 {
                    if !d.get_grid_delta(&grid, (x, y), i).map_or(false, |v| *v == i as u8) {
                        break;
                    }
                    if i == 3 {
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count.to_string())
}

pub fn y2024d4b(input: &str) -> Result<String, String> {
    let mut count = 0;
    let grid = input.split("\n").map(|l| {
        l.as_ascii().unwrap().iter().map(|c| match c.to_char() {
            'X' => 3,
            'M' => 1,
            'A' => 0,
            'S' => 2,
            c @ _ => unreachable!("the input should not have this char: {}", c)
        }).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>();
    for y in 0..grid.len() {
        'main_loop: for x in 0..grid[y].len() {
            if grid[y][x] != 0 { continue }
            for d in XmasDirection::corners() { //find first m, clockwise -> check if opposite is s, find next m quarter clockwise/counter clockwise, check opposite s
                if d.get_grid_delta(&grid, (x, y), 1).map_or(false, |v| *v == 1) {
                    if d.opposite().get_grid_delta(&grid, (x, y), 1).map_or(false, |v| *v == 2) {
                        let d = d.rotate_clockwise().rotate_clockwise();
                        let t = d.get_grid_delta(&grid, (x, y), 1).map_or(0, |v| *v);
                        if t == 1 || t == 2 {
                            if d.opposite().get_grid_delta(& grid, (x, y), 1).map_or(false, |v| (*v == 2 && t == 1) || (*v == 1 && t == 2)) {
                                count += 1;
                                continue 'main_loop;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(count.to_string())
}
