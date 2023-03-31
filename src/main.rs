mod fight;
mod rps;
mod tournament;

use fight::Fight;
use rps::RPSMove;
use tournament::Tournament;

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
                println!("{:?}", splits);
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
            println!("{:?}", t);
        }
        // let mut tournaments = lines
        //     .map(|line| {
        //         let fighters = line.as_bytes().chunks(2);
        //         Tournament::new(
        //             fighters
        //                 .map(|f| Fight::new(RPSMove::new(f[0] as char), RPSMove::new(f[1] as char)))
        //                 .collect::<Vec<Fight>>(),
        //         )
        //     })
        //     .collect::<Vec<Tournament>>();

        // for t in &mut tournaments {
        //     t.torn(2);
        // }

        // let out = tournaments
        //     .into_iter()
        //     .map(|t| format!("{}", t))
        //     .collect::<Vec<String>>()
        //     .join("\n");

        // std::fs::write(format!("./level2/level2_{i}.out"), out).unwrap();
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
