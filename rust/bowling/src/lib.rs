use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

// I need to know which frame is in turn, because every 10 frame is a special case
// I need to know state of a game
// Strate of a frame is undeterministic, because it depends on scores of individual rolls.

pub struct BowlingGame {
    frames: Vec<Frame>,
    active_frame: Option<Frame>,
    // score: Option<u16>,
    // bowl: f32,
    // last_pins: u16,
    // is_strike: bool,
    // is_spare: bool,
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self {
            frames: std::vec::Vec::with_capacity(10),
            active_frame: Some(Frame::default()),
            // score: None,
            // bowl: 0f32,
            // last_pins: 16,
            // is_strike: false,
            // is_spare: false,
        }
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame::default()
    }

    // since I do not understand requirement clear and loud. It's a game of
    // adjusting this method to tests. It means a lot of rewritng for condidition
    // Docs for what is done. State of the game is holding vars to see if game
    // is strike or spare. And updates accordingli.
    // ten_frames_without_a_strike_or_spare() test is not passig apparently I need to assign
    // last_pins to zero again with some condition.

    // roll must call referenced active frame
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // if Some(activeframe) call active frame roll_frame
        // if not create defaut adjust requred fields in self and call
        // roll_frame of new active.
        // dbg!(self.bowl);
        if pins.gt(&10) {
            return Err(Error::NotEnoughPinsLeft);
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // if_the_last_frame_is_a_strike_you_cannot_score_before_the_extra_rolls_are_taken
        // if_the_last_frame_is_a_spare_you_cannot_create_a_score_before_extra_roll_is_taken
        // special case: last_two_strikes_followed_by_only_last_bonus_with_non_strike_points
        // a_game_score_is_some_if_ten_frames_have_been_rolled()
        // you_cannot_score_a_game_with_no_rolls
        // a_game_score_is_none_if_fewer_than_ten_frames_have_been_rolled
        // twenty_zero_pin_rolls_scores_zero
        // ten_frames_without_a_strike_or_spare
        // points_scored_in_the_roll_after_a_spare_are_counted_twice_as_a_bonus
        // consecutive_spares_each_get_a_one_roll_bonus
        //

        if self.frames.len().eq(&0) {
            return None;
        }
        Some(0)

        // Must redo this lines below.
    }
}

// how to advance frames in game? must habe bool flag for frame active or something.
struct Frame {
    score: u16,
    rolls_left: u8,
    special: bool,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            score: 0,
            rolls_left: 2,
            special: false,
        }
    }
}

impl Add for Frame {
    type Output = u16;

    fn add(self, other: Self) -> u16 {
        self.score + other.score
    }
}
/*
### Structure of a Bowling Game

A standard game of bowling consists of 10 frames. In each frame, the player has up to two rolls to knock down ten pins. The exception is the 10th frame, where the player can roll up to three times if they roll a strike or spare on their first two rolls.

### Scoring in Bowling

Scoring in bowling can seem a bit complex at first, but it's straightforward once you understand the basics. Here are the key concepts:

1. **Open Frame**: If a player doesn't knock down all ten pins in two rolls, the score for that frame is simply the number of pins knocked down.

   **Example**: If a player knocks down 3 pins on the first roll and 4 pins on the second roll, the score for that frame is 3 + 4 = 7.

2. **Spare**: If a player knocks down all ten pins in two rolls, they get a spare. The score for that frame is 10 plus the number of pins knocked down on the next roll.

   **Example**: If a player rolls a spare and then knocks down 5 pins on the next roll, the score for the frame with the spare is 10 + 5 = 15.

3. **Strike**: If a player knocks down all ten pins on the first roll, they get a strike. The score for that frame is 10 plus the number of pins knocked down on the next two rolls.

   **Example**: If a player rolls a strike and then knocks down 3 pins and 6 pins on the next two rolls, the score for the frame with the strike is 10 + 3 + 6 = 19.

### 10th Frame Special Case

In the 10th frame, if a player rolls a strike or a spare, they get additional rolls:

- **If a player rolls a strike**: They get two more rolls.
- **If a player rolls a spare**: They get one more roll.

The total score for the 10th frame will include these extra rolls.

### Example Calculation

Let’s walk through an example game frame by frame:

1. **Frame 1**: The player rolls a strike (X).
2. **Frame 2**: The player rolls 3 and then 6.

   **Score for Frame 1**: 10 + 3 + 6 = 19

   **Score for Frame 2**: 3 + 6 = 9

   **Total Score after Frame 2**: 19 + 9 = 28

3. **Frame 3**: The player rolls a spare (5 and 5).
4. **Frame 4**: The player rolls 4 and then 2.

   **Score for Frame 3**: 10 + 4 = 14

   **Score for Frame 4**: 4 + 2 = 6

   **Total Score after Frame 4**: 28 + 14 + 6 = 48

5. **Frame 5**: The player rolls 2 and then 7.

   **Score for Frame 5**: 2 + 7 = 9

   **Total Score after Frame 5**: 48 + 9 = 57

6. **Frame 6**: The player rolls a strike (X).
7. **Frame 7**: The player rolls a strike (X).
8. **Frame 8**: The player rolls 8 and then 1.

   **Score for Frame 6**: 10 + 10 + 8 = 28

   **Score for Frame 7**: 10 + 8 + 1 = 19

   **Score for Frame 8**: 8 + 1 = 9

   **Total Score after Frame 8**: 57 + 28 + 19 + 9 = 113

9. **Frame 9**: The player rolls 9 and then 0.

   **Score for Frame 9**: 9 + 0 = 9

   **Total Score after Frame 9**: 113 + 9 = 122

10. **Frame 10**: The player rolls 7 and then a spare (3). They get one extra roll and knock down 6 pins.

    **Score for Frame 10**: 10 + 6 = 16

    **Total Score for the game**: 122 + 16 = 138

### Final Note

Understanding the basics of how strikes and spares add points based on subsequent rolls is key to grasping bowling scoring. It’s often helpful to keep track frame by frame, especially when additional rolls from strikes and spares come into play.

If you have any further questions or need clarification on a specific aspect, feel free to ask!
*/
