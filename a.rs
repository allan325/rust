fn main() {
    println!("lista de contato");// sempre tem que ter ; no final
    let nome :&str = "allan";  // str são para marca letras a,b,c etc 
    let telefone :f32 = 98765432.0; // flot numeros quebrados 1.3 , 3.5 etc
    let id: i32 = 022; // numeros inteiro 1, 2,3 etc
    let status: bool = true; // vdd ou falso 
    let token: char ='%';
    println!("nome= {} \ntelefone= {}\nid={}\nstatus={}\ntoken={}",nome,telefone,id,status,token); // \n é a forma de se escrever println!  e ele ja pula a lina
}

fn main(){
    let var1: i32 = 12; 
    let var2: i32 = 9;
    let var3: f32 = 2.3;
    let var1 =var1 as f32 //transforma a var1 em dados que aceita numeros quebrados
    let resp = var1 + var3; // vc não pode somar com dados diferentes
    println! ("Resposta ={}",resp);  //  print normal 
}

fn main(){
    println("--ifood--")
    let var1: i32 = 2;
    if var1 == 1 {

fn main(){
    println("--ifood--")
    let var1: i32 = 2;
    if var1 == 1 {
        println!("cardapio");
    }else if var1 ==2 { 
        println!("carrinho");
    }else if var1 ==3 {
        println!("pagamento");
    }else if var1 ==0 {
        println!("sair");
    }else {
        println!("não é nenhuma das opcão acima")
    }
}
