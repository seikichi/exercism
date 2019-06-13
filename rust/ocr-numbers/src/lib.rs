#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let input = input.split('\n').collect::<Vec<_>>();

    if input.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(input.len()));
    }

    if let Some(&line) = input.iter().find(|line| line.len() % 3 != 0) {
        return Err(Error::InvalidColumnCount(line.len()));
    }

    let mut results = vec![];

    for lines in (&input[..]).chunks(4) {
        let l1 = lines[0].chars().collect::<Vec<_>>();
        let l2 = lines[1].chars().collect::<Vec<_>>();
        let l3 = lines[2].chars().collect::<Vec<_>>();
        let number = (0..l1.len())
            .step_by(3)
            .map(|i| match (&l1[i..i + 3], &l2[i..i + 3], &l3[i..i + 3]) {
                ([' ', '_', ' '], ['|', ' ', '|'], ['|', '_', '|']) => '0',
                ([' ', ' ', ' '], [' ', ' ', '|'], [' ', ' ', '|']) => '1',
                ([' ', '_', ' '], [' ', '_', '|'], ['|', '_', ' ']) => '2',
                ([' ', '_', ' '], [' ', '_', '|'], [' ', '_', '|']) => '3',
                ([' ', ' ', ' '], ['|', '_', '|'], [' ', ' ', '|']) => '4',
                ([' ', '_', ' '], ['|', '_', ' '], [' ', '_', '|']) => '5',
                ([' ', '_', ' '], ['|', '_', ' '], ['|', '_', '|']) => '6',
                ([' ', '_', ' '], [' ', ' ', '|'], [' ', ' ', '|']) => '7',
                ([' ', '_', ' '], ['|', '_', '|'], ['|', '_', '|']) => '8',
                ([' ', '_', ' '], ['|', '_', '|'], [' ', '_', '|']) => '9',
                _ => '?',
            })
            .collect::<String>();
        results.push(number);
    }

    Ok(results.join(","))
}
