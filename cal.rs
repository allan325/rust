use std::io;
pub fn pergunta()-> i32  {
    let mut entrada = String::new();
    println!("Digite um numero:");
    io::stdin().read_line(&mut entrada).unwrap();
    let mut num : i32 = entrada.trim().parse().unwrap();
    return num
}
