use std::mem;

fn revtuple(pair: (i32, bool)) -> (bool, i32) {
    (pair.1, pair.0)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn analyze_slice(slice: &[i32]) {
    println!("\tFirst element of slice is: {}", slice[0]);
    println!("\tThis slice has {} elements", slice.len());
}

pub fn run() {
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);
    println!(
        "Tuples are immutable and can contain any number of tupes: {:?}",
        tuple_of_tuples
    );

    let pair = (1, false);
    println!("pair is {:?}", pair);
    println!("Reversed pair is {:?}", revtuple(pair));

    let tup = (1, "lincoln", 4.5, false);

    let (a, b, c, d) = tup;
    println!(
        "You can destructure tuples as well: {:?}, {:?}, {:?}, {:?}",
        a, b, c, d
    );

    let mat = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("mat is a tuple struct: {:?}", mat);

    /* Fixed size array */
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!(
        "Array is of len {} and contains {} bytes",
        xs.len(),
        mem::size_of_val(&xs)
    );
    println!("Borrowing array as slice...");
    analyze_slice(&xs);

    println!("Borrow a subset of an array as slice...");
    analyze_slice(&ys[3..17]);
}
