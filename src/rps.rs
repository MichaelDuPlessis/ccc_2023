pub struct RPSMove(char);

enum MatchOutcome {
    Win,
    Draw,
    Loss,
}

impl RPSMove {
    pub fn new(rps_move: char) -> Self {
        Self(rps_move)
    }

    pub fn outcome_of_match(&self, other: RPSMove) -> MatchOutcome {
        match (self.0, other.0) {
            ('R', 'S') => MatchOutcome::Win,
            ('P', 'R') => MatchOutcome::Win,
            ('S', 'P') => MatchOutcome::Win,
            ('R', 'R') => MatchOutcome::Draw,
            ('S', 'S') => MatchOutcome::Draw,
            ('P', 'P') => MatchOutcome::Draw,
            _ => MatchOutcome::Loss,
        }
    }
}
