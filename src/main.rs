extern crate ggez;

use ggez::*;
use std::time::Duration;

struct MainState {}

impl MainState {
    fn new() -> GameResult<Self> {
        let s = MainState {};
        Ok(s)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("ggez-skeleton", "jamiltron", c).unwrap();
    let state = &mut MainState::new().unwrap();
    event::run(ctx, state).unwrap();
}
