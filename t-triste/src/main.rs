extern crate t_triste_lib;
use t_triste_lib::*;

use bevy::prelude::*;

// Start function
fn main() {
    App::new()
        .add_plugins(GamePlugin)
        .run();
}
