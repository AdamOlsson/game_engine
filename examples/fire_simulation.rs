//extern crate game_engine;
//
//use crate::engine::{physics_engine::{collision::rigid_body::RigidBodyType, constraint::resolver::elastic::ElasticConstraintResolver, integrator::verlet::VerletIntegrator, narrowphase::{naive::Naive, NarrowPhase}}, renderer_engine::shapes::{circle::Circle, Shape}, util::log_performance::LogPerformance};
//
//use std::iter::zip;
//use cgmath::{MetricSpace, Vector3, Zero};
//use crate::engine::{Simulation, State};
//use crate::engine::renderer_engine::vertex::Vertex;
//use crate::engine::physics_engine::constraint::Constraint;
//use crate::engine::physics_engine::constraint::box_constraint::BoxConstraint;
//use crate::engine::physics_engine::collision::collision_handler::SimpleCollisionSolver;
//use crate::engine::physics_engine::collision::rigid_body::RigidBody;
//use crate::engine::physics_engine::broadphase::BroadPhase;
//use crate::engine::physics_engine::broadphase::blockmap::BlockMap;
//use crate::engine::init_utils::{create_grid_positions, generate_random_radii};
//use rayon::prelude::*;
//
//const CIRCLE_CONTACT_SURFACE_AREA: f32 = 0.0002;
//const BOTTOM_HEAT_SOURCE_TEMPERATURE: f32 = 3500.0;
//const BOTTOM_HEAT_BOUNDARY: f32 = -0.985;
//const HEAT_TRANSFER_COEFFICIENT: f32 = 0.028;
//const BASE_GRAVITY: f32 = 98000.0;
//
//const COMMON_RADIUS: f32 = 10.0;
//const VELOCITY_CAP: f32 = 400000.0;
//const NUM_COLS: u32 = 10;
//const NUM_ROWS: u32 = 10;
//const INITIAL_SPACING: f32 = 2.0;
//
//const COLOR_SPECTRUM_BUCKET_SIZE: f32 = 0.3;
//
//const BLACK: Vector3<f32> = Vector3::new(31.0, 17.0, 15.0);
//const RED: Vector3<f32> = Vector3::new(231.0, 24.0, 24.0); 
//const ORANGE: Vector3<f32> = Vector3::new(231.0, 110.0, 24.0);
//const YELLOW: Vector3<f32> = Vector3::new(249.0, 197.0, 26.0);
//const WHITE: Vector3<f32> = Vector3::new(254.0, 244.0, 210.0);
//
//pub struct FireState {
//    integrator: VerletIntegrator,
//    num_instances: u32,
//
//    temperatures: Vec<f32>,
//}
//
//impl FireState {
//    pub fn new(num_rows: u32, num_cols: u32, spacing: f32) -> Self {
//        let initial_spacing = (COMMON_RADIUS * 2.0) + spacing;
//        let initial_spacing_var = 0.001;
//        let target_num_instances: u32 = num_rows * num_cols;
//
//        let radii = generate_random_radii(target_num_instances, COMMON_RADIUS, 0.0);
//
//        let prev_positions = create_grid_positions(num_rows, num_cols, initial_spacing, Some(initial_spacing_var));
//        let positions = prev_positions.clone();
//        let acceleration = vec![Vector3::new(0.0, -150.0, 0.0); target_num_instances as usize];
//
//        let bodies: Vec<RigidBody> = zip(zip(prev_positions, acceleration), zip(positions, radii))
//            .enumerate()
//            .map(|(i, ((pp,a), (p, r)))| RigidBody::circle(i, Vector3::zero(), a, pp, p, r,Vector3::zero()))
//            .collect();
//
//        let temperatures = vec![0.0; target_num_instances as usize];
//        
//        let integrator = VerletIntegrator::new(VELOCITY_CAP, bodies);
//        
//        Self {
//            integrator,
//            num_instances: target_num_instances,
//            temperatures,
//        }
//    }
//}
//
//impl State for FireState {
//    fn get_bodies(&self) -> &Vec<RigidBody> {
//        self.integrator.get_bodies()
//    }
//    fn get_bodies_mut(&mut self) -> &mut Vec<RigidBody> {
//        self.integrator.get_bodies_mut()
//    }
//}
//
//pub struct FireSimulation {
//    // Simulation information
//    state: FireState,
//    dt: f32,
//    constraint: Box<dyn Constraint>,
//    broadphase: Box<dyn BroadPhase>,
//    narrowphase: Box<dyn NarrowPhase>,
//
//    // Performance information
//    performance: LogPerformance,
//
//    // Render information 
//    color_spectrum: ColorSpectrum,
//    pub colors: Vec<Vector3<f32>>,
//    pub indices: Vec<u16>,
//    pub vertices: Vec<Vertex>,
//    pub num_indices: u32,
//}
//
//#[allow(unreachable_code)]
//impl FireSimulation {
//    pub fn new(window_size: &winit::dpi::PhysicalSize<u32>) -> Self {
//        panic!("Fire simulation is deprecated and need rework");
//        let mut state = FireState::new(NUM_ROWS, NUM_COLS, INITIAL_SPACING);
//        let dt = 0.001;
//        let color_spectrum = ColorSpectrum::new(
//            vec![BLACK, RED, ORANGE, YELLOW, WHITE], COLOR_SPECTRUM_BUCKET_SIZE);
//
//        let mut colors = vec![color_spectrum.get(0); state.num_instances as usize];
//        let color_spectrum_len = color_spectrum.len();
//        for i in 0..state.num_instances as usize {
//            let index = (state.temperatures[i] as usize).min(color_spectrum_len - 1);
//            colors[i] = color_spectrum.get(index);
//        }
//
//        let bodies = state.get_bodies_mut();
//        colors.iter().enumerate().for_each(|(i,c)| bodies[i].color = *c);
//
//        let indices = Circle::compute_indices();
//        let num_indices = indices.len() as u32;
//        let vertices = Circle::compute_vertices();
//
//        let mut constraint = Box::new(BoxConstraint::new(ElasticConstraintResolver::new()));
//        constraint.set_top_left(Vector3::new(-(window_size.width as f32), window_size.height as f32, 0.0));
//        constraint.set_bottom_right(Vector3::new(window_size.width as f32, -(window_size.height as f32), 0.0));
//
//        let broadphase = Box::new(BlockMap::new(window_size.width as f32));
//        let collision_solver = SimpleCollisionSolver::new();
//        let narrowphase = Box::new(Naive::new(collision_solver));
//
//        let performance = LogPerformance::new();
//
//        Self {
//            performance,
//            state, constraint, broadphase, narrowphase, dt, color_spectrum, 
//            colors, indices, vertices, num_indices
//        }
//    }
//
//    #[allow(non_snake_case)]
//    fn heat_conduction(temp1: f32, temp2: f32, distance: f32) -> (f32, f32) {
//        let k = 1.0; // thermal conductivity
//        let A = CIRCLE_CONTACT_SURFACE_AREA;
//        let m1 = 1.0; // mass
//        let m2 = 1.0; // mass
//        let c1 = 1.0; // Heat capacity
//        let c2 = 1.0; // Heat capacity
//        let dT = (temp1 - temp2).abs();
//        let dT1dt = (k*A*dT)/(distance.sqrt()*m1*c1);
//        let dT2dt = (k*A*dT)/(distance.sqrt()*m2*c2);
//        if temp1 > temp2 {
//            return (-dT1dt, dT2dt);
//        } else {
//            return (dT1dt, -dT2dt);
//        }
//    }
//
//    fn heat_convection(object_temp: f32, fluid_temp: f32, object_radius: f32) -> f32 {
//        let surface_area = 2.0 * std::f32::consts::PI * object_radius; 
//        HEAT_TRANSFER_COEFFICIENT*surface_area*(fluid_temp - object_temp)
//    }
//
//
//    fn heat_transfer(
//        bodies: &Vec<RigidBody>, temperatures: &Vec<f32>,
//        broadphase: &Box<dyn BroadPhase>) -> Vec<f32> 
//    {
//        let candidates = broadphase.collision_detection(&bodies);
//        let mut thermal_delta = vec![0.0; bodies.len()];
//
//        let candidate_bodies: Vec<Vec<RigidBody>> = candidates.iter().map(| cs | {
//            cs.indices.iter().map(| idx | (bodies[*idx].clone())).collect()
//        }).collect();
//
//        let candidate_temperatures: Vec<Vec<f32>> = candidates.iter().map(| cs | {
//            cs.indices.iter().map(| idx | (temperatures[*idx])).collect()
//        }).collect();
//
//        // This is still the most expensive operation of heat transfer by far.
//        let thermal_deltas: Vec<Vec<f32>> = 
//            candidate_bodies.par_iter()
//            .zip(candidate_temperatures.par_iter())
//            .map( | (bs, ts )| -> Vec<f32> {
//                Self::local_heat_transfer(&bs, &ts)
//            })
//            .collect();
//        
//        thermal_deltas.iter()
//            .zip(candidate_bodies.iter())
//            .for_each(|(ts, bs)| {
//                bs.iter()
//                    .enumerate()
//                    .for_each(
//                        |(i, b)| thermal_delta[b.id] += ts[i]
//                    )
//            });
//
//        return thermal_delta;
//    }
//
//    /// Calculate the heat transfer due to convection between bodies
//    #[allow(non_snake_case)]
//    fn local_heat_transfer(
//        bodies: &Vec<RigidBody>, temperatures: &Vec<f32>) -> Vec<f32> 
//    {
//        let num_instances = bodies.len();
//        let mut thermal_delta = vec![0.0; num_instances ];
//
//        for i in 0..num_instances {
//            let radius = match bodies[i].body_type {
//                RigidBodyType::Circle { radius } => radius,
//                _ => panic!(),
//            };
//            // Bottom of the screen heats the objects
//            if bodies[i].position.y <= BOTTOM_HEAT_BOUNDARY {
//                let (temp_delta_i, _) = Self::heat_conduction(temperatures[i], BOTTOM_HEAT_SOURCE_TEMPERATURE, radius);
//                thermal_delta[i] += temp_delta_i; 
//            } else {
//                // Loose heat due to convection with air
//                thermal_delta[i] += Self::heat_convection(temperatures[i], 0.0, radius);
//            }
//
//            for j in (i+1)..num_instances {
//                // Heat conduction only happens between touching objects
//                let dist_sq = bodies[i].position.distance2(bodies[j].position);
//                
//                let (type_i, type_j) = (&bodies[i].body_type, &bodies[j].body_type);
//                match (type_i, type_j) {
//               
//                    (RigidBodyType::Circle { radius: ri }, RigidBodyType::Circle { radius: rj }) =>
//                        if dist_sq -0.01 <= (ri + rj).powi(2) {
//                            let (temp_delta_i, temp_delta_j) = Self::heat_conduction(temperatures[i], temperatures[j], ri + rj);
//                            thermal_delta[i] += temp_delta_i;
//                            thermal_delta[j] += temp_delta_j;
//                        },
//                    (_, _) => panic!(),
//                    }
//                }
//            }
//        return thermal_delta;
//    }
//}
//
//impl Simulation for FireSimulation {
//
//    fn update(&mut self) {
//        self.performance.log();
//        let num_instances = self.state.num_instances;
//        
//        // Update positions
//        self.state.integrator.update(self.dt);
//        let bodies = self.state.integrator.get_bodies_mut();
//
//        for _ in 0..8 {
//            // Constraint Application
//            for i in 0..num_instances as usize {
//                self.constraint.apply_constraint(&mut bodies[i]);
//            }
//    
//            // Broadphase
//            let candidates = self.broadphase.collision_detection(&bodies);
//            // Narrowphase
//            for c in candidates.iter() {
//                self.narrowphase.collision_detection(bodies, c);
//            }
//        }
//
//        // Heat transfer
//        let thermal_delta = Self::heat_transfer(&bodies, &self.state.temperatures, &self.broadphase);
//        
//        let color_spectrum_len = self.color_spectrum.len();
//        for (i, t) in thermal_delta.iter().enumerate() {
//            self.state.temperatures[i] += t; 
//            self.state.temperatures[i] = self.state.temperatures[i].max(0.0);
//            let index = (self.state.temperatures[i] as usize).min(color_spectrum_len - 1);
//            self.colors[i] = self.color_spectrum.get(index);
//            self.state.integrator.set_acceleration_y(i, -BASE_GRAVITY + (self.state.temperatures[i].powi(2)));
//        }
//    }
//
//    fn get_bodies(&self) -> &Vec<RigidBody> {
//        self.state.get_bodies()
//    }
//}
//
//struct ColorSpectrum {
//    spectrum: Vec<Vector3<f32>>,
//    bucket_size: f32,
//}
//impl ColorSpectrum {
//    pub fn new(key_colors: Vec<Vector3<f32>>, bucket_size: f32) -> Self {
//        let mut spectrum = vec![];
//        for i in 0..(key_colors.len() -1) {
//            let color1 = key_colors[i];
//            let color2 = key_colors[i+1];
//            let colors = Self::interpolate(color1, color2, 100);
//            spectrum.extend(colors);
//        }
//        Self {
//            spectrum,
//            bucket_size
//        }
//    }
//
//    pub fn get(&self, index: usize) -> Vector3<f32> {
//        let i = (index as f32 / self.bucket_size) as usize;
//        self.spectrum[i.min(self.spectrum.len()-1)]
//    }
//
//    pub fn len(&self) -> usize {
//        self.spectrum.len()
//    }
//    fn interpolate(color1: Vector3<f32>, color2: Vector3<f32>, num_steps: u32) -> Vec<Vector3<f32>> {
//        let mut colors = vec![];
//        for i in 0..num_steps {
//            let t = i as f32 / num_steps as f32;
//            let color = (color1 + (color2 - color1) * t)/255.0;
//            colors.push(color);
//        }
//        colors
//    }
//}
////use game_engine::examples::fire_simulation::FireSimulation;
////use winit::dpi::PhysicalSize;
#[allow(unreachable_code)]
fn main() {
    panic!("Fire simulation is deprecated and need rework.");
    //let window_size = PhysicalSize::new(800, 800);
    //let simulation = FireSimulation::new(&window_size);
    //pollster::block_on(run(simulation, window_size, 0));
}
