use rand::Rng;
use rand::thread_rng;


struct Line {
    slope: f32,
    intercept: f32
}

impl genetic::Individual for Line {

    /// Reproduction of a line will take the average of the slope and
    /// intercept
    fn reproduce(&self, other: &Self) -> Self {
        Line {
            slope: (self.slope + other.slope) / 2.0,
            intercept: (self.intercept + other.intercept) / 2.0
        }
    }

    /// Make random will generate a random slope and random y-intercept
    fn make_random() -> Self {
        Line {
            slope: thread_rng().gen::<f32>(),
            intercept: thread_rng().gen::<f32>()
        }
    }

    /// Mutation involves slight, but random changes to both the slope and
    /// intercept
    fn mutate(&mut self) {
        self.slope += thread_rng().gen_range(-10.0..10.0);
        self.intercept += thread_rng().gen_range(-10.0..10.0);
    }
}

fn main() {

    let content = std::fs::read_to_string("test.csv").expect("Failed to open file");
    let mut reader = csv::Reader::from_reader(content.as_bytes());

    let points: Vec<(f32, f32)> = reader.records().map(|record|
        (record.as_ref().unwrap()[0].parse::<f32>().unwrap(), record.unwrap()[1].parse::<f32>().unwrap())).collect();

    // Mean squared error based on the line compared against points
    let loss = |line: &Line| -> f32 {
        let mut total_error = 0.0;

        for point in &points {
            let predicted = point.0 * line.slope + line.intercept;
            total_error += f32::powf(predicted - point.1, 2.0);
        }
        return total_error / (points.len() as f32);
    };

    let result: Line = genetic::train(500, 500, 0.1, loss);

    println!("Slope: {}", result.slope);
    println!("Intercept: {}", result.intercept);

}
