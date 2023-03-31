use crate::rps::RPSMove;

pub struct Fight {
    f1: RPSMove,
    f2: RPSMove,
}

impl Fight {
    pub fn new(f1: RPSMove, f2: RPSMove) -> Self {
        Self { f1, f2 }
    }
}
