use super::Layout;

pub fn merge_image(layout: &Layout) -> String {
    let normalized = normalize_layout(layout);
    let trimmed = trim_tiles(&normalized);
    let lines = to_lines(&trimmed);

    return lines.join("\n");
}

fn to_lines(trimmed: &Vec<Vec<Vec<String>>>) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let first_row_of_tiles = trimmed.first().unwrap();
    let num_rows_per_tile = first_row_of_tiles.first().unwrap().len();

    for row_of_tiles in trimmed.iter() {
        for y in 0..num_rows_per_tile {
            let mut line: String = "".to_string();

            for tile in row_of_tiles {
                line.push_str(tile.get(y).unwrap_or(&"".to_string()));
            }

            lines.push(line);
        }
    }

    return lines.iter().filter(|s| !s.is_empty()).cloned().collect();
}

fn trim_tiles(normalized: &Vec<Vec<Vec<String>>>) -> Vec<Vec<Vec<String>>> {
    let mut trimmed: Vec<Vec<Vec<String>>> = Vec::new();

    for (y, row) in normalized.iter().enumerate() {
        let mut new_row: Vec<Vec<String>> = Vec::new();

        for (x, lines) in row.iter().enumerate() {
            let mut lines = lines.clone();

            if y > 0 {
                lines = trim_first_line(lines);
            }

            if x > 0 {
                lines = trim_first_char_of_each_line(lines);
            }

            new_row.push(lines);
        }

        trimmed.push(new_row);
    }

    return trimmed;
}

fn trim_first_line(lines: Vec<String>) -> Vec<String> {
    lines.iter().skip(1).cloned().collect()
}

fn trim_first_char_of_each_line(lines: Vec<String>) -> Vec<String> {
    lines.iter().map(|line| line.chars().skip(1).collect()).collect()
}

fn normalize_layout(layout: &Layout) -> Vec<Vec<Vec<String>>> {
    let &max_x = layout.iter().map(|([x, _y], _)| x).max().unwrap_or(&0);
    let &max_y = layout.iter().map(|([_x, y], _)| y).max().unwrap_or(&0);

    let mut normalized: Vec<Vec<Vec<String>>> = Vec::new();

    for y in 0..=max_y {
        let mut row: Vec<Vec<String>> = Vec::new();

        for x in 0..=max_x {
            let tile = layout.get(&[x, y]).unwrap();
            row.push(tile.lines.clone());
        }

        normalized.push(row);
    }

    return normalized;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stage_2::{tile::Tile, Layout};

    #[test]
    fn test_merge_image() -> Result<(), String> {
        let tile_1: Tile = "
            Tile 1:
            1$
            .#
        "
        .parse()?;

        let tile_2: Tile = "
            Tile 2:
            $2
            #x
        "
        .parse()?;

        let tile_3: Tile = "
            Tile 3:
            .#
            3=
        "
        .parse()?;

        let tile_4: Tile = "
            Tile 4:
            #x
            =4
        "
        .parse()?;

        let layout = Layout::from([([0, 0], tile_1), ([1, 0], tile_2), ([0, 1], tile_3), ([1, 1], tile_4)]);
        let actual = merge_image(&layout);
        let expected = cleanup(
            "
            1$2
            .#x
            3=4
        ",
        );

        assert_eq!(expected, actual, "\nexpected:\n{}\n\nactual:\n{}\n\n", expected, actual);
        Ok(())
    }

    fn cleanup(input: &str) -> String {
        input.trim().lines().map(|l| l.trim()).collect::<Vec<&str>>().join("\n")
    }
}
