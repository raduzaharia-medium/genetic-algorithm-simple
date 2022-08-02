mod individual;
mod population;

use crate::population::Population;
use std::time::Instant;

fn main() {
    let mut population = Population::new_random(100, 100);
    println!("Initial fitness: {}", population.get_best_individual().fitness);

    let now = Instant::now();
    for _ in 0..500 {
        population = population.next_generation(0.8);
    }
    let elapsed = now.elapsed();

    println!("Evolved fitness: {} - elapsed: {:.2?}", population.get_best_individual().fitness, elapsed);
}
