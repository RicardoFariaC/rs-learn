
enum Animal {
    Cachorro,
    Gato,
    Calopsita,
    Pedro
}

fn main() {
    let pet: Animal = Animal::Pedro;
    match pet {
        Animal::Cachorro => println!("AuAu"),
        Animal::Calopsita => println!("Pru"),
        Animal::Gato => println!("Miau"),
        _ => println!("...")
    };
}
