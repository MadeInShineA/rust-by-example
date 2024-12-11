#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    let deep: Deep = Deep(Structure(69));
    println!("Deep : {deep:#?}");
    println!("Deep : {deep:?}");
}
