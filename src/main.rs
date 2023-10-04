//// 2023/10/04 // 15:14 //

//// # 14 Vectores

// Vectores
// Los vectores son estructuras de datos que nos vn a permitir almacenar varions
// valores en una única estructura, colocando todos los valores en posiciones de
// memoria consecutivas.

// Los vectores solo pueden almacenar valores del mismo tipo. Son útiles cuando
// tiene una lista de artículos del mismo tipo, como las notas de un curso o los
// precios de los artículos en un carrito de compras.

fn main() {

    let mut v0: Vec<i32> = Vec::new();
    v0.push(5);
    v0.push(98);
    v0.push(32);
    println!("La longitud del vector es: {}", v0.len());

    v0.pop();
    println!("La longitud del vector es: {}", v0.len());


    let v1 = vec![1, 2, 3, 4, 5];

    let tercero: &i32 = &v1[2];
    println!("El tercer elemento es: {}", tercero);


    match v1.get(3) {
        Some(tercero) => println!("Coincide con el tercer elemento"),
        None => println!("No coincide"),
    }


    // let elemento_no_existe = &v1[500];
    let elemento_no_existe = v1.get(500);


    let mut v2 = vec![1, 2, 3, 4, 5];
    // let primero = &v2[0];
    // v2.push(6);
    // println!("El primer elemento es: {}", primero);


    for i in &v2 {
        println!("{}", i);
    }

    
    for i in &mut v2 {
        *i += 10;
    }

    for i in &v2 {
        println!("{}", i);
    }


    let fila = vec![
        CeldaHojaCalculo::Int(9),
        CeldaHojaCalculo::Text(String::from("Texto de ejemplo")),
        CeldaHojaCalculo::Float(9.81),
    ];

    // for i22 in &fila {
    //     println!("{}", i22);
    // }

}

enum CeldaHojaCalculo {
    Int(i32),
    Float(f64),
    Text(String),
}
