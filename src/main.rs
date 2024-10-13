//Primer caso de return clasico

/* fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
fn main() {
    let resultado = is_even(34);
    println!("El resultado es: {}",resultado);
} */

//Nueva forma de implicit return 
//En el caso de la una implementation el implicit return es mas abreviado ver el repositorio "implementations_methods"

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        //implicit return
        true
    } else {
        false
    }
}
fn main() {
    let resultado = is_even(34);
    println!("El resultado es: {}",resultado);
}
