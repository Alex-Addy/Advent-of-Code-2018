use std::cmp::max;
use std::io::{BufRead, BufReader, Read};
use std::num::ParseIntError;
use std::ops::Deref;

pub fn work<R: Read>(r: R) {
    let reader = BufReader::new(r);

    let lines: Vec<_> = reader.lines().map(|res| res.unwrap()).collect();

    match count_overlapping(&lines) {
        Ok(over) => info!("Part 1: {}", over),
        Err(err) => error!("Failed part 1: {}", err),
    }
}

fn count_overlapping<T: Deref<Target = str>>(lines: &[T]) -> Result<usize, String> {
    let mut claims = Vec::new();

    for line in lines {
        let claim = Claim::try_parse(line)?;
        claims.push(claim);
    }

    debug!("Got {} claims", claims.len());

    let mut height = 0;
    let mut width = 0;
    for c in &claims {
        height = max(height, c.top + c.height);
        width = max(width, c.left + c.width);
    }

    debug!("Generating grid {}x{}", height, width);

    let mut grid = vec![vec![0u8; width]; height];
    for c in &claims {
        for h in 0..c.height {
            for w in 0..c.width {
                grid[c.top + h][c.left + w] += 1;
            }
        }
    }

    debug!("Grid generated, counting overlap");

    let mut overlap = 0;
    for row in &grid {
        for point in row {
            if point >= &2 {
                overlap += 1;
            }
        }
    }

    Ok(overlap)
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    /// Attempt to parse from a string.
    ///
    /// Expected format is "#<id> @ <left>,<top>: <width>x<height>".
    /// For example: "#123 @ 3,2: 5x4" is a valid claim.
    fn try_parse(s: &str) -> Result<Self, String> {
        let mut iter = s
            .split(|c| !char::is_digit(c, 10))
            .filter(|s| !s.is_empty());
        let id = iter
            .next()
            .ok_or("couldn't get id")?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let left = iter
            .next()
            .ok_or("failed to get left edge")?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let top = iter
            .next()
            .ok_or("failed to get top edge")?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let width = iter
            .next()
            .ok_or("failed to get width")?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let height = iter
            .next()
            .ok_or("failed to get height")?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;

        Ok(Claim {
            id,
            left,
            top,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_pattern() {
        let line = "#1 @ 1,3: 4x4";
        let list: Vec<_> = line
            .split(|c| !char::is_digit(c, 10))
            .filter(|s| !s.is_empty())
            .collect();
        let expected = vec!["1", "1", "3", "4", "4"];
        assert_eq!(expected, list);
    }

    #[test]
    fn test_claim_try_parse() {
        let expected = Claim {
            id: 1,
            left: 2,
            top: 3,
            width: 4,
            height: 5,
        };
        let line = "#1 @ 2,3: 4x5";
        assert_eq!(expected, Claim::try_parse(line).unwrap());
    }

    #[test]
    fn overlapping_example() {
        let boxes = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
        assert_eq!(Ok(4), count_overlapping(&boxes));
    }
}
