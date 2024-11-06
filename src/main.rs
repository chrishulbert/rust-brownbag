// Rust brownbag

// Install it!
// rustup == nvm
// Paste the command from here and follow defaults: https://www.rust-lang.org/learn/get-started
// I needed to retry, maybe zscaler related.
// Rust VSCode extension rust-analyzer

// Use it!
// cargo == yarn / npm
// cargo new rust-for-js-devs
// cd rust-for-js-devs
// cargo run

#[derive(Debug)] // Boilerplate to allow debug-printing.
enum Direction {
    North,
    South,
    East,
    West
}

// Immutable-by-default
// Instead of var vs let vs const, there's just 'let' which is immutable.
// If you want a mutable var, you can do 'let mut' which is like an extra step to
// encourage you that this isn't what you'd do most of the time. It's still fine though.

// Proper enums!

// Enum even with assoc values. Eg only get a certain value for a certain case.
enum Shape {
    Circle(f64), // Radius.
    Rectangle(f64, f64), // Width and height.
}
impl Shape { // Make functions that run on 'shape'.
    fn area(&self) -> f64 { // Area runs on 'self' and returns a f64.
        return match self { // Match is like 'switch'.
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
        };
    }
}

// Borrowing.
// & means 'i'm going to borrow this' as in read-only and I can't consume the memory.

// This is the usual way you'd take strings: borrowed (&str) in, owned (String) out.
fn do_something_to_a_string(foo: &str) -> String {
    return foo.to_owned() + " something"; // To_owned makes a copy that is owned by this func.
}

// Rust doesn't have classes/OOP/inheritance (it does have protocols though).
struct Car {
    make: String,
    model: String,
    reliability: i32,
}

// This function takes responsibility for owning the car, and the caller loses it.
fn sell_car_to_jim(car: Car) {
    println!("This car was sold to Jim, he owns it now: {} {}", car.make, car.model);
}

// Just like loaning your car to your kid, you expect to get it back in one piece.
fn loan_car_to_child(car: &Car) {
    println!("Your child takes this car for a joyride but it's still your car when it needs its tyres changed tomorrow: {}", car.model);
}

// To make this work, introduce how deps work.
// Add to cargo.toml:
// png = "0.17.14"
// All you have to do is add a line here, no need to eg yarn install,
// it just automatically figures it out next time you build or run or anything
fn do_slow_thing() {
    let width: u32 = 3840; // 4k resolution.
    let height: u32 = 2160;
    let mut data = Vec::<u8>::new(); // New empty array of rgba values (vector).
    for y in 0..height {
        for x in 0..width {
            let r = ((x as f64) / 1000. * 3. + (y as f64) / 1000. * 7.).sin();
            let g = ((x as f64) / 1000. * 5. + (y as f64) / 1000. * 5.).sin();
            let b = ((x as f64) / 1000. * 7. + (y as f64) / 1000. * 3.).sin();
            data.push((128. + r * 126.) as u8);
            data.push((128. + g * 126.) as u8);
            data.push((128. + b * 126.) as u8);
            data.push(255);
        }
    }

    let file = std::fs::File::create("Wavey.png").unwrap();
    let w = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
}

fn main() {
    let mut direction = Direction::North;
    direction = Direction::South; // Can't until let is let mut.
    println!("Which direction? {:#?}", direction);

    // This enum allows values stored in it with certain states, with the type system enforcing that if you're a certain
    // state, you definitely have that associated value.
    let circle = Shape::Circle(0.1);
    println!("Area: {} m^2", circle.area());

    let s = do_something_to_a_string("Blah");
    println!("String: {}", s);

    let my_old_bomb = Car{
        make: "Subaru".to_owned(),
        model: "Liberty".to_owned(),
        reliability: 100, // Smart enough to error if you forget a field.
    };
    let my_previous_car = Car{
        make: "BMW".to_owned(),
        model: "X5".to_owned(),
        reliability: -1000,
    };
    loan_car_to_child(&my_old_bomb); // & means 'loan it out' or 'get a borrowing reference'.
    sell_car_to_jim(my_previous_car); // No & means 'give it, its not mine any more'.
    println!("Cars i still own: {}", my_old_bomb.model);
    //println!("Cars i still own: {}", my_previous_car.model); // Rust won't let you drive your own car again after you sold it!

    // See how quick this is with: cargo run --release
    let start = std::time::Instant::now();
    do_slow_thing();
    println!("Slow thing took: {:.3}s", start.elapsed().as_secs_f64());
}
