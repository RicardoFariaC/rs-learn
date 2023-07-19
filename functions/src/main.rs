fn say_hello(name: &str, color: &str) {
    println!("Hello, {name}. Your favorite color is: {color}");
}

fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    say_hello("Ricardo", "Orange");
    say_hello("Pedro", "Black");

    let _y: () = {
        say_hello("Ricardo", "Orange");
        let _x = 42;
    };

    let z = add_numbers(10, 5);

    println!("{z}");

    let input = "56 55 48 59 23 45 78 34";

    let result: Vec<i32> = input
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .map(|n: i32| n*2)
        .collect();
    println!("{:?}", result);

}
