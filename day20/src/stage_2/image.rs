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

    for row in normalized.iter() {
        let mut trimmed_row: Vec<Vec<String>> = Vec::new();

        for tile in row.iter() {
            trimmed_row.push(trim_tile(tile));
        }

        trimmed.push(trimmed_row);
    }

    return trimmed;
}

fn trim_tile(tile: &Vec<String>) -> Vec<String> {
    let mut trimmed: Vec<String> = Vec::new();

    let mut rows = tile.iter();
    rows.next(); // drop the first row
    rows.next_back(); // drop the last row

    for row in rows {
        let mut row_iter = row.chars();
        row_iter.next(); // drop the first char
        row_iter.next_back(); // drop the last char
        trimmed.push(row_iter.collect());
    }

    return trimmed;
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
    use std::fs;

    use super::*;
    use crate::stage_2::{tile::Tile, Layout};

    #[test]
    fn test_merge_image() -> Result<(), String> {
        let tile_1: Tile = "
            Tile 1:
            xxx
            x1x
            xxx
        "
        .parse()?;

        let tile_2: Tile = "
            Tile 2:
            xxx
            x2x
            xxx
        "
        .parse()?;

        let tile_3: Tile = "
            Tile 3:
            xxx
            x3x
            xxx
        "
        .parse()?;

        let tile_4: Tile = "
            Tile 4:
            xxx
            x4x
            xxx
        "
        .parse()?;

        let layout = Layout::from([([0, 0], tile_1), ([1, 0], tile_2), ([0, 1], tile_3), ([1, 1], tile_4)]);
        let actual = merge_image(&layout);
        #[rustfmt::skip]
        let expected = cleanup("
            12
            34
        ");

        assert_eq!(expected, actual, "\nexpected:\n{}\n\nactual:\n{}\n\n", expected, actual);
        Ok(())
    }

    #[test]
    fn test_3_by_3() {
        let input = parse_image_to_normalized(
            "
            xxx xxx xxx
            x.x x.x x.x
            xxx xxx xxx

            xxx xxx xxx
            x.x x.x x.x
            xxx xxx xxx

            xxx xxx xxx
            x.x x.x x.x
            xxx xxx xxx
        ",
        );

        let expected = parse_image_to_normalized(
            "
            . . .

            . . .

            . . .
        ",
        );

        let actual = trim_tiles(&input);

        assert_eq!(
            expected,
            actual,
            "\nexpected:\n{}\n\nactual:\n{}",
            format_normalized(&expected),
            format_normalized(&actual)
        );
    }

    #[test]
    fn metatest_parse_image_to_normalized_one_tile() {
        let actual = parse_image_to_normalized(
            "
          ...
          ...
          ...
        ",
        );

        #[rustfmt::skip]
        let expected = s(vec![vec![vec![
            "...",
            "...",
            "...",
        ]]]);

        assert_eq!(
            expected,
            actual,
            "\nexpected:\n{}\n\nactual:\n{}",
            format_normalized(&expected),
            format_normalized(&actual)
        );
    }

    #[test]
    fn test_parse_image_to_normalized_4_tiny_tiles() {
        let actual = parse_image_to_normalized(
            "
          . .

          . .
        ",
        );

        #[rustfmt::skip]
        let expected = s(vec![
            vec![vec!["."], vec!["."]],
            vec![vec!["."], vec!["."]],
        ]);

        assert_eq!(
            expected,
            actual,
            "\nexpected:\n{}\n\nactual:\n{}",
            format_normalized(&expected),
            format_normalized(&actual)
        );
    }

    #[test]
    fn test_example_1() {
        let input_str = fs::read_to_string("./test-data/example-ordered.txt").unwrap();
        let input = parse_image_to_normalized(&input_str);
        let actual = trim_tiles(&input);

        let expected_str = fs::read_to_string("./test-data/example-ordered-trimmed.txt").unwrap();
        let expected = parse_image_to_normalized(&expected_str);

        assert_eq!(
            expected,
            actual,
            "\nexpected:\n{}\n\nactual:\n{}",
            format_normalized(&expected),
            format_normalized(&actual)
        );
    }

    fn s(input: Vec<Vec<Vec<&str>>>) -> Vec<Vec<Vec<String>>> {
        stringify_normalized(input)
    }

    fn stringify_normalized(input: Vec<Vec<Vec<&str>>>) -> Vec<Vec<Vec<String>>> {
        input.iter().map(stringify_tile_row).collect()
    }

    fn stringify_tile_row(input: &Vec<Vec<&str>>) -> Vec<Vec<String>> {
        input.iter().map(stringify_tile).collect()
    }

    fn stringify_tile(input: &Vec<&str>) -> Vec<String> {
        input.iter().map(|l| l.to_string()).collect()
    }

    fn parse_image_to_normalized(input: &str) -> Vec<Vec<Vec<String>>> {
        let input = cleanup(input);
        let lines: Vec<&str> = input.lines().collect();
        let first_line = lines.first().unwrap();
        let dimensions = first_line.chars().filter(|&c| c == ' ').count() + 1;

        let mut tiles: Vec<Vec<Vec<String>>> = Vec::new();
        let mut tile_row = new_tile_row(dimensions);

        for line in lines {
            if line.is_empty() {
                tiles.push(tile_row);
                tile_row = new_tile_row(dimensions);
                continue;
            }

            for (tile, chunk) in tile_row.iter_mut().zip(line.split(' ')) {
                tile.push(chunk.to_string());
            }
        }

        tiles.push(tile_row);

        return tiles;
    }

    fn new_tile_row(num_tiles: usize) -> Vec<Vec<String>> {
        let mut tile_row: Vec<Vec<String>> = Vec::new();

        for _ in 0..num_tiles {
            tile_row.push(Vec::new());
        }

        tile_row
    }

    fn cleanup(input: &str) -> String {
        input.trim().lines().map(|l| l.trim()).collect::<Vec<&str>>().join("\n")
    }

    fn format_normalized(input: &Vec<Vec<Vec<String>>>) -> String {
        if input.is_empty() {
            return "".into();
        }

        to_lines(&input).join("\n")
    }
}
