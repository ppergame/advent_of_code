use array2d::Array2D;
use num_integer::Roots;
use sscanf::scanf;
use std::collections::HashMap;

// invariant: always square
#[derive(Clone)]
struct Grid(Array2D<bool>);

impl Grid {
    fn parse(s: &str) -> Self {
        let len = s.len();
        let size = len.sqrt();
        assert_eq!(size * size, len);
        let iter = s.chars().map(|c| match c {
            '#' => true,
            '.' => false,
            _ => unreachable!(),
        });
        Self(Array2D::from_iter_row_major(iter, size, size).unwrap())
    }

    fn empty(size: usize) -> Self {
        Self(
            Array2D::from_iter_row_major(std::iter::repeat(false).take(size * size), size, size)
                .unwrap(),
        )
    }

    fn flip(&self) -> Self {
        let mut ret = self.clone();
        for row in 0..self.size() {
            for col in 0..self.size() {
                ret.0[(row, self.size() - col - 1)] = self.0[(row, col)];
            }
        }
        ret
    }

    fn rotate(&self) -> Self {
        let mut ret = self.clone();
        for row in 0..self.size() {
            for col in 0..self.size() {
                ret.0[(col, self.size() - row - 1)] = self.0[(row, col)];
            }
        }
        ret
    }

    fn apply(&self, patterns: &HashMap<Vec<bool>, Vec<bool>>) -> Self {
        let (block_size, new_block_size) = if self.size() % 2 == 0 { (2, 3) } else { (3, 4) };
        let new_size = self.size() / block_size * new_block_size;
        let mut ret = Grid::empty(new_size);
        for row in (0..self.size()).step_by(block_size) {
            for col in (0..self.size()).step_by(block_size) {
                let mut acc = vec![];
                for srow in row..row + block_size {
                    for scol in col..col + block_size {
                        acc.push(self.0[(srow, scol)]);
                    }
                }
                ret.write(
                    row / block_size * new_block_size,
                    col / block_size * new_block_size,
                    new_block_size,
                    &patterns[&acc],
                );
            }
        }
        ret
    }

    fn write(&mut self, row: usize, col: usize, block_size: usize, block: &[bool]) {
        let mut idx = 0;
        assert_eq!(block_size * block_size, block.len());
        for srow in row..row + block_size {
            for scol in col..col + block_size {
                self.0[(srow, scol)] = block[idx];
                idx += 1;
            }
        }
    }

    fn size(&self) -> usize {
        self.0.num_rows()
    }

    fn to_vec(&self) -> Vec<bool> {
        self.0.as_row_major()
    }

    fn count_ones(&self) -> usize {
        self.0.elements_row_major_iter().filter(|&&b| b).count()
    }
}

fn parse(inp: &str) -> HashMap<Vec<bool>, Vec<bool>> {
    let mut patterns = HashMap::new();
    for line in inp.lines() {
        let (left, right) = scanf!(line, "{} => {}", str, str).unwrap();
        let mut from = Grid::parse(&left.replace('/', ""));
        let to = Grid::parse(&right.replace('/', ""));
        for _ in 0..4 {
            patterns.insert(from.to_vec(), to.to_vec());
            from = from.rotate();
        }
        from = from.flip();
        for _ in 0..4 {
            patterns.insert(from.to_vec(), to.to_vec());
            from = from.rotate();
        }
    }
    patterns
}

fn part1(inp: &str) -> usize {
    let patterns = parse(inp);
    let mut grid = Grid::parse(".#...####");
    for _ in 0..5 {
        grid = grid.apply(&patterns);
    }
    grid.count_ones()
}

fn part2(inp: &str) -> usize {
    let patterns = parse(inp);
    let mut grid = Grid::parse(".#...####");
    for _ in 0..18 {
        grid = grid.apply(&patterns);
    }
    grid.count_ones()
}

xaoc::xaoc!(no_sample = true);
