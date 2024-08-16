mod measurements;
mod storage;

use std::thread;
use std::time::Duration;

fn main() {
    // Charger la distance totale précédemment enregistrée
    let mut total_distance = storage::load_distance();

    // Initialiser la position précédente de la souris
    let mut previous_position = measurements::position();

    loop {
        thread::sleep(Duration::from_millis(20));
        let current_position = measurements::position();
        let distance = measurements::point_diff(&previous_position, &current_position);
        total_distance += distance;

        if total_distance >= 3779527.5591 {
            println!(
                "Distance parcourue: {:.2} pixels (Total: {:.2} kilometers)",
                distance,
                total_distance * 0.0000002645833
            );
        } else {
            println!(
                "Distance parcourue: {:.2} pixels (Total: {:.2} meters)",
                distance,
                total_distance * 0.0002645833
            );
        }

        previous_position = current_position;
        storage::save_distance(total_distance);
    }
}
