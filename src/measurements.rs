use mouse_position::mouse_position::Mouse;

// Récupère la position actuelle de la souris
pub fn position() -> Mouse {
    Mouse::get_mouse_position()
}

// Calcule la distance entre deux points sur un plan cartésien
pub fn point_diff(point_a: &Mouse, point_b: &Mouse) -> f64 {
    match (point_a, point_b) {
        (Mouse::Position { x: x1, y: y1 }, Mouse::Position { x: x2, y: y2 }) => {
            let x_diff = (x2 - x1) as f64;
            let y_diff = (y2 - y1) as f64;
            (x_diff.powi(2) + y_diff.powi(2)).sqrt()
        }
        _ => 0.0, // Retourne 0 si l'une des positions est invalide
    }
}
