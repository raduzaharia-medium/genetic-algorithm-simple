use crate::individual::Individual;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Population {
    members: Vec<Individual>,
}

impl Population {
    pub fn new_random(population_size: usize, individual_size: usize) -> Self {
        let mut result = vec![];

        for _ in 0..population_size {
            result.push(Individual::new_random(individual_size));
        }

        Population { members: result }
    }

    pub fn next_generation(&self, mutation_chance: f64) -> Self {
        let mut new_members = vec![];

        for i in 0..self.members.len() {
            let parent1 = &self.members[i];
            let parent2 = &self.select_random_individual();
            let child = parent1.cross(parent2);
            let mutant = child.mutate(mutation_chance);

            if parent1.fitness < parent2.fitness {
                if child.fitness < parent1.fitness {
                    if mutant.fitness < child.fitness {
                        new_members.push(mutant);
                    } else {
                        new_members.push(child);
                    }
                } else if mutant.fitness < parent1.fitness {
                    new_members.push(mutant);
                } else {
                    new_members.push(parent1.clone());
                }
            } else if child.fitness < parent2.fitness {
                if mutant.fitness < child.fitness {
                    new_members.push(mutant);
                } else {
                    new_members.push(child);
                }
            } else if mutant.fitness < parent2.fitness {
                new_members.push(mutant);
            } else {
                new_members.push(parent2.clone());
            }
        }

        Population { members: new_members }
    }

    pub fn get_best_individual(&self) -> Individual {
        let mut best = &self.members[0];

        for i in 0..self.members.len() {
            if self.members[i].fitness < best.fitness {
                best = &self.members[i];
            }
        }

        best.clone()
    }

    fn select_random_individual(&self) -> Individual {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.members.len());

        self.members[index].clone()
    }
}
