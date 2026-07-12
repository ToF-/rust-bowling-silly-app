pub struct Game {
    score: usize,
}

impl Game {
    pub fn new() -> Self {
        Game { score: 0 }
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn add_roll(&mut self, roll: usize) -> () {
        self.score += roll
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use speculoos::assert_that;

    #[test]
    fn init_game_puts_score_to_zero() {
        let game = Game::new();
        speculoos::assert_that!(game.score()).is_equal_to(0);
    }
    #[test]
    fn add_average_roll_scores_average() {
        let mut game = Game::new();
        game.add_roll(4);
        game.add_roll(3);
        speculoos::assert_that!(game.score()).is_equal_to(7);
    }
}
