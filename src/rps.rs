#[derive(Clone, Copy)]
pub struct RPSMove(char);

#[derive(Clone, Copy, Debug)]
pub enum MatchOutcome {
    Win(char),
    Draw(char),
    Loss(char),
}

impl std::fmt::Display for RPSMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for MatchOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl MatchOutcome {
    pub fn get_char(&self) -> char {
        *match self {
            MatchOutcome::Win(c) => c,
            MatchOutcome::Draw(c) => c,
            MatchOutcome::Loss(c) => c,
        }
    }
}

impl RPSMove {
    pub fn new(rps_move: char) -> Self {
        Self(rps_move)
    }

    pub fn outcome_of_match(&self, other: &RPSMove) -> MatchOutcome {
        match (self.0, other.0) {
            ('R', 'S') => MatchOutcome::Win('R'),
            ('P', 'R') => MatchOutcome::Win('P'),
            ('S', 'P') => MatchOutcome::Win('S'),
            ('R', 'R') => MatchOutcome::Draw('R'),
            ('S', 'S') => MatchOutcome::Draw('S'),
            ('P', 'P') => MatchOutcome::Draw('P'),
            ('R', 'P') => MatchOutcome::Loss('P'),
            ('P', 'S') => MatchOutcome::Loss('S'),
            ('S', 'R') => MatchOutcome::Loss('R'),
            _ => MatchOutcome::Loss('X'),
        }
    }

    pub fn get_char(&self) -> char {
        self.0
    }
}

pub fn outcome_of_tournament_round(competitors: Vec<RPSMove>) {
    let mut winners = vec![];

    for i in (0..competitors.len()).into_iter().step_by(2) {
        if let MatchOutcome::Win(m) = competitors[i].outcome_of_match(&competitors[i + 1]) {
            winners.push(competitors[i]);
        } else {
            winners.push(competitors[i + 1]);
        }
    }
}

/*
    |=> S(S/P)
SP --
    |=> P(P/R)
*/
