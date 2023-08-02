use std::time::Instant;

use mellow_ecs::world::World;

fn main() {
    let mut world = World::default();

    let start_time = Instant::now();

    (0..1000000).for_each(|i| {
        world.spawn((i,));
    });

    let duration = Instant::now() - start_time;
    println!("checkpoint in {}", duration.as_secs_f64());

    let mut sum: i32 = 0;
    for (_, num) in world.query::<&i32>() {
        sum = sum.wrapping_add(*num);
    }

    let duration = Instant::now() - start_time;
    println!("finished in {} with sum {}", duration.as_secs_f64(), sum);
}
