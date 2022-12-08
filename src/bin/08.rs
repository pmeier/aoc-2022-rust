fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let row_grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();
    let col_grid = transpose(&row_grid);
    (row_grid, col_grid)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (row_grid, col_grid) = parse(input);

    let num_rows = row_grid.len();
    let num_cols = col_grid.len();

    fn check(height: &usize, other_heights: &[usize]) -> bool {
        other_heights
            .iter()
            .all(|other_height: &usize| other_height < height)
    }

    let mut num_visible_trees = 2 * (num_rows + num_cols) - 4;
    for i in 1..num_rows - 1 {
        for j in 1..num_cols - 1 {
            let height = row_grid[i][j];
            if
            // left
            check(&height, &row_grid[i][..j])
                // right
                || check(&height, &row_grid[i][j + 1..])
                // top
                || check(&height, &col_grid[j][..i])
                // bottom
                || check(&height, &col_grid[j][i + 1..])
            {
                num_visible_trees += 1;
            }
        }
    }
    Some(num_visible_trees as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (row_grid, col_grid) = parse(input);

    let num_rows = row_grid.len();
    let num_cols = col_grid.len();

    fn num_trees<'a, I>(height: &usize, other_heights: I) -> usize
    where
        I: Iterator<Item = &'a usize>,
    {
        let mut num = 0;
        for other_height in other_heights {
            num += 1;
            if other_height >= height {
                break;
            }
        }
        num
    }

    let mut scenic_scores = vec![];
    for i in 1..num_rows - 1 {
        for j in 1..num_cols - 1 {
            let height = row_grid[i][j];

            scenic_scores.push(
                //left
                num_trees(&height, row_grid[i][..j].iter().rev())
                // right
                * num_trees(&height, row_grid[i][j+1..].iter())
                // top
                * num_trees(&height, col_grid[j][..i].iter().rev())
                // bottom
                * num_trees(&height, col_grid[j][i+1..].iter()),
            );
        }
    }

    Some(*scenic_scores.iter().max().unwrap() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
