fn to_uppercase(text: &mut String) {
    *text = text.to_uppercase()
}

fn add_prefix(text: &mut String) {
    *text = format!("FOO_{text}")
}

fn main() {
    let a = String::from("Ricardo");
    let b = &a;

    println!("{}", a);
    println!("{}", b);

    let mut name = "ricardo".to_string();
    to_uppercase(&mut name);
    add_prefix(&mut name);

    println!("{name}");

}

/*
    1. Cada valor tem um dono;
    2. Só pode ter um único dono;
    3. Quando o dono sai de escopo, o valor é limpo;
    4. A posse só pode ser movida para outro dono;

*/