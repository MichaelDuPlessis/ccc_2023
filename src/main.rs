mod fight;
mod rps;
mod tree_attempt;

use fight::Fight;
use rps::RPSMove;

fn main() {
    for i in 1..=5 {
        let data = std::fs::read_to_string(format!("./level2/level2_{i}.in")).unwrap();
        let mut lines = data.lines();
        let (n_str, m_str) = lines.next().unwrap().split_once(' ').unwrap();
        let n = n_str.parse::<usize>().unwrap();
        let m = m_str.parse::<usize>().unwrap();

        let fights = lines
            .map(|line| {
                let fighters = line.as_bytes().chunks(2);
                fighters
                    .map(|f| Fight::new(RPSMove::new(f[0] as char), RPSMove::new(f[1] as char)))
                    .collect::<Vec<Fight>>()
            })
            .collect::<Vec<_>>();

        // std::fs::write(format!("./level1/level1_{i}.out"), out.join("\n")).unwrap();
    }
}
