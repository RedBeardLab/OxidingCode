// // 1 / 6
// struct Particle(f32, f32);
// struct Velocity(f32, f32);
//
// fn update_state(p: Particle, v: Velocity) -> (Particle, Velocity) {
//     (Particle(p.0 + v.0, p.1 + v.1), v)
// }
//
// fn main() {
//     let mut p = Particle(3.0, 1.0);
//     let mut v = Velocity(2.0, 3.0);
//
//     // for _ in 0..10 {
//     //     let new_state = update_state(p, v);
//     //     p = new_state.0;
//     //     v = new_state.1;
//     // }
//
//     let new_state = update_state(p, v);
//     p = new_state.0;
//     v = new_state.1;
//     let new_state = update_state(p, v);
//     p = new_state.0;
//     v = new_state.1;
//
//     println!("Particle ({}, {})", p.0, p.1);
// }

// // 2 / 6
// // Impractical to return all arguments to the calee.
// // Introducing references
//
// struct Particle(f32, f32);
// struct Velocity(f32, f32);
//
// fn update_state(p: Particle, v: &Velocity) -> Particle {
//     Particle(p.0 + v.0, p.1 + v.1)
// }
//
// fn main() {
//     let p = Particle(3.0, 1.0);
//     let v = Velocity(2.0, 3.0);
//
//     let p01 = update_state(p, &v);
//     let p02 = update_state(p01, &v);
//
//     println!("Particle ({}, {})", p02.0, p02.1);
// }

// // 3/ 6
// // Introducing structs with references
//
// struct Particle(f32, f32);
// struct Velocity(f32, f32);
//
// struct Universe<'v> {
//     p1: Particle,
//     v1: &'v Velocity,
// }
//
// fn update_state(p: Particle, v: &Velocity) -> Particle {
//     Particle(p.0 + v.0, p.1 + v.1)
// }
//
// fn main() {
//     let p = Particle(3.0, 1.0);
//     let v = Velocity(2.0, 3.0);
//     let universe = Universe { p1: p, v1: &v };
//
//     let p01 = update_state(p, &v);
//     let p02 = update_state(p01, &v);
//
//     println!("Particle ({}, {})", p02.0, p02.1);
// }

// // 4 / 6
// // Make Universe hold a reference to Particle
//
// struct Particle(f32, f32);
// struct Velocity(f32, f32);
//
// struct Universe<'v> {
//     p1: &'v Particle,
//     v1: &'v Velocity,
// }
//
// fn update_state(p: &Particle, v: &Velocity) -> Particle {
//     Particle(p.0 + v.0, p.1 + v.1)
// }
//
// fn main() {
//     let p = Particle(3.0, 1.0);
//     let v = Velocity(2.0, 3.0);
//     let universe = Universe { p1: &p, v1: &v };
//
//     let p01 = update_state(&p, &v);
//     let p02 = update_state(&p01, &v);
//
//     println!("Particle ({}, {})", p02.0, p02.1);
//     println!("Particle ({}, {})", universe.p1.0, universe.p1.1);
// }

// // 5 / 6
// struct Particle(f32, f32);
// struct Velocity(f32, f32);
//
// struct Universe<'v> {
//     p1: Particle,
//     v1: &'v Velocity,
// }
//
// fn update_state(p: Particle, v: &Velocity) -> Particle {
//     Particle(p.0 + v.0, p.1 + v.1)
// }
//
// fn update_universe(uni: &mut Universe) {
//     uni.p1.0 += uni.v1.0;
//     uni.p1.1 += uni.v1.1;
// }
//
// fn main() {
//     let p = Particle(3.0, 1.0);
//     let v = Velocity(2.0, 3.0);
//     let mut universe = Universe { p1: p, v1: &v };
//
//     update_universe(&mut universe);
//     update_universe(&mut universe);
//
//     println!("Particle ({}, {})", universe.p1.0, universe.p1.1);
// }

// 6 / 6
struct Particle(f32, f32);
struct Velocity(f32, f32);

struct Universe<'v> {
    p1: &'v mut Particle,
    v1: &'v Velocity,
}

fn update_state(p: &mut Particle, v: &Velocity) {
    p.0 += v.0;
    p.1 += v.1;
}

fn update_universe(uni: &mut Universe) {
    uni.p1.0 += uni.v1.0;
    uni.p1.1 += uni.v1.1;
}

fn main() {
    let mut p = Particle(3.0, 1.0);
    let v = Velocity(2.0, 3.0);

    let mut universe = Universe { p1: &mut p, v1: &v };

    update_universe(&mut universe);
    update_universe(&mut universe);
    // update_state(&mut p, &v);

    println!("Particle ({}, {})", universe.p1.0, universe.p1.1);
}
//
//
