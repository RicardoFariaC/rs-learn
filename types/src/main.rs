fn main() {
    // Inteiros
    /*
        i8      | u8
        i16     | u16
        i32     | u32
        i64     | u64
        i128    | u128
        isize   | usize

        signed (-2^n-1 até 2^n-1 - 1)

        unsigned (0 até 2^n-1)

     */

    let _x = 5_u8; // i32 como default
    let _y = 123_456_789;
    let _hexa = 0xff;
    let _octal = 0o77;
    let _binario = 0b1111_0000;
    let _byte = b'A';

    // Floats
    /*
        f32 || f64        
     */

    let _fx = 5.0; // f64 como default
    let _fy: f32 = 5.0;

    // Bool
    /*
        true || false
     */

    let _bool = true;

    // Character

    let _character = '🦀'; // Utiliza até 4 bytes da tabela unicode.

    // Tuplas

    let _tupla = (1, '🦀', 4.5); // tamanho fixo, mesma estrutura.

    println!("({}, {}, {})", _tupla.0, _tupla.1, _tupla.2);

    // Arrays

    let _array = [1., 2., 3., 4.]; // unico tipo
    println!("[{}, {}, {}, {}]", _array[0], _array[1], _array[2], _array[3]);

}
