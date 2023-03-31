mod fight;
mod rps;
mod tournament;
mod tree_attempt;

use fight::Fight;
use rps::RPSMove;
use tournament::Tournament;
use tree_attempt::try_solve;

fn main() {
    for i in 1..=5 {
        let data = std::fs::read_to_string(format!("./level3/level3_{i}.in")).unwrap();
        // let data = std::fs::read_to_string(format!("./level2/level2_example.in")).unwrap();
        let mut lines = data.lines();
        let (n_str, m_str) = lines.next().unwrap().split_once(' ').unwrap();
        let num_lines = n_str.parse::<usize>().unwrap();
        let num_fighters = m_str.parse::<usize>().unwrap();

        let tournaments = lines
            .map(|line| {
                let splits = line.split(" ").collect::<Vec<&str>>();
                let r = (splits[0][0..splits[0].len() - 1])
                    .parse::<usize>()
                    .unwrap();
                let p = (splits[1][0..splits[1].len() - 1])
                    .parse::<usize>()
                    .unwrap();
                let s = (splits[2][0..splits[2].len() - 1])
                    .parse::<usize>()
                    .unwrap();
                (r, p, s)
            })
            .collect::<Vec<_>>();

        for t in tournaments {
            try_solve(t);
        }
    }
}

fn get_win(style: char) -> char {
    match style {
        'R' => 'P',
        'P' => 'S',
        'S' => 'R',
        _ => panic!(),
    }
}
