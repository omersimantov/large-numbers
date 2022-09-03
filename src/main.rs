// Bernoulli's law of large numbers

fn main() {
    let mut sum = 0.0; // Sum of all generated values
    let mut avg; // Average generated value

    let mut n = 1;

    loop {
        // Generate a random 0-1 value
        sum += rand::random::<f32>();

        // avg approaches 0.5 as n approaches infinity
        avg = sum / (n as f32);

        // Persist the printing of avg to illustrate the effect
        for _ in 0..50000 {
            println!("{}", avg);
        }

        n += 1;
    }
}
