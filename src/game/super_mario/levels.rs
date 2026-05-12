use super::constants::{LEVEL_COLS, LEVEL_ROWS};

pub fn build_level_1_1() -> Vec<Vec<char>> {
    let cols = LEVEL_COLS as usize;
    let rows = LEVEL_ROWS as usize;
    let mut g: Vec<Vec<char>> = (0..rows).map(|_| vec![' '; cols]).collect();

    for c in 0..cols {
        g[13][c] = 'G';
    }
    for c in 84..86 {
        g[13][c] = ' ';
    }
    for c in 124..126 {
        g[13][c] = ' ';
    }

    g[12][1] = 'M';
    for c in [14usize, 28, 50, 52, 110, 154] {
        g[12][c] = 'g';
    }
    for top in [66usize, 76, 86, 96] {
        g[12][top] = 'p';
        g[12][top + 1] = 'p';
        g[11][top] = 'p';
        g[11][top + 1] = 'p';
    }
    g[12][146] = 'C';

    g[10][30] = 'Q';
    g[10][36] = 'B';
    g[10][38] = 'U';
    g[10][40] = 'B';
    g[10][101] = 'B';
    g[10][103] = 'U';
    g[10][105] = 'B';

    g[9][131] = 'S';
    g[9][132] = 'S';
    g[9][133] = 'S';
    g[9][134] = 'S';
    g[8][132] = 'S';
    g[8][133] = 'S';
    g[8][134] = 'S';
    g[7][133] = 'S';
    g[7][134] = 'S';
    g[6][134] = 'S';

    g[7][138] = 'F';

    g
}

pub fn build_level_1_2() -> Vec<Vec<char>> {
    let cols = LEVEL_COLS as usize;
    let rows = LEVEL_ROWS as usize;
    let mut g: Vec<Vec<char>> = (0..rows).map(|_| vec![' '; cols]).collect();

    for c in 0..cols {
        g[13][c] = 'G';
    }
    for c in 0..145 {
        g[0][c] = 'B';
        g[1][c] = 'B';
    }

    g[12][1] = 'M';

    g[3][20] = 'Q';
    g[3][24] = 'U';
    g[3][50] = 'B';
    g[3][52] = 'Q';
    g[3][54] = 'B';
    g[3][86] = 'B';
    g[3][88] = '*';
    g[3][90] = 'B';

    for &c in [16usize, 36, 60, 90, 120].iter() {
        g[12][c] = 'k';
    }

    for &top in [70usize, 100].iter() {
        g[12][top] = 'p';
        g[12][top + 1] = 'p';
        g[11][top] = 'p';
        g[11][top + 1] = 'p';
    }

    g[7][148] = 'F';
    g[12][156] = 'C';

    g
}

pub fn build_level_1_3() -> Vec<Vec<char>> {
    let cols = LEVEL_COLS as usize;
    let rows = LEVEL_ROWS as usize;
    let mut g: Vec<Vec<char>> = (0..rows).map(|_| vec![' '; cols]).collect();

    for c in 0..12 {
        g[13][c] = 'G';
    }
    for c in 138..cols {
        g[13][c] = 'G';
    }

    g[12][1] = 'M';

    let small_islands: [(usize, usize, usize); 12] = [
        (15, 18, 6),
        (22, 24, 4),
        (28, 30, 6),
        (38, 42, 7),
        (50, 52, 5),
        (60, 64, 7),
        (74, 78, 6),
        (86, 88, 4),
        (94, 98, 6),
        (108, 112, 5),
        (118, 122, 4),
        (128, 134, 6),
    ];
    for (start, end, h) in small_islands {
        for c in start..=end {
            g[h][c] = 'S';
        }
    }

    g[7][33] = 'L';
    g[7][68] = 'L';
    g[7][102] = 'L';

    g[5][39] = 'r';
    g[6][75] = 'r';
    g[5][95] = 'r';
    g[12][140] = 'r';
    g[12][148] = 'r';

    g[6][26] = 'U';
    g[5][125] = '*';

    g[7][156] = 'F';

    g
}

pub fn build_level_1_4() -> Vec<Vec<char>> {
    let cols = LEVEL_COLS as usize;
    let rows = LEVEL_ROWS as usize;
    let mut g: Vec<Vec<char>> = (0..rows).map(|_| vec![' '; cols]).collect();

    for c in 0..cols {
        g[13][c] = 'D';
    }

    for &(start, end) in [(35usize, 39usize), (70, 75), (110, 115)].iter() {
        for c in start..=end {
            g[13][c] = 'l';
            g[12][c] = 'l';
        }
    }

    g[12][1] = 'M';

    for c in 28..32 {
        g[10][c] = 'D';
    }
    for c in 44..48 {
        g[8][c] = 'D';
    }
    for c in 60..64 {
        g[10][c] = 'D';
    }
    for c in 80..84 {
        g[10][c] = 'D';
    }
    for c in 92..98 {
        g[8][c] = 'D';
    }

    for c in 0..30 {
        g[2][c] = 'D';
    }
    for c in 100..145 {
        g[2][c] = 'D';
    }

    g[12][20] = 'g';
    g[12][55] = 'k';
    g[12][88] = 'g';
    g[12][125] = 'k';

    g[12][140] = 'b';

    g[12][156] = 'A';

    g
}
