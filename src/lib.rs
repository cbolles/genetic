

/// An individual represents a single member of a population in the geneic
/// algorithm. The individual represents a single possible solution to the
/// given task.
pub trait Individual {
    /// Generate a new individual based on the features of this individual
    /// and the provided individual.
    ///
    /// # Arguments
    /// * `other` - The other individual to use to produce a new individual
    fn reproduce(&self, other: &Self) -> Self;

    /// Create a "random" individual whose features are generated in some
    /// "random" fashion.
    fn make_random() -> Self;

    /// Mutates some aspects of the indivdual's features.
    fn mutate(&mut self);
}


/// Runs through the genetic algorithm. Will do the following.
///
/// 1. Generate a population of size `num_individuals` using the
///    `Individual::make_random` method
/// 2. Run the following until either `max_iterations` is hit or an
///    `Individual` with a `target_fitness_loss` is found
///    * Determine the fitness of each individual in the population
///    * Sort by the individuals with the lowest fitness loss
///    * Have the most fit individuals reproduce
///    * Implement some amount of mutation
///
/// # Arguments
/// * `num_individuals`: The number of individuals that are in the population
/// * `max_iterations`: The maximum number of populations the algorithm will go through
/// * `target_fitness_loss`: Target loss value of an indeal individual
/// * `loss`: The loss function that can be applied to an indiviual
pub fn train<T: Individual, F>(num_individuals: usize, max_iterations: usize, target_fitness_loss: f32, loss: F) -> T
    where F: Fn(&T) -> f32,
          T: Iterator {
    let mut population: Vec<(T, f32)> = Vec::with_capacity(num_individuals);

    // Initialize population
    for _i in 0..num_individuals {
        population.push((T::make_random(), 0.0));
    }

    // Represent the index limit for the top 20% of the population
    let max_index: usize = (0.2 * (num_individuals as f32)) as usize;

    // Training loop
    for _i in 0..max_iterations {
        // Apply loss function to each individual
        population.iter_mut().for_each(|pair| pair.1 = loss(&pair.0));

        // Sort by loss
        population.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Check to see if we have found a solution
        if population[0].1 <= target_fitness_loss {
            break;
        }

        // Handle reproduction, for now, only the top 20% can reproduce


    }

    // Assume that the first element is the best
    // HACK to avoid having T require a Copy implementation
    return std::mem::replace(&mut population[0].0, T::make_random());
}
