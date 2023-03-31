use crate::rps::RPSMove;

pub struct Fight {
    f1: RPSMove,
    f2: RPSMove,
}

impl Fight {
    pub fn new(f1: RPSMove, f2: RPSMove) -> Self {
        Self { f1, f2 }
    }

    pub fn fight(self) -> RPSMove {
        match self.f1.outcome_of_match(&self.f2) {
            crate::rps::MatchOutcome::Win(_) => self.f1,
            crate::rps::MatchOutcome::Draw(_) => self.f1,
            crate::rps::MatchOutcome::Loss(_) => self.f2,
        }
    }

    pub fn fight_char(&self) -> char {
        self.f1.outcome_of_match(&self.f2).get_char()
    }
}
