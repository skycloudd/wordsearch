use std::fs::read_to_string;

fn main() {
    const WIDTH: usize = 20;
    const HEIGHT: usize = 20;

    let text = read_to_string(
        std::env::args()
            .nth(1)
            .expect("word list as cmd line argument"),
    )
    .expect("cant read file");

    let words: Vec<&str> = text
        .lines()
        .flat_map(|line| {
            if is_bad_line(line, WIDTH, HEIGHT) {
                eprintln!("skipping line '{}'", line);

                None
            } else {
                Some(line)
            }
        })
        .collect();

    if words.is_empty() {
        eprintln!("no words to place");
        return;
    }

    let grid = Grid::<WIDTH, HEIGHT>::construct(&words);

    println!("{}", html(&grid, words));
}

fn html<const WIDTH: usize, const HEIGHT: usize>(
    grid: &Grid<WIDTH, HEIGHT>,
    words: Vec<impl core::fmt::Display + Ord>,
) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>");
    html.push_str("<html>");
    html.push_str("<head>");
    html.push_str("<style>");
    html.push_str("table {border-collapse: collapse;}");
    html.push_str("td{border: 1px solid black;width:20px;height:20px;text-align:center;}");
    html.push_str(".column{float:left;width:50%;}");
    html.push_str(".row:after{content:\"\";display:table;clear:both;}");
    html.push_str("</style>");
    html.push_str("</head>");
    html.push_str("<body>");

    html.push_str("<h1>Kyra's Word Search :3</h1>");

    let theme = std::env::args().nth(2).expect("theme as cmd line argument");

    html.push_str(&format!(
        "<p>Today's theme is: <strong>{theme}</strong></p>"
    ));

    html.push_str("<div class=\"row\">");

    html.push_str("<div class=\"column\">");

    html.push_str("<table>");

    for row in grid.cells {
        html.push_str("<tr>");

        for cell in row.iter() {
            html.push_str("<td>");
            html.push(*cell);
            html.push_str("</td>");
        }

        html.push_str("</tr>");
    }

    html.push_str("</table>");

    html.push_str("</div>");

    html.push_str("<div class=\"column\">");

    html.push_str("<ul>");

    let mut words = words;
    words.sort_unstable();

    for word in words {
        html.push_str("<li>");
        html.push_str(&word.to_string());
        html.push_str("</li>");
    }

    html.push_str("</ul>");

    html.push_str("</div>");

    html.push_str("</body>");
    html.push_str("</html>");

    html
}

fn is_bad_line(line: &str, width: usize, height: usize) -> bool {
    if line.is_empty() {
        eprintln!("empty line");
        return true;
    }

    if line.len() > width || line.len() > height {
        eprintln!("line too long");
        return true;
    }

    if line.chars().any(|c| !c.is_ascii_alphabetic()) {
        eprintln!("non-alphabetic character");
        return true;
    }

    false
}

#[derive(Debug)]
struct Grid<const WIDTH: usize, const HEIGHT: usize> {
    cells: [[char; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Grid<WIDTH, HEIGHT> {
    fn construct(words: &[&str]) -> Self {
        let cells = loop {
            let mut cells = [[None; WIDTH]; HEIGHT];

            match make_cells(words, &mut cells) {
                Ok(()) => {
                    break cells;
                }
                Err(()) => {
                    eprintln!("retrying");
                    continue;
                }
            }
        };

        Grid {
            cells: cells.map(|row| row.map(|cell| cell.unwrap())),
        }
    }
}

fn make_cells<const WIDTH: usize, const HEIGHT: usize>(
    words: &[&str],
    cells: &mut [[Option<char>; WIDTH]; HEIGHT],
) -> Result<(), ()> {
    for word in words {
        eprintln!("- {}", word);

        place_word(word, cells)?;
    }

    for row in cells.iter_mut() {
        for cell in row.iter_mut() {
            if cell.is_none() {
                loop {
                    let c = (rand::random::<u8>() % 26 + b'a').into();

                    if (c != 'a' && c != 'e' && c != 'i' && c != 'o' && c != 'u')
                        | (rand::random::<f32>() < 0.8)
                    {
                        *cell = Some(c);
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

fn place_word<const WIDTH: usize, const HEIGHT: usize>(
    word: &str,
    cells: &mut [[Option<char>; WIDTH]; HEIGHT],
) -> Result<(), ()> {
    let direction = match rand::random::<u8>() % 9 {
        0..=1 => Direction::Horizontal,
        2..=4 => Direction::Vertical,
        _ => Direction::Diagonal,
    };

    let (from_x, from_y) = find_empty_space(cells, word, direction)?;

    eprintln!(
        "placing word '{}' at ({}, {}) in direction {:?}",
        word, from_x, from_y, direction
    );

    let mut x = from_x;
    let mut y = from_y;

    for c in word.chars() {
        cells[y][x] = Some(c);

        match direction {
            Direction::Horizontal => x += 1,
            Direction::Vertical => y += 1,
            Direction::Diagonal => {
                x += 1;
                y += 1;
            }
        }
    }

    Ok(())
}

fn find_empty_space<const WIDTH: usize, const HEIGHT: usize>(
    cells: &[[Option<char>; WIDTH]; HEIGHT],
    word: &str,
    direction: Direction,
) -> Result<(usize, usize), ()> {
    let word_len = word.len();

    let mut attempts = 0;

    loop {
        let from_x = rand::random::<usize>() % WIDTH;
        let from_y = rand::random::<usize>() % HEIGHT;

        let (to_x, to_y) = match direction {
            Direction::Horizontal => (from_x + word_len, from_y),
            Direction::Vertical => (from_x, from_y + word_len),
            Direction::Diagonal => (from_x + word_len, from_y + word_len),
        };

        if to_x <= WIDTH && to_y <= HEIGHT {
            let mut is_empty = true;

            let coords: Vec<(usize, usize)> = match direction {
                Direction::Horizontal => (from_x..to_x).zip(std::iter::repeat(from_y)).collect(),
                Direction::Vertical => (std::iter::repeat(from_x)).zip(from_y..to_y).collect(),
                Direction::Diagonal => (from_x..to_x).zip(from_y..to_y).collect(),
            };

            eprintln!("{:?}", coords);

            for (index, (x, y)) in coords.into_iter().take(word_len).enumerate() {
                if cells[y][x].is_some() && cells[y][x] != Some(word.chars().nth(index).unwrap()) {
                    is_empty = false;
                    break;
                }
            }

            if is_empty {
                return Ok((from_x, from_y));
            }
        }

        attempts += 1;

        if attempts > 100 {
            eprintln!("too many attempts");
            return Err(());
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}
