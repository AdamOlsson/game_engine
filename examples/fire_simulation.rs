extern crate game_engine;

use game_engine::examples::fire_simulation::FireSimulation;
use game_engine::engine::run::run;
use winit::dpi::PhysicalSize;

fn main() {
    println!("Fire simulation is not working as intended.");
    let window_size = PhysicalSize::new(800, 800);
    let simulation = FireSimulation::new(&window_size);
    pollster::block_on(run(simulation, window_size));
}
