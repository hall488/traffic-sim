mod app;
mod config;
mod simulation;
mod vehicle;
mod collision;
mod grid;
mod drawing_util;
mod stop_light;
mod intersection_manager;
mod qlearning;

use crate::app::App;
use winit::event_loop::{ControlFlow, EventLoop};
use std::time::{Duration, Instant};
use rayon::prelude::*;
use crate::simulation::Simulation;
use std::sync::{Arc, Mutex};

fn run_simulation(duration: Duration, counter: Arc<Mutex<usize>>) -> (Simulation, f64) {
    let mut simulation = Simulation::new(None);
    let start_time = Instant::now();
    let mut volume_sum = 0.0;
    let mut volume_count = 0;
    let frame_duration = Duration::from_secs_f64(1.0 / 60.0); // 60 FPS

    while start_time.elapsed() < duration {
        let frame_start_time = Instant::now();
        let dt = Duration::from_millis(10);
        simulation.update(dt);

        let total_volume: u32 = simulation.intersection_manager.intersection_volume.iter().sum();
        volume_sum += total_volume as f64;
        volume_count += 1;

        let elapsed = frame_start_time.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    let average_volume = if volume_count > 0 {
        volume_sum / volume_count as f64
    } else {
        0.0
    };

    let mut count = counter.lock().unwrap();
    *count += 1;

    println!("Simulations completed: {}, Average intersection volume: {:.2}", *count, average_volume);

    (simulation, average_volume)
}


fn run_simulation_batch(num_simulations: usize, duration: Duration, counter: Arc<Mutex<usize>>) -> Vec<(Simulation, f64)> {
    (0..num_simulations).into_par_iter()
        .map(|_| run_simulation(duration, Arc::clone(&counter)))
        .collect()
}

fn run_multiple_simulations(num_simulations: usize, batch_size: usize, duration: Duration) -> Vec<(Simulation, f64)> {
    let counter = Arc::new(Mutex::new(0));
    let mut results = Vec::new();

    for _ in (0..num_simulations).step_by(batch_size) {
        let batch_results = run_simulation_batch(batch_size, duration, Arc::clone(&counter));
        results.extend(batch_results);
    }

    results
}

fn main() {
    let num_simulations = 500;
    let simulation_duration = Duration::from_secs(6);
    let batch_size = 16; // Adjust this according to the number of cores

    // Run multiple simulations in batches without drawing
    let simulation_results = run_multiple_simulations(num_simulations, batch_size, simulation_duration);

    // Extract simulations and their average volumes
    let simulations: Vec<Simulation> = simulation_results.into_iter().map(|(sim, _)| sim).collect();

    // Optionally, aggregate the models or select the best model
    let best_simulation = simulations.into_iter().max_by(|a, b| {
        a.qlearning.q_table.sum().partial_cmp(&b.qlearning.q_table.sum()).unwrap()
    }).unwrap();

    // Save the best model (pseudo-code, implement actual saving logic)
    // save_model(&best_simulation.q_learning);

    // Initialize the window and visualize using the best model
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();

    // Load the best model into the app's simulation
    if let Some(simulation) = app.simulation.as_mut() {
        simulation.qlearning = best_simulation.qlearning;
    }

    let _ = event_loop.run_app(&mut app);
}
