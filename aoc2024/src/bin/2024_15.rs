use hashbrown::HashSet;

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn parse(c: char) -> Self {
        match c {
            '^' => Self::N,
            '>' => Self::E,
            'v' => Self::S,
            '<' => Self::W,
            _ => panic!("bad dir {c}"),
        }
    }

    fn delta(&self) -> (i64, i64) {
        match self {
            Self::N => (-1, 0),
            Self::E => (0, 1),
            Self::S => (1, 0),
            Self::W => (0, -1),
        }
    }

    #[allow(dead_code)]
    fn to_char(self) -> char {
        match self {
            Self::N => '^',
            Self::E => '>',
            Self::S => 'v',
            Self::W => '<',
        }
    }
}

struct Map {
    max_col: i64,
    max_row: i64,
    bot: (i64, i64),
    walls: HashSet<(i64, i64)>,
    boxes: HashSet<(i64, i64)>,
    dirs: Vec<Dir>,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut it = inp.lines();
        let mut bot = None;
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut max_col = 0;
        let mut max_row = 0;

        for (row, l) in (&mut it).enumerate() {
            if l.is_empty() {
                break;
            }
            let row = row as i64;
            max_row = max_row.max(row);
            for (col, c) in l.chars().enumerate() {
                let col = col as i64;
                max_col = max_col.max(col);
                match c {
                    '#' => {
                        walls.insert((row, col));
                    }
                    '.' => (),
                    '@' => {
                        bot = Some((row, col));
                    }
                    'O' => {
                        boxes.insert((row, col));
                    }
                    _ => unreachable!(),
                }
            }
        }
        let Some(bot) = bot else {
            panic!("No bot found");
        };

        let mut dirs = vec![];
        for l in it {
            for c in l.chars() {
                match c {
                    ' ' | '\n' => (),
                    _ => dirs.push(Dir::parse(c)),
                }
            }
        }
        Self {
            max_col,
            max_row,
            bot,
            walls,
            boxes,
            dirs,
        }
    }

    fn run(&mut self) {
        // println!("Initial state:");
        // self.print();
        for d in std::mem::take(&mut self.dirs) {
            let (dr, dc) = d.delta();
            let (mut lrow, mut lcol) = self.bot;
            // find the space after the last adjacent box
            loop {
                lrow += dr;
                lcol += dc;
                if !self.boxes.contains(&(lrow, lcol)) {
                    break;
                }
            }
            // bonk
            if self.walls.contains(&(lrow, lcol)) {
                continue;
            }
            let (row, col) = self.bot;
            let nrow = row + dr;
            let ncol = col + dc;
            // move boxes if any
            if nrow != lrow || ncol != lcol {
                self.boxes.remove(&(nrow, ncol));
                self.boxes.insert((lrow, lcol));
            }
            self.bot = (nrow, ncol);
            // println!("Move {}:", d.to_char());
            // self.print();
        }
    }

    fn score(&self) -> i64 {
        self.boxes.iter().map(|(r, c)| r * 100 + c).sum()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..=self.max_row {
            for col in 0..=self.max_col {
                let c = if self.bot == (row, col) {
                    '@'
                } else if self.boxes.contains(&(row, col)) {
                    'O'
                } else if self.walls.contains(&(row, col)) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

fn part1(inp: &str) -> i64 {
    let mut map = Map::parse(inp);
    map.run();
    map.score()
}

struct Map2 {
    max_col: i64,
    max_row: i64,
    bot: (i64, i64),
    walls: HashSet<(i64, i64)>,
    // left half of each box
    boxes: HashSet<(i64, i64)>,
    dirs: Vec<Dir>,
}

impl Map2 {
    fn parse(inp: &str) -> Self {
        let mut it = inp.lines();
        let mut bot = None;
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut max_col = 0;
        let mut max_row = 0;

        for (row, l) in (&mut it).enumerate() {
            if l.is_empty() {
                break;
            }
            let row = row as i64;
            max_row = max_row.max(row);
            for (col, c) in l.chars().enumerate() {
                let col = col as i64;
                max_col = max_col.max(col * 2 + 1);
                match c {
                    '#' => {
                        walls.insert((row, col * 2));
                        walls.insert((row, col * 2 + 1));
                    }
                    '.' => (),
                    '@' => {
                        bot = Some((row, col * 2));
                    }
                    'O' => {
                        boxes.insert((row, col * 2));
                    }
                    _ => unreachable!(),
                }
            }
        }
        let Some(bot) = bot else {
            panic!("No bot found");
        };
        let mut dirs = vec![];
        for l in it {
            for c in l.chars() {
                match c {
                    ' ' | '\n' => (),
                    _ => dirs.push(Dir::parse(c)),
                }
            }
        }
        Self {
            max_col,
            max_row,
            bot,
            walls,
            boxes,
            dirs,
        }
    }

    fn run(&mut self) {
        // println!("Initial state:");
        'outer: for d in std::mem::take(&mut self.dirs) {
            // self.print();
            // println!("Move {}:", d.to_char());
            let (dr, dc) = d.delta();
            // boxes to move
            let mut to_move = HashSet::new();

            let (row, col) = self.bot;
            // squares to consider
            let mut to_consider = vec![(row + dr, col + dc)];

            while let Some((row, col)) = to_consider.pop() {
                if self.walls.contains(&(row, col)) {
                    continue 'outer;
                }
                for (nrow, ncol) in [(row, col), (row, col - 1)] {
                    if self.boxes.contains(&(nrow, ncol)) && !to_move.contains(&(nrow, ncol)) {
                        to_move.insert((nrow, ncol));
                        // XXX: is this different between vertical and horizontal?
                        to_consider.push((nrow + dr, ncol + dc));
                        to_consider.push((nrow + dr, ncol + dc + 1));
                    }
                }
            }

            for &(row, col) in &to_move {
                self.boxes.remove(&(row, col));
            }
            for (row, col) in to_move {
                self.boxes.insert((row + dr, col + dc));
            }

            self.bot = (row + dr, col + dc);
        }
        // self.print();
    }

    fn score(&self) -> i64 {
        self.boxes.iter().map(|(r, c)| r * 100 + c).sum()
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut is_box = false;
        for row in 0..=self.max_row {
            for col in 0..=self.max_col {
                if is_box {
                    print!("]");
                    is_box = false;
                    continue;
                }
                let c = if self.bot == (row, col) {
                    '@'
                } else if self.boxes.contains(&(row, col)) {
                    is_box = true;
                    '['
                } else if self.walls.contains(&(row, col)) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

fn part2(inp: &str) -> i64 {
    let mut map = Map2::parse(inp);
    map.run();
    map.score()
}

xaoc::xaoc!();
