use bracket_lib::prelude::*;
mod constants;
mod game_mode;
mod player;
mod state;

use crate::state::*;

// TODO: page 63

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
