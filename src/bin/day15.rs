use advent_2024::{Direction, TileIndex};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Entity {
    Empty,
    Boulder(BoulderShape),
    Wall,
    Player,
}

#[derive(Debug, PartialEq)]
enum BoulderShape {
    Single,
    WideLeft,
    WideRight,
}

impl From<char> for Entity {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Entity::Wall,
            '@' => Entity::Player,
            'O' => Entity::Boulder(BoulderShape::Single),
            '[' => Entity::Boulder(BoulderShape::WideLeft),
            ']' => Entity::Boulder(BoulderShape::WideRight),
            '.' => Entity::Empty,
            _ => panic!("unexpected ch {:?}", ch),
        }
    }
}

impl From<&Entity> for char {
    fn from(entity: &Entity) -> Self {
        match entity {
            Entity::Wall => '#',
            Entity::Player => '@',
            Entity::Boulder(BoulderShape::Single) => 'O',
            Entity::Boulder(BoulderShape::WideLeft) => '[',
            Entity::Boulder(BoulderShape::WideRight) => ']',
            Entity::Empty => '.',
        }
    }
}

#[derive(PartialEq, Debug)]
struct Sokoban {
    data: Vec<Entity>,
    tiles: TileIndex,
    player_pos: usize,
}

impl Sokoban {
    fn forward(&mut self, dir: Direction) {
        let mut to_move: Vec<usize> = vec![self.player_pos];
        let mut border: HashSet<usize> = HashSet::new();
        border.insert(self.player_pos);

        loop {
            // Find the next border (and if we fall off, fail)
            let Some(next_border) = border
                .iter()
                .map(|pos| {
                    self.tiles
                        .dir_to(*pos, dir)
                        .map(|pos| (pos, &self.data[pos]))
                })
                .collect::<Option<Vec<(usize, &Entity)>>>()
            else {
                return;
            };

            // Give up if any of them are walls.
            if next_border
                .iter()
                .any(|(_, entity)| matches!(entity, Entity::Wall))
            {
                return;
            }

            // Push if all of them are empty
            if next_border
                .iter()
                .all(|(_, entity)| matches!(entity, Entity::Empty))
            {
                for pos in to_move.into_iter().rev() {
                    self.data.swap(pos, self.tiles.dir_to(pos, dir).unwrap());
                }

                self.player_pos = self.tiles.dir_to(self.player_pos, dir).unwrap();
                return;
            }

            // Otherwise, set up the border with the boulders, and loop.
            border = next_border
                .iter()
                .flat_map(|(pos, entity)| match entity {
                    Entity::Empty => vec![],
                    Entity::Boulder(shape) => {
                        if dir.is_horizontal() {
                            vec![*pos]
                        } else {
                            match shape {
                                BoulderShape::Single => {
                                    vec![*pos]
                                }
                                BoulderShape::WideLeft => {
                                    vec![*pos, self.tiles.dir_to(*pos, Direction::Right).unwrap()]
                                }
                                BoulderShape::WideRight => {
                                    vec![*pos, self.tiles.dir_to(*pos, Direction::Left).unwrap()]
                                }
                            }
                        }
                    }
                    Entity::Wall => panic!("impossible"),
                    Entity::Player => vec![*pos],
                })
                .collect();
            to_move.extend(border.iter());
        }
    }

    fn score(&self) -> u32 {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(pos, entity)| match entity {
                Entity::Boulder(BoulderShape::Single) => Some(
                    100 * (pos as u32 / self.tiles.width as u32)
                        + (pos as u32 % self.tiles.width as u32),
                ),
                Entity::Boulder(BoulderShape::WideLeft) => {
                    let row = pos as u32 / self.tiles.width as u32;
                    let col = pos as u32 % self.tiles.width as u32;
                    let result = Some(100 * row + col);
                    result
                }
                _ => None,
            })
            .sum()
    }
}

impl FromStr for Sokoban {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let data: Vec<Entity> = s
            .lines()
            .flat_map(|line| line.trim().chars().map(Entity::from))
            .collect();
        let height = s.lines().count();
        let width = data.len() / height;
        let player_pos = data
            .iter()
            .position(|x| *x == Entity::Player)
            .ok_or("No player found in map")?;
        Ok(Self {
            data,
            tiles: TileIndex { width, height },
            player_pos,
        })
    }
}

impl std::fmt::Display for Sokoban {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if self.data[self.player_pos] != Entity::Player {
            write!(
                formatter,
                "Warning!  player_pos inconsistency!  player_pos: {:?}",
                self.player_pos
            )?;
        }

        for i in 0..self.data.len() {
            if i != 0 && i % self.tiles.width == 0 {
                writeln!(formatter)?;
            }
            write!(formatter, "{}", char::from(&self.data[i]))?;
        }
        writeln!(formatter)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
    use indoc::indoc;

    #[gtest]
    fn test_sokoban_parsing() -> Result<()> {
        let data = "\
	####
        #.O#
        #@.#
        ####
	";
        use Entity as E;
        verify_that!(
            data.parse::<Sokoban>().into_test_result()?,
            eq(&Sokoban {
                data: vec![
                    E::Wall,
                    E::Wall,
                    E::Wall,
                    E::Wall,
                    E::Wall,
                    E::Empty,
                    E::Boulder(BoulderShape::Single),
                    E::Wall,
                    E::Wall,
                    E::Player,
                    E::Empty,
                    E::Wall,
                    E::Wall,
                    E::Wall,
                    E::Wall,
                    E::Wall,
                ],
                player_pos: 9,
                tiles: TileIndex {
                    width: 4,
                    height: 4
                },
            },)
        )
    }

    #[gtest]
    fn test_movement() -> Result<()> {
        let mut board: Sokoban = indoc! {"
	####
        #@.#
        #..#
        ####
	"}
        .parse()
        .into_test_result()?;

        board.forward(Direction::Right);

        verify_that!(
            format!("{}", board),
            eq(indoc! {"
        ####
        #.@#
        #..#
        ####
"
            })
        )?;

        board.forward(Direction::Down);

        verify_that!(
            format!("{}", board),
            eq(indoc! {"
        ####
        #..#
        #.@#
        ####
"
            })
        )?;

        board.forward(Direction::Left);

        verify_that!(
            format!("{}", board),
            eq(indoc! {"
        ####
        #..#
        #@.#
        ####
"
            })
        )?;

        board.forward(Direction::Up);

        verify_that!(
            format!("{}", board),
            eq(indoc! {"
        ####
        #@.#
        #..#
        ####
"
            })
        )?;

        Ok(())
    }

    #[gtest]
    fn test_movement_pushing() -> Result<()> {
        let mut board: Sokoban = indoc! {"
        ###########
        #@O.O.#...#
        #.........#
        ###########
	"}
        .parse()
        .into_test_result()?;

        board.forward(Direction::Right);

        verify_that!(
            format!("{}", board),
            eq(indoc! {"
        ###########
        #.@OO.#...#
        #.........#
        ###########
"
            })
        )?;

        board.forward(Direction::Right);

        verify_that!(
            format!("{}", board),
            eq(indoc! {"
        ###########
        #..@OO#...#
        #.........#
        ###########
"
            })
        )?;

        board.forward(Direction::Right);

        verify_that!(
            format!("{}", board),
            eq(indoc! {"
        ###########
        #..@OO#...#
        #.........#
        ###########
"
            })
        )?;

        Ok(())
    }

    #[gtest]
    fn test_pushing_wide() -> Result<()> {
        let mut board: Sokoban = indoc! {"
            ##############
            ##......##..##
            ##..........##
            ##....[][]@.##
            ##....[]....##
            ##..........##
            ##############
            "
        }
        .parse()
        .into_test_result()?;

        board.forward(Direction::Left);

        verify_that!(
            format!("{}", board),
            eq(indoc! {"
            ##############
            ##......##..##
            ##..........##
            ##...[][]@..##
            ##....[]....##
            ##..........##
            ##############
"
            })
        )?;

        board.forward(Direction::Down);
        board.forward(Direction::Down);
        board.forward(Direction::Left);
        board.forward(Direction::Left);

        verify_that!(
            format!("{}", board),
            eq(indoc! {"
            ##############
            ##......##..##
            ##..........##
            ##...[][]...##
            ##....[]....##
            ##.....@....##
            ##############
"
            })
        )?;

        board.forward(Direction::Up);

        verify_that!(
            format!("{}", board),
            eq(indoc! {"
            ##############
            ##......##..##
            ##...[][]...##
            ##....[]....##
            ##.....@....##
            ##..........##
            ##############
"
            })
        )?;

        board.forward(Direction::Up);

        verify_that!(
            format!("{}", board),
            eq(indoc! {"
            ##############
            ##......##..##
            ##...[][]...##
            ##....[]....##
            ##.....@....##
            ##..........##
            ##############
"
            })
        )?;

        Ok(())
    }

    #[gtest]
    fn test_small_example() -> Result<()> {
        let data = indoc! {"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"
        };
        let (mut sokoban, directions) = parse_part_1_problem(data);
        for direction in directions {
            sokoban.forward(direction);
        }

        verify_that!(sokoban.score(), eq(2028))
    }

    #[gtest]
    fn test_large_example() -> Result<()> {
        let data = indoc! {"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
        };
        let (mut sokoban, directions) = parse_part_1_problem(data);
        for direction in directions {
            sokoban.forward(direction);
        }

        verify_that!(sokoban.score(), eq(10092))
    }

    #[gtest]
    fn test_score_wide() -> Result<()> {
        let board: Sokoban = indoc! {"
        ##########
        #@.[]....#
        #........#
        ##########
"}
        .parse()
        .into_test_result()?;
        verify_that!(board.score(), eq(103))?;

        let board: Sokoban = indoc! {"
        ##########
        #@..[]...#
        #........#
        ##########
"}
        .parse()
        .into_test_result()?;
        verify_that!(board.score(), eq(104))?;

        let board: Sokoban = indoc! {"
        ##########
        #@...[]..#
        #........#
        ##########
"}
        .parse()
        .into_test_result()?;
        verify_that!(board.score(), eq(105))?;

        let board: Sokoban = indoc! {"
        ##########
        #@.......#
        #....[]..#
        ##########
"}
        .parse()
        .into_test_result()?;
        verify_that!(board.score(), eq(205))?;

        let board: Sokoban = indoc! {"
        ##########
        #@.......#
        #........#
        #........#
        #........#
        #........#
        #....[]..#
        ##########
"}
        .parse()
        .into_test_result()?;
        verify_that!(board.score(), eq(605))?;

        Ok(())
    }

    #[gtest]
    fn test_large_example_scaled() -> Result<()> {
        let data = indoc! {"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
        };
        let (mut sokoban, directions) = parse_part_2_problem(data);
        for direction in directions {
            sokoban.forward(direction);
        }

        verify_that!(
            format!("{}", sokoban),
            eq(indoc! {"
####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################
"
            })
        )?;

        verify_that!(sokoban.score(), eq(9021))
    }
}

fn parse_directions(s: &str) -> Vec<Direction> {
    s.trim()
        .chars()
        .filter(|ch| !ch.is_ascii_whitespace())
        .map(|ch| Direction::try_from(ch).expect("Directions"))
        .collect()
}

fn parse_part_1_problem(s: &str) -> (Sokoban, Vec<Direction>) {
    let mut items = s.split("\n\n");
    let sokoban: Sokoban = items.next().expect("map").trim().parse().unwrap();
    let directions: Vec<Direction> = parse_directions(items.next().expect("directions").trim());
    (sokoban, directions)
}

/// This handles the map expansion for part 2.
fn widen_map(s: &str) -> String {
    s.chars()
        .flat_map(|ch| match ch {
            '#' => vec!['#', '#'],
            'O' => vec!['[', ']'],
            '.' => vec!['.', '.'],
            '@' => vec!['@', '.'],
            _ => vec![ch],
        })
        .collect()
}

fn parse_part_2_problem(s: &str) -> (Sokoban, Vec<Direction>) {
    let mut items = s.split("\n\n");
    let sokoban: Sokoban = widen_map(items.next().expect("map").trim())
        .parse()
        .unwrap();
    let directions: Vec<Direction> = parse_directions(items.next().expect("directions").trim());
    (sokoban, directions)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::io::read_to_string(std::io::stdin())?;
    let (mut sokoban, directions) = parse_part_1_problem(&data);
    for direction in directions {
        sokoban.forward(direction);
    }
    println!("Part 1: {}", sokoban.score());

    let (mut sokoban, directions) = parse_part_2_problem(&data);
    for direction in directions {
        sokoban.forward(direction);
    }
    println!("Part 2: {}", sokoban.score());

    Ok(())
}
