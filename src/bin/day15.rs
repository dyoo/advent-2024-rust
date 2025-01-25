use advent_2024::{Direction, TileIndex};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Entity {
    Empty,
    Boulder,
    Wall,
    Player,
}

impl From<char> for Entity {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Entity::Wall,
            '@' => Entity::Player,
            'O' => Entity::Boulder,
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
            Entity::Boulder => 'O',
            Entity::Empty => '.',
        }
    }
}

#[derive(PartialEq)]
struct Sokoban {
    data: Vec<Entity>,
    tiles: TileIndex,
    player_pos: usize,
}

impl Sokoban {
    fn forward(&mut self, dir: Direction) {
        let Some(to) = self.tiles.dir_to(self.player_pos, dir) else {
            return;
        };

        match self.data[to] {
            Entity::Empty => {
                self.data.swap(self.player_pos, to);
                self.player_pos = to;
            }
            Entity::Wall => {}
            Entity::Boulder => {
                let mut vacancy_candidate = to;
                loop {
                    let Some(next_candidate) = self.tiles.dir_to(vacancy_candidate, dir) else {
                        return;
                    };
                    match self.data[next_candidate] {
                        Entity::Empty => {
                            self.data[next_candidate] = Entity::Boulder;
                            self.data[self.player_pos] = Entity::Empty;
                            self.data[to] = Entity::Player;
                            self.player_pos = to;
                            return;
                        }
                        Entity::Wall => {
                            return;
                        }
                        Entity::Boulder => {
                            // Being pushed as well.
                        }
                        Entity::Player => {
                            panic!("Ran into self?");
                        }
                    }
                    vacancy_candidate = next_candidate;
                }
            }
            Entity::Player => {
                panic!("Ran into self?");
            }
        }
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        self.data
            .iter()
            .enumerate()
            .filter(|(_, entity)| **entity == Entity::Boulder)
            .map(|(pos, _)| {
                100 * (pos as u32 / self.tiles.width as u32)
                    + (pos as u32 % self.tiles.width as u32)
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

impl std::fmt::Debug for Sokoban {
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
                write!(formatter, "\n")?;
            }
            write!(formatter, "{}", char::from(&self.data[i]))?;
        }
        write!(formatter, "\n")?;
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
                    E::Boulder,
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
            format!("{:?}", board),
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
            format!("{:?}", board),
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
            format!("{:?}", board),
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
            format!("{:?}", board),
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
            format!("{:?}", board),
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
            format!("{:?}", board),
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
            format!("{:?}", board),
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
        let (mut sokoban, directions) = parse_problem(data);
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
        let (mut sokoban, directions) = parse_problem(data);
        for direction in directions {
            sokoban.forward(direction);
        }

        verify_that!(sokoban.score(), eq(10092))
    }
}

fn parse_problem(s: &str) -> (Sokoban, Vec<Direction>) {
    let mut items = s.split("\n\n");
    let sokoban: Sokoban = items.next().expect("map").trim().parse().unwrap();
    let directions: Vec<Direction> = items
        .next()
        .expect("directions")
        .trim()
        .chars()
        .filter(|ch| !ch.is_ascii_whitespace())
        .map(|ch| Direction::try_from(ch).expect("Directions"))
        .collect();
    (sokoban, directions)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::io::read_to_string(std::io::stdin())?;
    let (mut sokoban, directions) = parse_problem(&data);
    for direction in directions {
        sokoban.forward(direction);
    }

    println!("Part 1: {}", sokoban.score());

    Ok(())
}
