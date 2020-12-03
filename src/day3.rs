use std::fs;

pub fn day_three() {
    println!("-----DAY3------");
    let grid = create_grid("inputs/input_day3.txt");
    println!("Part 1");
    part_one(grid.clone());
    println!("Part 2");
    part_two(grid);
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Tree,
    Empty,
}

#[derive(Debug, Clone)]
struct Row {
    row: Vec<Tile>,
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Row>,
}

impl Grid {
    fn get_tile(&self, coords: (i32, i32)) -> &Tile {
        &self.grid[coords.1 as usize].row[coords.0 as usize]
    }

    fn get_dimensions(&self) -> (i32, i32) {
        let rows = self.grid.len() as i32;
        let columns = self.grid[0].row.len() as i32;

        (rows, columns)
    }

    fn count_trees(&self, step: (i32, i32)) -> i32 {
        let mut tree_count = 0;
        let mut xpos: i32 = 0;
        let mut ypos: i32 = 0;
        let (rows, columns) = self.get_dimensions();

        while ypos < rows {
            if self.get_tile((xpos, ypos)) == &Tile::Tree {
                tree_count += 1;
            }
            xpos += step.0;
            ypos += step.1;
            if xpos >= columns {
                xpos -= columns;
            }
        }
        tree_count
    }
}

fn part_one(grid: Grid) {
    let trees = grid.count_trees((3, 1));

    println!("Number of trees bumped: {}", trees);
}

fn part_two(grid: Grid) {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product: usize = slopes
        .iter()
        .map(|(slopex, slopey)| grid.count_trees((*slopex, *slopey)) as usize)
        .product();

    println!("Number of trees bumped: {:?}", product);
}

fn create_grid(filename: &str) -> Grid {
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let mut rows: Vec<Row> = Vec::new();

    for line in contents.lines() {
        let row = line
            .chars()
            .map(|c| match c {
                '.' => Tile::Empty,
                '#' => Tile::Tree,
                _ => panic!("Invalid character provided"),
            })
            .collect::<Vec<Tile>>();
        rows.push(Row { row });
    }
    Grid { grid: rows }
}
