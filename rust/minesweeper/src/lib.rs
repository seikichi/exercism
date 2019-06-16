pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut minefield = minefield
        .iter()
        .map(|&s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for i in 0..minefield.len() {
        for j in 0..minefield[i].len() {
            if minefield[i][j] != ' ' {
                continue;
            }
            let mut mines = 0;

            for i in &[i.checked_sub(1), Some(i), Some(i + 1)] {
                for j in &[j.checked_sub(1), Some(j), Some(j + 1)] {
                    if let (Some(i), Some(j)) = (i, j) {
                        if *i < minefield.len()
                            && *j < minefield[*i].len()
                            && minefield[*i][*j] == '*'
                        {
                            mines += 1;
                        }
                    }
                }
            }
            if mines > 0 {
                minefield[i][j] = std::char::from_digit(mines, 10).unwrap();
            }
        }
    }

    minefield
        .iter()
        .map(|v| v.iter().collect::<String>())
        .collect::<Vec<_>>()
}
