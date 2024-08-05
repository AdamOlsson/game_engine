
extern crate game_engine;

use game_engine::examples::debug_simulation::DebugSimulation;
use game_engine::engine::run::run;
use winit::dpi::PhysicalSize;

fn main() {
    let window_size = PhysicalSize::new(1000, 800);
    let simulation = DebugSimulation::new(&window_size);
    pollster::block_on(run(simulation, window_size, 0));
}
