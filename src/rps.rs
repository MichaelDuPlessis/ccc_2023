pub struct RPSMove(char);

pub enum MatchOutcome {
    Win(char),
    Draw(char),
    Loss(char),
}

impl RPSMove {
    pub fn new(rps_move: char) -> Self {
        Self(rps_move)
    }

    pub fn outcome_of_match(&self, other: RPSMove) -> MatchOutcome {
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
}
