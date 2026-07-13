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

    pub fn spare(&mut self) -> bool {
        match self.last_roll {
            None => false,
            Some(last) => {
                self.add_roll(10 - last);
                true
            }
        }
    }

    pub fn strike(&mut self) -> bool {
        match self.last_roll {
            None => {
                self.add_roll(10);
                true
            }
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use speculoos::assert_that;
    use speculoos::boolean::BooleanAssertions;


    #[test]
    fn init_game_puts_score_to_zero() {
        let game = Game::new();
        speculoos::assert_that(&game.score()).is_equal_to(0);
    }
    #[test]
    fn add_average_roll_scores_average() {
        let mut game = Game::new();
        game.add_roll(4);
        game.add_roll(3);
        speculoos::assert_that(&game.score()).is_equal_to(7);
    }
    #[test]
    fn add_a_spare_scores_a_bonus() {
        let mut game = Game::new();
        game.add_roll(5);
        game.spare();
        game.add_roll(4);
        speculoos::assert_that(&game.score()).is_equal_to(18);
    }
    #[test]
    fn add_a_strike_scores_two_boni() {
        let mut game = Game::new();
        game.strike();
        game.add_roll(5);
        game.add_roll(4);
        speculoos::assert_that(&game.score()).is_equal_to(28);
    }
    #[test]
    fn twelve_strikes_scores_a_perfect() {
        let mut game = Game::new();
        for _ in 0..12 {
            game.strike();
        };
        speculoos::assert_that(&game.score()).is_equal_to(300);
    }
    #[test]
    fn complete_a_spare_change_the_score() {
        let mut game = Game::new();
        game.add_roll(4);
        speculoos::assert_that(&game.spare());
        game.add_roll(5);
        speculoos::assert_that(&game.score()).is_equal_to(20);
    }
    #[test]
    fn complete_a_spare_cant_change_the_score_on_new_frame() {
        let mut game = Game::new();
        game.add_roll(4);
        game.add_roll(4);
        speculoos::assert_that(&game.spare()).is_false();
        speculoos::assert_that(&game.score()).is_equal_to(8);
    }
    #[test]
    fn strike_change_the_score() {
        let mut game = Game::new();
        speculoos::assert_that(&game.strike());
        game.add_roll(4);
        game.add_roll(3);
        speculoos::assert_that(&game.score()).is_equal_to(24);
    }
    #[test]
    fn strike_cant_change_the_score_on_half_frame() {
        let mut game = Game::new();
        game.add_roll(4);
        speculoos::assert_that(&game.strike()).is_false();
    }
}
