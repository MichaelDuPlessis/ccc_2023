use crate::fight::Fight;

pub struct Tournament {
    fights: Vec<Fight>,
}

impl std::fmt::Display for Tournament {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for f in &self.fights {
            out.push_str(&format!("{}", f))
        }
        write!(f, "{}", out)
    }
}

impl Tournament {
    pub fn new(fights: Vec<Fight>) -> Self {
        Self { fights }
    }

    pub fn torn(&mut self, num_rounds: usize) {
        for _ in 0..num_rounds {
            self.next_round()
        }
    }

    fn next_round(&mut self) {
        let mut temp = Vec::new();
        std::mem::swap(&mut temp, &mut self.fights);

        self.fights = temp
            .into_iter()
            .map(|f| f.fight())
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|f| Fight::new(f[0], f[1]))
            .collect()
    }
}
