fn main() {
    //prueba_inmutabilidad();
    //datos_compuestos();
    //factorial(10)
    // let x = String::from("holas");
    // nuevo_duenio(&x);
    // println!("soy last {}", x)
    let mut x2 = String::from("hasta ");
    add_to_string(&mut x2);
    println!("soy mut {}", x2)
}
//un macro no es nada más que una forma de generar código en tiempo de compilación
//8 bits = un byte por lo que i8 ocupa un byte, y puede contener valores entre el -128 y el 127.
//i64 ocupa 64 bits o, lo que es lo mismo, 8 bytes y puede representar números entre el -9223372036854775808 y el 9223372036854775807.
//(isize y usize) El número de bits del tipo nativo del procesador de tu ordenador
// const CONSTANTE: i8 = 1;
// fn prueba_inmutabilidad() {
//     let mut x = 1;
//     println!("el valor es  {}", x); // macro
//     x = 2;
//     println!("el valor 2 es  {}", x);
// }

// fn datos_compuestos() -> i32 {
//     //aquéllos cuyos valores pueden en globar a varios datos simultáneamente.
//     let tuplas = (1, 2.32, "b");
//     let array = [1, 2, 3, 4];
//     for i in array.iter() {
//         println!("number{}", i)
//     }
//     let mut ent = 0;
//     while ent < 10 {
//         //println!("ent test {}", ent);
//         ent += 1
//     }
//     let mut ent2 = 0;
//     loop {
//         ent2 = ent2 + 10;
//         println!("im ent2 {}", ent2);
//         if ent2 > 30 {
//             break;
//         }
//     }
//     ent2
// }

// fn factorial(number: u128) {
//     let mut result = number;
//     for i in (1..number).rev() {
//         result *= i;
//     }
//     println!("im i result {}", result)
// }

// fn string (){
//     let x = String::from("hola mundo");
// }
// fn nuevo_duenio(sting: &String) {
//     println!("stirng {}", sting)
// }
fn add_to_string(s: &mut String) {
    s.push_str("nunca");
}
