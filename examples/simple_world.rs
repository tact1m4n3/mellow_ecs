use mellow_ecs::world::World;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
enum ObjectColor {
    Red,
    Green,
}

#[derive(Debug)]
struct Object {
    color: ObjectColor,
}

struct Important;

fn main() {
    let mut world = World::default();

    let first = world.spawn((
        Position {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Object {
            color: ObjectColor::Red,
        },
    ));

    let second = world.spawn((
        Position {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Object {
            color: ObjectColor::Red,
        },
    ));

    let third = world.spawn((
        Position {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        Object {
            color: ObjectColor::Green,
        },
        Important,
    ));

    let forth = world.spawn((
        Position {
            x: 0.0,
            y: 0.0,
            z: 2.0,
        },
        Object {
            color: ObjectColor::Red,
        },
        Important,
    ));

    for (_id, pos) in world.query::<&Position>() {
        println!("position {:?}", pos)
    }

    for (_id, (obj, imp)) in world.query::<(&Object, Option<&Important>)>() {
        if imp.is_some() {
            println!("object with color {:?} is important", obj.color);
        } else {
            println!("object with color {:?} is not important", obj.color);
        }
    }

    for (_id, (obj, _imp)) in world.query::<(&mut Position, &Important)>() {
        obj.x += 2.0;
        println!("important object moved");
    }

    world.del(third);
    world.del(forth);

    let important_count = world.query::<&Important>().count();
    println!("{} important objects left", important_count);

    world.del(first);
    world.del(second);
}
