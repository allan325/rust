use std::io;
mod calcular ;
fn main(){

       let num:i32 = calcular::pergunta(); // vai retornar o num do mod e colocar nessa variavel
        if num < 0 {
            println!("numero negativo")
        }else if num > 0 {
            println!("numero positivo");
        }
        else  {
            println!("igual a zero");
        }
}
