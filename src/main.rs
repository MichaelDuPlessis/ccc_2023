mod fight;
mod rps;

use rps::RPSMove;

fn main() {
    for i in 1..=5 {
        let data = std::fs::read_to_string(format!("./level1/level1_{i}.in")).unwrap();
        let mut lines = data.lines();
        let num = lines.next().unwrap().parse::<usize>().unwrap();

        let fights = lines
            .map(|line| {
                let fighters = line.as_bytes().chunks(2);
                fighters
                    .map(|f| vec![RPSMove::new(f[0] as char), RPSMove::new(f[1] as char)])
                    .flatten()
                    .collect::<Vec<RPSMove>>()
            })
            .collect::<Vec<Vec<RPSMove>>>();

        // std::fs::write(format!("./level1/level1_{i}.out"), out.join("\n")).unwrap();
    }
}
