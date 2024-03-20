#![allow(unused_mut)]
#![allow(unused)]
// I have a pointer to slice of string literals, literals are supposed
// to be saved an the stack, will result vector's u8 still poin to stack?
// I not sure if whole module implementaion is good, it took horrible ammount of
// time test and type it. I don't like usage of Vectors and this implementatio
// requires many util methods, but I simply can not think of an algorithm
// even though I can see patterns for walls and corners.
// Anyway I created groups of cells and simply check which group given cell belongs to.
/// Creates main game board will use it to iterate over it.
fn create_board<'a>(sl: &'a [&str]) -> Vec<&'a u8> {
    if sl.len() == 0 {
        return vec![];
    }
    sl.iter()
        .flat_map(|c| c.as_bytes())
        .collect::<Vec<&'a u8>>()
}

// Methods to create board geometry need them to check if cell is on the wall or
// in the corner.
// Get cell indexes of a top wall of the  board
// row_length- length of a row or in our case len of any of &str in minefield
fn top_wall_idx(row_length: &usize, corners: &Vec<usize>) -> Vec<usize> {
    // let end = row_length - 1;
    (1..*row_length - 1)
        .filter(|n| !corners.contains(n))
        .collect::<Vec<usize>>()
}

fn bottom_wall_idx(row_length: &usize, total_length: &usize, corners: &Vec<usize>) -> Vec<usize> {
    let start = *total_length - *row_length + 1;
    let end = *total_length - 2;
    (start..=end)
        .filter(|n| !corners.contains(n))
        .collect::<Vec<usize>>()
}

fn left_wall_idx(row_length: &usize, total_length: &usize, corners: &Vec<usize>) -> Vec<usize> {
    (0..*total_length)
        .step_by(*row_length as usize)
        .filter(|n| !corners.contains(n))
        .collect::<Vec<usize>>()
}

fn right_wall_idx(row_length: &usize, total_length: &usize, corners: &Vec<usize>) -> Vec<usize> {
    let start = *row_length - 1;
    (start..*total_length)
        .step_by(*row_length as usize)
        .filter(|n| !corners.contains(n))
        .collect::<Vec<usize>>()
}

fn corners_idx(row_length: &usize, total_length: &usize) -> Vec<usize> {
    let left_top = row_length - 1;
    let left_bottom = total_length - row_length;
    let right_bottom = total_length - 1;
    vec![0, left_top, left_bottom, right_bottom]
}

/// top left corner indexes to check
fn left_top_corner_idx(row_length: &usize, total_length: &usize) -> Vec<usize> {
    if row_length == total_length {
        return vec![1];
    }
    vec![1, *row_length, row_length + 1]
}

// will handle one special case presume if row_length is less than total it means
// that everything is OK.
/// top right corner indexes to check
fn right_top_corner_idx(row_length: &usize, total_length: &usize) -> Vec<usize> {
    if row_length == total_length {
        return vec![row_length - 2];
    }
    let i = row_length - 1;
    let one = i - 1;
    let two = i + row_length;
    let three = two - 1;
    vec![one, two, three]
}

// no special cases here if thing has a bottom so it has a top then.
/// bottom left corner indexes to check
fn left_bottom_corner_idx(row_length: &usize, total_length: &usize) -> Vec<usize> {
    let i = total_length - row_length;
    let one = i + 1;
    let two = i - row_length;
    let three = two + 1;
    vec![one, two, three]
}

/// bottom right corner indexes to check
fn right_bottom_corner_idx(row_length: &usize, total_length: &usize) -> Vec<usize> {
    let i = total_length - 1;
    let one = i - 1;
    let two = i - row_length;
    let three = two - 1;
    vec![one, two, three]
}

/// Indexes to check when cell is on the top wall but not corner
fn top_wall_cell_idx(i: usize, row_length: &usize) -> Vec<usize> {
    let left = i - 1;
    let right = i + 1;
    let middle = i + row_length;
    let m_left = middle - 1;
    let m_right = middle + 1;
    vec![left, right, middle, m_left, m_right] // [0, 2 , 4, 3, 5]
}

/// Indexes to check when cell is on the bottom wall but not corner
fn bottom_wall_cell_idx(i: usize, row_length: &usize) -> Vec<usize> {
    let left = i - 1;
    let right = i + 1;
    let middle = i - row_length;
    let m_left = middle - 1;
    let m_right = middle + 1;
    vec![left, right, middle, m_left, m_right] // [6, 8, 4, 3, 5]
}

/// Indexes to check when cell is on the left wall but not corner
fn left_wall_cell_idx(i: usize, row_length: &usize) -> Vec<usize> {
    let top = i - row_length;
    let bottom = i + row_length;
    let middle = i + 1;
    let m_top = middle - row_length;
    let m_bottom = middle + row_length;
    vec![top, bottom, middle, m_top, m_bottom] // [0,6, 1, 4, 7]
}
/// Indexes to check when cell is on the right wall but not corner
fn right_wall_cell_idx(i: usize, row_length: &usize) -> Vec<usize> {
    let top = i - row_length;
    let bottom = i + row_length;
    let middle = i - 1;
    let m_top = middle - row_length;
    let m_bottom = middle + row_length;
    vec![top, bottom, middle, m_top, m_bottom] // [2, 8, 4, 1, 7]
}

/// Indexes to check when cell is not on the wall and not in the corner
fn middle_cell_idx(i: usize, row_length: &usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::with_capacity(8);
    res.push(i - 1);
    res.push(i + 1);
    let tprs = i - row_length - 1;
    res.push(tprs);
    res.push(tprs + 1);
    res.push(tprs + 2);
    let btrs = i + row_length + 1;
    res.push(btrs);
    res.push(btrs - 1);
    res.push(btrs - 2);
    res
}

/// calculate number of stars in cells that surround cell.
/// indexes is vector of cells indixes that surround cell.
fn calculate_stars<'a>(indixes: Vec<usize>, board: &Vec<&'a u8>) -> u8 {
    let mut res = 0_u8;
    indixes.iter().for_each(|idx| {
        if board[*idx] == &42 {
            res += 1
        }
    });
    res
}

/// repetitious code updating result string with digit of empty space
fn update_res_string(res_string: &mut String, stars: u8) {
    if stars > 0 {
        res_string.push_str(&format!("{stars}"));
    } else {
        res_string.push_str(" ");
    }
}

/// handles special cases with single column or single row
fn handle_single(step: usize, mut slices: usize, total: &usize, board: &[&u8]) -> Vec<String> {
    let mut res_string = String::with_capacity(*total);

    for (idx, v) in board.iter().enumerate() {
        if **v == 42 {
            res_string.push_str("*");
            continue;
        }
        match idx {
            i if i == 0 => {
                if total >= &1 {
                    if board[idx + 1] == &42 {
                        res_string.push_str("1");
                    } else {
                        res_string.push_str(" ");
                    }
                }
            }
            i if (1..*total).contains(&i) => {
                let mut stars = 0;
                if idx < total - 1 {
                    if board[idx - 1] == &42 {
                        stars += 1;
                    }
                    if board[idx + 1] == &42 {
                        stars += 1;
                    }
                    update_res_string(&mut res_string, stars);
                } else if idx == total - 1 {
                    if board[idx - 1] == &42 {
                        res_string.push_str("1");
                    } else {
                        res_string.push_str(" ");
                    }
                }
            }
            _ => (),
        }
    }
    let mut res: Vec<String> = Vec::new();
    if step == 1 && slices > 1 {
        while slices > 0 {
            let (s, tail) = res_string.split_at(step);
            res.push(s.to_string());
            res_string = String::from(tail);
            slices -= 1;
        }
    } else if step > 1 && slices == 1 {
        res.push(res_string.to_string());
    }
    return res;
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    if minefield.len() == 0 {
        return res;
    }

    // wouldn't it be great to convert it to array?
    // Vec stored in the heap.
    let mut board = create_board(minefield);
    let total = board.len();
    //
    // check special cases. No stars at all and case when all stars.
    {
        let stars = board.iter().filter(|n| ***n == 42).count();
        if stars == total {
            return minefield
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
        } else if stars == 0_usize {
            return minefield
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
        }
    }

    // at this point at least one str must be in slice
    let first = minefield[0];
    // used to calculate indexes
    let step = first.len();
    // used to create result Vec<String>
    let mut slices = minefield.len();

    // special cases when board in one column or in one row.
    if step == 1 || slices == 1 {
        return handle_single(step, slices, &total, &board);
    }

    // board geometry walls and corners
    let corners = corners_idx(&step, &total);
    let top_wall = top_wall_idx(&step, &corners);
    let bottom_wall = bottom_wall_idx(&step, &total, &corners);
    let left_wall = left_wall_idx(&step, &total, &corners);
    let right_wall = right_wall_idx(&step, &total, &corners);
    // this string holds whole result as a single string
    let mut res_string = String::with_capacity(total);

    for (idx, v) in board.iter().enumerate() {
        if **v == 42 {
            res_string.push_str("*");
            continue;
        }
        match idx {
            ref idx if corners.contains(idx) => {
                // what corner?
                for (i, c) in corners.iter().enumerate() {
                    if idx == c {
                        if i == 0 {
                            let stars = calculate_stars(left_top_corner_idx(&step, &total), &board);
                            update_res_string(&mut res_string, stars);
                        } else if i == 1 {
                            let stars =
                                calculate_stars(right_top_corner_idx(&step, &total), &board);
                            update_res_string(&mut res_string, stars);
                        } else if i == 2 {
                            let stars =
                                calculate_stars(left_bottom_corner_idx(&step, &total), &board);
                            update_res_string(&mut res_string, stars);
                        } else {
                            let stars =
                                calculate_stars(right_bottom_corner_idx(&step, &total), &board);
                            update_res_string(&mut res_string, stars);
                        }
                    }
                }
            }
            ref idx if top_wall.contains(idx) => {
                let stars = calculate_stars(top_wall_cell_idx(*idx, &step), &board);
                update_res_string(&mut res_string, stars);
            }
            ref idx if left_wall.contains(idx) => {
                let stars = calculate_stars(left_wall_cell_idx(*idx, &step), &board);
                update_res_string(&mut res_string, stars);
            }
            ref idx if right_wall.contains(idx) => {
                let stars = calculate_stars(right_wall_cell_idx(*idx, &step), &board);
                update_res_string(&mut res_string, stars);
            }
            ref idx if bottom_wall.contains(idx) => {
                let stars = calculate_stars(bottom_wall_cell_idx(*idx, &step), &board);
                update_res_string(&mut res_string, stars);
            }
            _ => {
                let stars = calculate_stars(middle_cell_idx(idx, &step), &board);
                update_res_string(&mut res_string, stars);
            }
        }
    }
    // let mut slices = minefield.len();
    while slices > 0 {
        let (s, tail) = res_string.split_at(step);
        res.push(s.to_string());
        res_string = String::from(tail);
        slices -= 1;
    }

    res
}

#[cfg(test)]
mod testing {
    use super::*;

    // test for indexes that belong to walls
    #[test]
    #[ignore]
    fn test_top_wall_idx() {
        let corners = vec![0, 2, 6, 8];
        assert_eq!(vec![1], top_wall_idx(&3, &corners));
    }
    #[test]
    #[ignore]
    fn test_bottom_wall_idx() {
        let corners = vec![0, 2, 6, 8];
        assert_eq!(vec![7], bottom_wall_idx(&3, &9, &corners));
    }
    #[test]
    #[ignore]
    fn test_left_wall_idx() {
        let corners = vec![0, 2, 6, 8];
        assert_eq!(vec![3], left_wall_idx(&3, &9, &corners));
        let corners = vec![0, 2, 9, 11];
        assert_eq!(vec![3, 6], left_wall_idx(&3, &12, &corners));
    }

    #[test]
    #[ignore]
    fn test_right_wall_idx() {
        let corners = vec![0, 2, 6, 8];
        assert_eq!(vec![5], right_wall_idx(&3, &9, &corners));
        let corners = vec![0, 2, 9, 11];
        assert_eq!(vec![5, 8], right_wall_idx(&3, &12, &corners));
    }

    // test for indexes to check when cell is in the corner
    #[test]
    #[ignore]
    fn test_corners_idx() {
        assert_eq!(vec![0, 2, 6, 8], corners_idx(&3, &9));
        assert_eq!(vec![0, 2, 9, 11], corners_idx(&3, &12));
    }

    #[test]
    #[ignore]
    fn test_left_top_corner_idx() {
        assert_eq!(vec![1, 3, 4], left_top_corner_idx(&3, &9));
        assert_eq!(vec![1], left_top_corner_idx(&3, &3));
    }

    #[test]
    #[ignore]
    fn test_right_top_corner_idx() {
        assert_eq!(vec![1, 5, 4], right_top_corner_idx(&3, &9));
        assert_eq!(vec![1], right_top_corner_idx(&3, &3));
    }

    #[test]
    #[ignore]
    fn test_bottom_left_corner_idx() {
        assert_eq!(vec![7, 3, 4], left_bottom_corner_idx(&3, &9));
    }

    #[test]
    #[ignore]
    fn test_bottom_right_corner_idx() {
        assert_eq!(vec![7, 5, 4], right_bottom_corner_idx(&3, &9));
    }

    // test creation of indexes vector for cells that are on the wall
    // but not in the corner
    #[test]
    #[ignore]
    fn test_top_wall_cell_idx() {
        assert_eq!(vec![0, 2, 4, 3, 5], top_wall_cell_idx(1, &3));
    }

    #[test]
    #[ignore]
    fn test_bottom_wall_cell_idx() {
        assert_eq!(vec![6, 8, 4, 3, 5], bottom_wall_cell_idx(7, &3));
    }

    #[test]
    #[ignore]
    fn test_left_wall_cell_idx() {
        assert_eq!(vec![0, 6, 4, 1, 7], left_wall_cell_idx(3, &3));
    }

    #[test]
    #[ignore]
    fn test_right_wall_cell_idx() {
        assert_eq!(vec![2, 8, 4, 1, 7], right_wall_cell_idx(5, &3));
    }

    // Indexes to check when cell is not on the wall and not in the corner
    #[test]
    #[ignore]
    fn test_middle_cell_idx() {
        assert_eq!(vec![3, 5, 0, 1, 2, 8, 7, 6], middle_cell_idx(4, &3));
    }
}
