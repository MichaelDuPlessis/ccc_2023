pub fn try_solve(amounts: (usize, usize, usize)) {
    let mut num_rocks = amounts.0;
    let mut num_papers = amounts.1;
    let mut num_scissors = amounts.2;

    let total = num_rocks + num_papers + num_scissors;

    let top_number = total / 4;

    let mut matches = String::from("");
    let mut top_scissors = 0;

    for _ in 0..top_number {
        if num_scissors > 0 {
            matches.push('S');
            num_scissors -= 1;
            top_scissors += 1;
        } else if num_papers > 0 {
            matches.push('P');
            num_papers -= 1;
        }
    }

    println!("{}", matches);

    for _ in 0..2 {
        let mut round_below = String::new();

        for c in matches.chars() {
            match c {
                'R' => {
                    if num_papers > 0 {
                        round_below.push('R');
                        round_below.push('P');
                        num_papers -= 1;
                    } else {
                        if num_rocks == 0 {
                            panic!("we failed at rock boss...");
                        }
                        round_below.push('R');
                        round_below.push('R');
                        num_rocks -= 1;
                    }
                }
                'P' => {
                    if num_rocks > 0 {
                        round_below.push('P');
                        round_below.push('R');
                        num_rocks -= 1;
                    } else if num_papers > 0 {
                        round_below.push('P');
                        round_below.push('P');
                        num_papers -= 1;
                    } else if num_scissors > 0 {
                        round_below.push('P');
                        round_below.push('S');
                        num_scissors -= 1;
                    } else {
                        panic!("failed at paper");
                    }
                }
                'S' => {
                    if num_papers > 0 {
                        round_below.push('S');
                        round_below.push('P');
                        num_papers -= 1;
                    } else if num_scissors > 0 {
                        round_below.push('S');
                        round_below.push('S');
                        num_scissors -= 1;
                    } else if num_rocks > 0 && top_scissors > 1 {
                        round_below.push('S');
                        round_below.push('R');
                        num_rocks -= 1;
                        top_scissors -= 1;
                    } else {
                        panic!("failed at scissors");
                    }
                }
                _ => panic!("wrong char!!"),
            }
        }

        println!("{}", round_below);
        matches = round_below;
    }
}
