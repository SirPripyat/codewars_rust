mod cockroach_speed;
mod get_volume_of_cuboid;
mod paperwork;
mod update_light;

fn main() {
    let paperwork1 = paperwork::paperwork(0, 5);
    println!("Paperwork 1: {}", paperwork1);

    let cockroach_speed = cockroach_speed::cockroach_speed(1.08);
    println!("Cockroach speed: {}", cockroach_speed);

    let volume_of_cuboid = get_volume_of_cuboid::get_volume_of_cuboid(2.0, 3.0, 4.0);
    println!("Volume of cuboid: {}", volume_of_cuboid);

    let update_light = update_light::update_light("green");
    println!("Update light: {}", update_light);
}
