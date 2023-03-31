mod rps;

fn main() {
    for i in 1..=5 {
        let data = std::fs::read_to_string(format!("./level2/level2_{i}.in")).unwrap();
        let mut lines = data.lines();
        let (n_str, m_str) = lines.next().unwrap().split_once(' ').unwrap();
        let n = n_str.parse::<usize>().unwrap();
        let m = m_str.parse::<usize>().unwrap();

        let out = lines
            .map(|line| match line {
                "RR" => "R",
                "RP" => "P",
                "RS" => "R",
                "SS" => "S",
                "SR" => "R",
                "SP" => "S",
                "PP" => "P",
                "PR" => "P",
                "PS" => "S",
                _ => "How did we get here",
            })
            .collect::<Vec<&str>>();

        std::fs::write(format!("./level1/level1_{i}.out"), out.join("\n")).unwrap();
    }
}
