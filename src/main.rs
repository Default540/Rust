fn main() {
    //arquitetura snake_case

    //inteiros, flutuante, booleano e caracter
    //tupla (1,2,4,true, 'a') Matriz (array): [1,2,3,4,5]
    //inteiros i32 permite numeros negativos (signed), u32 (unsigned) não permite

    //signed: -(2^(n-1)) -> 2^(n-1) - 1
    //unsigned: 0 -> 2^n-1

    let x: u8 = 7;
    let y = 6_u8;

    println!("{}", (x + y));

    let dez_bilhoes: u64 = 10_000_000_000; //visual

    println!("{}", dez_bilhoes);

    let h = 0xff; //hex
    let o = 0o77; //oct
    let b = 0b11_10; //bin
    let by = b'A';

    println!("Hex: {}\nOct: {}\nBin: {}\nBy: {}", h, o, b, by);

    let mut tupla = (1, 2, 3.5);

    println!("{:?}", tupla); //print da tupla inteira precisa da formatação {:?}
    println!("{}", tupla.0);

    tupla.0 = 3;
    tupla.2 = 3.7;

    println!("{:?}", tupla);

    let array: [i32; 3];
    let array: [i32; 5] = [5, 6, 7, 2, 3];

    println!("{:?}", array);
    println!("{:?}", &array[0..2]);
    println!("{:?}", &array[2..]);
}
