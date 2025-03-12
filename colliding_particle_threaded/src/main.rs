extern crate glium;
extern crate winit;
extern crate rand;
extern crate scoped_threadpool;

use rand::Rng;
use scoped_threadpool::Pool;
use std::time::Instant;

const ENCLOSURE_SIZE: f32 = 10.0; // Size of grid
const NUM_PARTICLES: usize = 100; // Number of particles
const NUM_MOVES: u32 = 1000000;  // Number of moves
const THREAD_COUNT: usize = 24; // Number of CPU threads

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    x: f32,
    y: f32,
}

impl Particle {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng(); // Random number generator
        Particle {
            x: rng.gen_range(0.0..ENCLOSURE_SIZE), // Random x between 0 and ENCLOSURE_SIZE.
            y: rng.gen_range(0.0..ENCLOSURE_SIZE), // Random y between 0 and ENCLOSURE_SIZE.
        }
    }

    pub fn move_randomly(&mut self) {
        // Randomly move the particle
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-1.0..=1.0); 
        let dy = rng.gen_range(-1.0..=1.0);

        // Clamp the new position
        self.x = (self.x + dx).clamp(0.0, ENCLOSURE_SIZE);
        self.y = (self.y + dy).clamp(0.0, ENCLOSURE_SIZE);
    }
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
    move_count: u32,
    total_moved_particles: usize,
}

impl ParticleSystem {
    pub fn new() -> Self {
        let particles = (0..NUM_PARTICLES).map(|_| Particle::new()).collect();
        ParticleSystem {
            particles,
            move_count: NUM_MOVES,
            total_moved_particles: NUM_PARTICLES,
        }
    }

    pub fn move_particles_loop(&mut self) {
        // This function handles the movement of particles over multiple threads
        let particles_per_thread = self.particles.len() / THREAD_COUNT; // Divide the particles among threads
        println!("Moving {} particles {} times across {} threads...", self.particles.len(), NUM_MOVES, THREAD_COUNT);

        let start_time = Instant::now(); // Start the timer to measure execution time

        let mut pool = Pool::new(THREAD_COUNT as u32); // Create a pool of threads for parallel execution
        pool.scoped(|scope| {
            for chunk in self.particles.chunks_mut(particles_per_thread) {
                // Each thread handles a chunk of particles.
                scope.execute(move || thread_main(chunk, NUM_MOVES));
            }
        });

        let elapsed_ms = Instant::now().duration_since(start_time).as_millis(); // Measure the elapsed time
        println!("\nElapsed time: {}ms", elapsed_ms);

        // Summaries of the moves made
        println!("Particles were moved {} times", self.move_count);
        println!("Total number of particles: {}", self.total_moved_particles);
    }
}

fn thread_main(chunk: &mut [Particle], iteration_count: u32) {
    for _ in 0..iteration_count {
        for particle in chunk.iter_mut() {
            particle.move_randomly(); // Move each particle randomly
        }
    }
}

fn main() {
    let mut system = ParticleSystem::new(); // Create a new particle system
    system.move_particles_loop(); // Run the loop to move the particles across threads
}