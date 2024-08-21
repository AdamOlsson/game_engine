extern crate game_engine;

use game_engine::examples::fire_simulation::FireSimulation;
use game_engine::engine::run::run;
use winit::dpi::PhysicalSize;
#[allow(unreachable_code)]
fn main() {
    panic!("Fire simulation is deprecated and need rework.");
    let window_size = PhysicalSize::new(800, 800);
    let simulation = FireSimulation::new(&window_size);
    pollster::block_on(run(simulation, window_size, 0));
}
