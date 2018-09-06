pub mod bowling {

    pub fn score_game(rolls: Vec<i32>) -> i32 {
        recursive_scorer(&rolls, 1)
    }

    fn recursive_scorer(rolls: &[i32], frame: i32) -> i32 {
        if frame > 10 { return 0 }
        let roll_one = rolls.get(0).unwrap_or(&0);
        let roll_two = rolls.get(1).unwrap_or(&0);
        let roll_three = rolls.get(2).unwrap_or(&0);

        if roll_one == &10 {
            return roll_one + roll_two + roll_three + recursive_scorer(&rolls[1..], frame + 1)
        }
        else if roll_one + roll_two == 10 {
            return roll_one + roll_two + roll_three + recursive_scorer(&rolls[2..], frame + 1)
        }
        else {
            roll_one + roll_two + recursive_scorer(&rolls[2..], frame + 1)
        }
    }

    // fn matching_scorer(rolls: &[i32], frame: i32) -> i32 {
    //     if frame > 10 { return 0 }

    //     // Needs nightly build
    //     match rolls {
    //         [10, bonus_one, bonus_two, rest..] => 10 + bonus_one + bonus_two + recursive_scorer(&rolls[1..], frame + 1),
    //         [roll_one, roll_two, bonus, rest..] if roll_one + roll_two == 10 => 10 + bonus + recursive_scorer(&rolls[2..], frame + 1),
    //         [roll_one, roll_two, rest..] => roll_one + roll_two + recursive_scorer(&rolls[2..], frame + 1),
    //         _ => 0
    //     }
    // }

    enum FrameState {
        First(i32),
        Second
    }

    fn iterative_scorer(rolls: Vec<i32>) -> i32 {
        let mut points = 0;
        let mut frame = 1;
        let mut state = FrameState::Second;
        let mut has_strike = false;
        let mut has_spare = false;
        for roll in rolls.iter() {
            if has_spare {
                points += roll;
                has_spare = false;
            }
            if has_strike {
                points += roll;
                has_spare = true;
                has_strike = false;
            }

            match state {
                FrameState::Second => {
                    if roll == &10 {
                        has_strike = true;
                        frame += 1;
                    }
                    else
                    {
                        state = FrameState::First(*roll);
                    }
                }
                FrameState::First(first_roll) => {
                    state = FrameState::Second;
                    if first_roll + roll == 10 {
                        has_spare = true;
                        frame += 1;
                    }
                }
            }

            if frame <= 10
            {
                points += roll;
            }
        }
        return points;
    }

}



#[cfg(test)]
mod tests {
    use std::iter;
    use bowling::score_game;

    #[test]
    fn all_zero_scores_zero() {
        let rolls = iter::repeat(0).take(20).collect();
        let score = score_game(rolls);
        assert_eq!(score, 0)
    }

    #[test]
    fn all_ones_scores_twenty() {
        let rolls = iter::repeat(1).take(20).collect();
        let score = score_game(rolls);
        assert_eq!(score, 20)
    }

    #[test]
    fn spares_score_next_roll_twice() {
        let mut rolls = vec!(0, 10);
        rolls.extend(iter::repeat(1).take(18));

        let score = score_game(rolls);
        assert_eq!(score, 29)
    }

    #[test]
    fn strikes_score_next_two_roll_twice() {
        let mut rolls = vec!(10);
        rolls.extend(iter::repeat(1).take(18));

        let score = score_game(rolls);
        assert_eq!(score, 30)
    }

    #[test]
    fn two_strikes_score_one_roll_thrice_and_two_rolls_twice() {
        let mut rolls = vec!(10, 10);
        rolls.extend(iter::repeat(1).take(16));

        let score = score_game(rolls);
        assert_eq!(score, 49)
    }

    #[test]
    fn all_strikes_scores_three_hundred() {
        let rolls = iter::repeat(10).take(12).collect();

        let score = score_game(rolls);
        assert_eq!(score, 300)
    }
}