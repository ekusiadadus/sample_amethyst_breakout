use amethyst::{
    core::ecs::{System, WriteStorage, ReadExpect},
    input::{InputHandler, StringBindings, VirtualKeyCode},
};

use crate::component::bar::*;
use crate::resource::score::Score;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        WriteStorage<'a, Bar>,
        ReadExpect<'a, InputHandler<StringBindings>>,
        ReadExpect<'a, Score>,
    );

    fn run(&mut self, (mut bars, input, score): Self::SystemData) {
        if score.is_game == false {
            return
        }
        
        for bar in bars.as_mut_slice() {
            if input.key_is_down(VirtualKeyCode::Right) {
                bar.set_speed(MoveDirection::Right);
            }
            else if input.key_is_down(VirtualKeyCode::Left) {
                bar.set_speed(MoveDirection::Left)
            }
            else {
                bar.set_speed(MoveDirection::None);
            }
        }
    }
}
