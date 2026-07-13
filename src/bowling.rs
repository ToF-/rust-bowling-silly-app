pub struct Game {
    score: usize,
    bonus: usize,
    next_bonus: usize,
    last_roll: Option<usize>,
    frames: usize,
}

impl Game {
    pub fn new() -> Self {
        Game { score: 0, bonus: 0, next_bonus: 0, last_roll: None, frames: 0, }
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn add_roll(&mut self, roll: usize) {
        self.score += roll * self.bonus;
        self.bonus = self.next_bonus;
        self.next_bonus = 0;
        if self.frames < 10 {
            match self.last_roll {
                None => {
                    if roll == 10 {
                        self.bonus += 1;
                        self.next_bonus = 1;
                        self.last_roll = None;
                        self.frames += 1;
                    } else {
                        self.last_roll = Some(roll);
                    }
                }
                Some(last) => {
                    if last + roll == 10 {
                        self.bonus = 1
                    };
                    self.last_roll = None;
                    self.frames += 1;
                }
            }
            self.score += roll
        }
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
    #[test]
    fn add_a_spare_scores_a_bonus() {
        let mut game = Game::new();
        game.add_roll(5);
        game.add_roll(5);
        game.add_roll(4);
        speculoos::assert_that!(game.score()).is_equal_to(18);
    }
    #[test]
    fn add_a_strike_scores_two_boni() {
        let mut game = Game::new();
        game.add_roll(10);
        game.add_roll(5);
        game.add_roll(4);
        speculoos::assert_that!(game.score()).is_equal_to(28);
    }
    #[test]
    fn twelve_strikes_scores_a_perfect() {
        let mut game = Game::new();
        for i in 0..12 {
            game.add_roll(10)
        };
        speculoos::assert_that!(game.score()).is_equal_to(300);
    }
}
