use std::io;

fn main() {
    let test = 'b';
    let txt: &str = "test"; //stack (tamanho fixo)
    
    println!("{test}\n{txt}");


    //heap (tamanho dinamico)
    let mut s = String::new();
    
    s.push('a');
    s.push_str("bcd");
    
    let s2: String = "Test".to_string();
    let s2: String = String::from("Test");
    let test2 = ['T','e','s','t'];
    let s2: String = String::from_iter(test2);
    let s2: String = "Test".into(); // tenta converter o valor para o tipo da variavel
    
    println!("{s}, {s2}");

    println!("{}", "-".repeat(40));

    //  '\' evita a quebra de linha
    let comment = "
        Digite um texto \
        A vai viarar b \
    ";
    println!("{comment}");


    let mut string = String::new();

    string.push('🙃');
    io::stdin().read_line(&mut string).expect("Erro ao ler o console");


    println!("=> {string}"); 
    println!("{}", string.trim().len()); //trim remove o \n do final //conta em bytes da tabla ASCII (🙃) => 4bytes
    println!("{}", string.trim().chars().count()); // conta os caracteres
    string = string.to_uppercase();
    string = string.replace("A", "b");
    println!("{string}");

    println!("{:-<40}", "sla");
    println!("{:-^40}", "sla");
    println!("{:->40}", "sla");

    println!("Coloque numeros separados por virgula");

    let mut numers = String::new();

    io::stdin().read_line(&mut numers).expect("Erro");


    let nums: Vec<i32> = numers.split(",").map( |c| c.trim().parse().expect("Erro, coloque numeros") ).collect();

    println!("{:?}", nums);

    //let n: i32 = nums[0].parse().expect("Erro, coloque numeros");
    //println!("{n}");

    let n:i32 = nums.iter().sum(); //a coleção só pode ser usada quando pego os valores diretamente, seja direta ou por referencia

    println!("{n}");

}