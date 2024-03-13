use core::fmt;
use rand::Rng;
use std::collections::HashMap;
use std::slice::Iter;

use dialoguer::FuzzySelect;

#[derive(Clone, Copy)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    fn iterator() -> Iter<'static, Season> {
        static SEASONS: [Season; 4] = [
            Season::Spring,
            Season::Summer,
            Season::Autumn,
            Season::Winter,
        ];
        SEASONS.iter()
    }
}

impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Season::Spring => write!(f, "Spring"),
            Season::Summer => write!(f, "Summer"),
            Season::Autumn => write!(f, "Autumn"),
            Season::Winter => write!(f, "Winter"),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    TopLeft,
    Top,
    TopRight,
    BottomRight,
    Bottom,
    BottomLeft,
    Stay,
}

impl Direction {
    fn get_coord_modifier(dir: Direction) -> (i32, i32, i32) {
        match dir {
            Direction::TopLeft => (0, 1, -1),
            Direction::Top => (1, 0, -1),
            Direction::TopRight => (1, -1, 0),
            Direction::BottomRight => (0, -1, 1),
            Direction::Bottom => (-1, 0, 1),
            Direction::BottomLeft => (-1, 1, 0),
            Direction::Stay => (0, 0, 0),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::TopLeft => write!(f, "Top Left"),
            Direction::Top => write!(f, "Top"),
            Direction::TopRight => write!(f, "Top Right"),
            Direction::BottomRight => write!(f, "Bottom Right"),
            Direction::Bottom => write!(f, "Bottom"),
            Direction::BottomLeft => write!(f, "Bottom Left"),
            Direction::Stay => write!(f, "Stay"),
        }
    }
}

fn coord_addition(coord1: (i32, i32, i32), coord2: (i32, i32, i32)) -> (i32, i32, i32) {
    let x: i32 = coord1.0 + coord2.0;
    let y: i32 = coord1.1 + coord2.1;
    let z: i32 = coord1.2 + coord2.2;

    (x, y, z)
}

fn hex_wraparound(coords: (i32, i32, i32), dir: Direction) -> (i32, i32, i32) {
    match coords {
        (2, 0, -2) => {
            if dir == Direction::TopLeft {
                (2, -2, 0)
            } else if dir == Direction::Top {
                (-2, 0, 2)
            } else if dir == Direction::TopRight {
                (0, 2, -2)
            } else {
                coord_addition(coords, Direction::get_coord_modifier(dir))
            }
        }
        (2, -1, -1) => {
            if dir == Direction::Top {
                (-1, -1, 2)
            } else if dir == Direction::TopRight {
                (-1, 2, -1)
            } else {
                coord_addition(coords, Direction::get_coord_modifier(dir))
            }
        }
        (2, -2, 0) => {
            if dir == Direction::Top {
                (0, -2, 2)
            } else if dir == Direction::TopRight {
                (-2, 2, 0)
            } else if dir == Direction::BottomRight {
                (2, 0, -2)
            } else {
                coord_addition(coords, Direction::get_coord_modifier(dir))
            }
        }
        (1, -2, 1) => {
            if dir == Direction::TopRight {
                (-2, 1, 1)
            } else if dir == Direction::BottomRight {
                (1, 1, -2)
            } else {
                coord_addition(coords, Direction::get_coord_modifier(dir))
            }
        }
        (0, -2, 2) => {
            if dir == Direction::TopRight {
                (-2, 0, 2)
            } else if dir == Direction::BottomRight {
                (0, 2, -2)
            } else if dir == Direction::Bottom {
                (2, -2, 0)
            } else {
                coord_addition(coords, Direction::get_coord_modifier(dir))
            }
        }
        (-1, -1, 2) => {
            if dir == Direction::BottomRight {
                (-1, 2, -1)
            } else if dir == Direction::Bottom {
                (2, -1, -1)
            } else {
                coord_addition(coords, Direction::get_coord_modifier(dir))
            }
        }
        (-2, 0, 2) => {
            if dir == Direction::BottomRight {
                (-2, 2, 0)
            } else if dir == Direction::Bottom {
                (2, 0, -2)
            } else if dir == Direction::BottomLeft {
                (0, -2, 2)
            } else {
                coord_addition(coords, Direction::get_coord_modifier(dir))
            }
        }
        (-2, 1, 1) => {
            if dir == Direction::Bottom {
                (1, 1, -2)
            } else if dir == Direction::BottomLeft {
                (1, -2, 1)
            } else {
                coord_addition(coords, Direction::get_coord_modifier(dir))
            }
        }
        (-2, 2, 0) => {
            if dir == Direction::Bottom {
                (0, 2, -2)
            } else if dir == Direction::BottomLeft {
                (2, -2, 0)
            } else if dir == Direction::TopLeft {
                (-2, 0, 2)
            } else {
                coord_addition(coords, Direction::get_coord_modifier(dir))
            }
        }
        (-1, 2, -1) => {
            if dir == Direction::BottomLeft {
                (2, -1, -1)
            } else if dir == Direction::TopLeft {
                (-1, -1, 2)
            } else {
                coord_addition(coords, Direction::get_coord_modifier(dir))
            }
        }
        (0, 2, -2) => {
            if dir == Direction::BottomLeft {
                (2, 0, -2)
            } else if dir == Direction::TopLeft {
                (0, -2, 2)
            } else if dir == Direction::Top {
                (-2, 2, 0)
            } else {
                coord_addition(coords, Direction::get_coord_modifier(dir))
            }
        }
        (1, 1, -2) => {
            if dir == Direction::TopLeft {
                (1, -2, 1)
            } else if dir == Direction::Top {
                (-2, 1, 1)
            } else {
                coord_addition(coords, Direction::get_coord_modifier(dir))
            }
        }
        _ => {
            println!("Invalid outer hex supplied to wrapping check");
            coord_addition(coords, Direction::get_coord_modifier(dir))
        }
    }
}

fn get_new_coord(coords: (i32, i32, i32), season: Season) -> ((i32, i32, i32), Season) {
    let mut move_dir: Direction = get_move_direction();

    if move_dir != Direction::Stay && coords.0.abs() + coords.1.abs() + coords.2.abs() == 4 {
        match season {
            Season::Spring => {
                move_dir = spring_hex_wrapping(coords, move_dir);
            }
            Season::Summer => {
                move_dir = summer_hex_wrapping(coords, move_dir);
            }
            Season::Autumn => {
                move_dir = autumn_hex_wrapping(coords, move_dir);
            }
            Season::Winter => {
                move_dir = winter_hex_wrapping(coords, move_dir);
            }
        }

        (hex_wraparound(coords, move_dir), season)
    } else {
        (
            coord_addition(coords, Direction::get_coord_modifier(move_dir)),
            season,
        )
    }
}

fn spring_hex_wrapping(coords: (i32, i32, i32), dir: Direction) -> Direction {
    match coords {
        (2, 0, -2) => {
            if dir == Direction::Top {
                Direction::Stay
            } else {
                dir
            }
        }
        (2, -2, 0) => {
            if dir == Direction::TopRight {
                Direction::Stay
            } else {
                dir
            }
        }
        (1, -2, 1) => {
            if dir == Direction::TopRight {
                Direction::Stay
            } else {
                dir
            }
        }
        (-1, -1, 2) => {
            if dir == Direction::BottomRight {
                Direction::Stay
            } else {
                dir
            }
        }
        (-2, 0, 2) => {
            if dir == Direction::Bottom {
                Direction::Stay
            } else {
                dir
            }
        }
        (-2, 1, 1) => {
            if dir == Direction::BottomLeft || dir == Direction::Bottom {
                Direction::Stay
            } else {
                dir
            }
        }
        (-2, 2, -0) => {
            if dir == Direction::BottomLeft {
                Direction::Stay
            } else {
                dir
            }
        }
        (-1, 2, -1) => {
            if dir == Direction::TopLeft {
                Direction::Stay
            } else {
                dir
            }
        }
        (1, 1, -2) => {
            if dir == Direction::Top {
                Direction::Stay
            } else {
                dir
            }
        }
        _ => dir,
    }
}

fn summer_hex_wrapping(coords: (i32, i32, i32), dir: Direction) -> Direction {
    match coords {
        (2, 0, -2) => {
            if dir == Direction::Top {
                Direction::Stay
            } else {
                dir
            }
        }
        (2, -1, -1) => {
            if dir == Direction::Top {
                Direction::Stay
            } else {
                dir
            }
        }
        (2, -2, 0) => {
            if dir == Direction::TopRight {
                Direction::Stay
            } else {
                dir
            }
        }
        (-1, -1, 2) => {
            if dir == Direction::Bottom {
                Direction::Stay
            } else {
                dir
            }
        }
        (-2, 0, 2) => {
            if dir == Direction::Bottom {
                Direction::Stay
            } else {
                dir
            }
        }
        (-2, 1, 1) => {
            if dir == Direction::Bottom {
                Direction::Stay
            } else {
                dir
            }
        }
        (-2, 2, -0) => {
            if dir == Direction::BottomLeft {
                Direction::Stay
            } else {
                dir
            }
        }
        (1, 1, -2) => {
            if dir == Direction::Top {
                Direction::Stay
            } else {
                dir
            }
        }
        _ => dir,
    }
}

fn autumn_hex_wrapping(coords: (i32, i32, i32), dir: Direction) -> Direction {
    match coords {
        (2, 0, -2) => {
            if dir == Direction::TopLeft {
                Direction::Stay
            } else {
                dir
            }
        }
        (2, -2, 0) => {
            if dir == Direction::BottomRight {
                Direction::Stay
            } else {
                dir
            }
        }
        (1, -2, 1) => {
            if dir == Direction::BottomRight {
                Direction::Stay
            } else {
                dir
            }
        }
        (-2, 1, 1) => {
            if dir == Direction::Bottom {
                Direction::Stay
            } else {
                dir
            }
        }
        (-2, 2, 0) => {
            if dir == Direction::Bottom {
                Direction::Stay
            } else {
                dir
            }
        }
        (0, 2, -2) => {
            if dir == Direction::Top {
                Direction::Stay
            } else {
                dir
            }
        }
        (1, 1, -2) => {
            if dir == Direction::TopLeft || dir == Direction::Top {
                Direction::Stay
            } else {
                dir
            }
        }
        _ => dir,
    }
}

fn winter_hex_wrapping(coords: (i32, i32, i32), dir: Direction) -> Direction {
    match coords {
        (2, 0, -2) => {
            if dir == Direction::Top {
                Direction::Stay
            } else {
                dir
            }
        }
        (2, -2, 0) => {
            if dir == Direction::TopRight {
                Direction::Stay
            } else {
                dir
            }
        }
        (-2, 0, 2) => {
            if dir == Direction::Bottom {
                Direction::Stay
            } else {
                dir
            }
        }
        (-2, 1, 1) => {
            if dir == Direction::Bottom {
                Direction::Stay
            } else {
                dir
            }
        }
        (-2, 2, 0) => {
            if dir == Direction::BottomLeft {
                Direction::Stay
            } else {
                dir
            }
        }
        (1, 1, -2) => {
            if dir == Direction::Top {
                Direction::Stay
            } else {
                dir
            }
        }
        _ => dir,
    }
}

fn get_move_direction() -> Direction {
    let roll1 = rand::thread_rng().gen_range(1..=6);
    let roll2 = rand::thread_rng().gen_range(1..=6);

    let move_roll = roll1 + roll2;

    match move_roll {
        2 => Direction::TopLeft,
        3 => Direction::BottomLeft,
        4 => Direction::BottomLeft,
        5 => Direction::Bottom,
        6 => Direction::Bottom,
        7 => Direction::BottomRight,
        8 => Direction::TopRight,
        9 => Direction::Stay,
        10 => Direction::Stay,
        11 => Direction::Top,
        12 => Direction::TopLeft,
        _ => {
            println!("Invalid move roll");
            Direction::Stay
        }
    }
}

fn get_weather_string(
    hex: (i32, i32, i32),
    season: Season,
    spring_list: HashMap<(i32, i32, i32), String>,
    summer_list: HashMap<(i32, i32, i32), String>,
    autumn_list: HashMap<(i32, i32, i32), String>,
    winter_list: HashMap<(i32, i32, i32), String>,
) -> (
    String,
    HashMap<(i32, i32, i32), String>,
    HashMap<(i32, i32, i32), String>,
    HashMap<(i32, i32, i32), String>,
    HashMap<(i32, i32, i32), String>,
) {
    match season {
        Season::Spring => (
            spring_list.get(&hex).unwrap().to_string(),
            spring_list,
            summer_list,
            autumn_list,
            winter_list,
        ),
        Season::Summer => (
            summer_list.get(&hex).unwrap().to_string(),
            spring_list,
            summer_list,
            autumn_list,
            winter_list,
        ),
        Season::Autumn => (
            autumn_list.get(&hex).unwrap().to_string(),
            spring_list,
            summer_list,
            autumn_list,
            winter_list,
        ),
        Season::Winter => (
            winter_list.get(&hex).unwrap().to_string(),
            spring_list,
            summer_list,
            autumn_list,
            winter_list,
        ),
    }
}

fn main() {
    let mut spring: HashMap<(i32, i32, i32), String> = HashMap::from([
        ((0, 0, 0), String::from("Clear & Nippy")),
        ((1, 0, -1), String::from("Sleet")),
        ((1, -1, 0), String::from("Hail")),
        ((0, -1, 1), String::from("Cold Wafts of Mist")),
        ((-1, 0, 1), String::from("Sunny & Clear")),
        ((-1, 1, 0), String::from("Cloudy & Warm")),
        ((0, 1, -1), String::from("Nippy & Humid")),
        ((2, 0, -2), String::from("Heavy Rainfall")),
        ((2, -1, -1), String::from("Snowy Rain")),
        ((2, -2, 0), String::from("Windy & Snowy")),
        ((1, -2, 1), String::from("Heavy Snowfall")),
        ((0, -2, 2), String::from("Light Snowfall")),
        ((-1, -1, 2), String::from("Cloudy & Dry")),
        ((-2, 0, 2), String::from("Pleasantly Warm")),
        ((-2, 1, 1), String::from("Strong Pollen Drift")),
        ((-2, 2, 0), String::from("Hot & Dry")),
        ((-1, 2, -1), String::from("Warm & Humid")),
        ((0, 2, -2), String::from("Warm Drizzle")),
        ((1, 1, -2), String::from("Short Showers")),
    ]);

    let mut summer: HashMap<(i32, i32, i32), String> = HashMap::from([
        ((0, 0, 0), String::from("Pleasantly Warm")),
        ((1, 0, -1), String::from("Cloudy & Humid")),
        ((1, -1, 0), String::from("Cloudy & Windy")),
        ((0, -1, 1), String::from("Warm Breeze")),
        ((-1, 0, 1), String::from("Hot & Dry")),
        ((-1, 1, 0), String::from("Warm & Cloudy")),
        ((0, 1, -1), String::from("Short, Warm Showers")),
        ((2, 0, -2), String::from("Torrential Rain")),
        ((2, -1, -1), String::from("Warm Storm")),
        ((2, -2, 0), String::from("Fierce Wind")),
        ((1, -2, 1), String::from("Partly Cloudy & Nippy")),
        ((0, -2, 2), String::from("Clear & Nippy")),
        ((-1, -1, 2), String::from("Sunny & Clear")),
        ((-2, 0, 2), String::from("Dry Heat Surges")),
        ((-2, 1, 1), String::from("Hot & Windy")),
        ((-2, 2, 0), String::from("Hot & Muggy")),
        ((-1, 2, -1), String::from("Warm Drizzle")),
        ((0, 2, -2), String::from("Warm Rain")),
        ((1, 1, -2), String::from("Downpour")),
    ]);

    let mut autumn: HashMap<(i32, i32, i32), String> = HashMap::from([
        ((0, 0, 0), String::from("Humid & Cloudy")),
        ((1, 0, -1), String::from("Sunny & Clear")),
        ((1, -1, 0), String::from("Cold Wafts of Msit")),
        ((0, -1, 1), String::from("Thick Fog Soup")),
        ((-1, 0, 1), String::from("Rain & Fog")),
        ((-1, 1, 0), String::from("Rain & Gusts")),
        ((0, 1, -1), String::from("Sunny & Cloudy")),
        ((2, 0, -2), String::from("Indian Summer")),
        ((2, -1, -1), String::from("Sporadic Gusts")),
        ((2, -2, 0), String::from("Cold Winds")),
        ((1, -2, 1), String::from("Frosty & Cloudy")),
        ((0, -2, 2), String::from("Cloudy & Nippy")),
        ((-1, -1, 2), String::from("Windy & Clear")),
        ((-2, 0, 2), String::from("Short, Light Showers")),
        ((-2, 1, 1), String::from("Heavy Downpour")),
        ((-2, 2, 0), String::from("Rainy Windstorm")),
        ((-1, 2, -1), String::from("Drizzle")),
        ((0, 2, -2), String::from("Sunny & Nippy")),
        ((1, 1, -2), String::from("Pleasantly Warm")),
    ]);

    let mut winter: HashMap<(i32, i32, i32), String> = HashMap::from([
        ((0, 0, 0), String::from("Cold & Humid")),
        ((1, 0, -1), String::from("Cold Fog Wafts")),
        ((1, -1, 0), String::from("Cold Rain Showers")),
        ((0, -1, 1), String::from("Cold & Cloudy")),
        ((-1, 0, 1), String::from("Wet Snowfall")),
        ((-1, 1, 0), String::from("Snowy Rain")),
        ((0, 1, -1), String::from("Clear & Windy")),
        ((2, 0, -2), String::from("Sunny & Nippy")),
        ((2, -1, -1), String::from("Light Drizzle")),
        ((2, -2, 0), String::from("Heavy Rain")),
        ((1, -2, 1), String::from("Cold Winds")),
        ((0, -2, 2), String::from("Icy & Cloudy")),
        ((-1, -1, 2), String::from("Sleet")),
        ((-2, 0, 2), String::from("Light Snowfall")),
        ((-2, 1, 1), String::from("Windy & Snowy")),
        ((-2, 2, 0), String::from("Blizzard")),
        ((-1, 2, -1), String::from("Hail")),
        ((0, 2, -2), String::from("Cold & Clear")),
        ((1, 1, -2), String::from("Cloudy & Nippy")),
    ]);

    let mut hex: (i32, i32, i32) = (0, 0, 0);
    let mut season: Season = Season::iterator().as_slice()[FuzzySelect::new()
        .with_prompt("Choose starting season")
        .items(Season::iterator().as_slice())
        .interact()
        .unwrap()];
    let mut weather: String;
    (weather, spring, summer, autumn, winter) =
        get_weather_string(hex, season, spring, summer, autumn, winter);

    println!(
        "\nStarting Season: {}, Starting Weather: {}\n",
        season, weather
    );

    let operations: [&str; 3] = ["Generate Weather", "Change Season", "Exit"];

    loop {
        let operation = FuzzySelect::new()
            .with_prompt("Operation")
            .items(&operations)
            .interact()
            .unwrap();

        if operation == 0 {
            (hex, season) = get_new_coord(hex, season);
            (weather, spring, summer, autumn, winter) =
                get_weather_string(hex, season, spring, summer, autumn, winter);
            println!("\nSeason: {}, Weather: {}\n", season, weather);
        } else if operation == 1 {
            season = Season::iterator().as_slice()[FuzzySelect::new()
                .with_prompt("Choose starting season")
                .items(Season::iterator().as_slice())
                .interact()
                .unwrap()];
            hex = (0, 0, 0);
            (weather, spring, summer, autumn, winter) =
                get_weather_string(hex, season, spring, summer, autumn, winter);
            println!("\nSeason: {}, Weather: {}\n", season, weather);
        } else {
            break;
        }
    }
}
