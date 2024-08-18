use bracket_lib::prelude::{main_loop, BError, BTermBuilder};
mod constants;
mod game_mode;
mod obstacle;
mod player;
mod state;

use crate::state::State;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
