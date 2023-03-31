mod fight;
mod rps;
mod tournament;

use fight::Fight;
use rps::RPSMove;
use tournament::Tournament;

fn main() {
    for i in 1..=5 {
        let data = std::fs::read_to_string(format!("./level2/level2_{i}.in")).unwrap();
        // let data = std::fs::read_to_string(format!("./level2/level2_example.in")).unwrap();
        let mut lines = data.lines();
        let (n_str, m_str) = lines.next().unwrap().split_once(' ').unwrap();
        let n = n_str.parse::<usize>().unwrap();
        let m = m_str.parse::<usize>().unwrap();

        let mut tournaments = lines
            .map(|line| {
                let fighters = line.as_bytes().chunks(2);
                Tournament::new(
                    fighters
                        .map(|f| Fight::new(RPSMove::new(f[0] as char), RPSMove::new(f[1] as char)))
                        .collect::<Vec<Fight>>(),
                )
            })
            .collect::<Vec<Tournament>>();

        for t in &mut tournaments {
            t.torn(2);
        }

        let out = tournaments
            .into_iter()
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>()
            .join("\n");

        std::fs::write(format!("./level2/level2_{i}.out"), out).unwrap();
    }
}
