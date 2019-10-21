struct Particle(f32, f32);
struct Velocity(f32, f32);

fn update_state(p: Particle, v: Velocity) -> (Particle, Velocity) {
    (Particle(p.0 + v.0, p.1 + v.1), v)
}

fn main() {
    let mut p = Particle(3.0, 1.0);
    let mut v = Velocity(2.0, 3.0);

    for _ in 0..10 {
        let new_state = update_state(p, v);
        p = new_state.0;
        v = new_state.1;
    }

    println!("Particle ({}, {})", p.0, p.1);
}
