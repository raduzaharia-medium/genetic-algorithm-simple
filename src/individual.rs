use rand::thread_rng;
use rand::Rng;
use rand_distr::Uniform;

#[derive(Clone, Debug)]
pub struct Individual {
    genes: Vec<f64>,
    pub fitness: f64,
}

impl Individual {
    pub fn new_random(size: usize) -> Self {
        let mut result = vec![];
        let mut fitness: f64 = 0.0;

        for _ in 0..size {
            let new_gene = create_random_gene();

            result.push(new_gene);
            fitness += new_gene;    // we try to get the vector with all values zero, so anything we add to it adds to the fitness
        }

        Individual {
            genes: result,
            fitness: fitness.abs(),
        }
    }

    pub fn mutate(&self, chance: f64) -> Self {
        if !check_chance(chance) {
            return self.clone();
        }

        let mut rng = rand::thread_rng();
        let mut result = self.genes.to_vec();

        let index = rng.gen_range(0..self.genes.len());
        let new_gene = create_random_gene();
        let new_fitness = self.fitness + new_gene - result[index];

        result[index] = new_gene;

        Individual {
            genes: result.to_vec(),
            fitness: new_fitness.abs(),
        }
    }

    pub fn cross(&self, another: &Self) -> Self {
        let cut_index = self.genes.len() / 2;
        let result: Vec<f64> = self.genes[..cut_index]
            .iter()
            .cloned()
            .chain(another.genes[cut_index..].iter().cloned())
            .collect();
        let new_fitness: f64 = result.iter().sum();

        Individual {
            genes: result,
            fitness: new_fitness.abs(),
        }
    }
}

fn create_random_gene() -> f64 {
    thread_rng().sample(Uniform::new(-1.0, 7.0))
}

fn check_chance(chance: f64) -> bool {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen::<f64>();

    random_number < chance
}
