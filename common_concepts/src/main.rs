fn main() {
    // let a = 10;
    // let mut b = 2;

    // const HIGHEST_PRICE: u32 = 100_000_000_000;

    // let x = 10;
    // let x = "ten";

    // scalar: integer, floating number, boolean, characters
    // let y: u16 = 10;
    // let f: f32 = 2.1;
    // let valid = true;
    // let invalid: bool = false;
    // let hex = 0xff;
    // let octal = 0o77;
    // let bin = 0b1101001;
    // let byte = b'A';

    // compund: tuples dan arrays
    // let tup: (i32, f64, u8) = (100, 1.3, 1);
    // let (x, y, z) = tup;
    // let first = tup.0;

    // let tup2: () = ();

    // let array = [1, 2, 3];
    // let array_1: [i32; 5] = [1, 2, 3, 4, 5];
    // let array_2 = [3, 5]; // => let b = [3, 3, 3, 3, 3]
    // let first = array_2[0];

    // println!("Hello world");
    // my_function(20, 'X');
    // let res = fungsi_tambah(10, 20);
    // println!("{res}")

    looping();
}

// fn my_function(value: i32, label: char) {
//     println!("Result {}-{}", value, label);
// }

// fn fungsi_tambah(a: i32, b: i32) -> i32 {
//     a + b
// }

// inline commmand

/*
    block command
*/

// fn if_elseif_else() {
//     let num = 3;

//     if num < 5 {
//         println!("condition true");
//     } else if num % 3 == 0 {
//         println!("num is divisible by 3");
//     } else {
//         println!("condition false");
//     }

//     let cond = true;
//     let num2 = if cond { 5 } else { 6 };
//     println!("{num2}");
// }

fn looping() {
    // let mut counter: usize = 0;
    // let arr = [2, 3, 1, 4, 7, 5, 6];

    // let result = loop {
    // println!("{}", arr[counter]);
    //     if arr[counter] == 5 {
    //         break "found 5";
    //     }

    //     counter += 1;
    // };

    // println!("{}", result);

    // let mut num = 6;

    // while num != 0 {
    //     println!("{num}");
    //     num -= 1;
    // }

    // println!("bye ...")

    // let arr = [1, 2, 3];

    // for element in arr {
    //     println!("The value is : {}", element);
    // }

    // for (i, element) in arr.into_iter().enumerate() {
    //     println!("element index ke {} adalah {}", i, element);
    // }

    for i in 1..=4 {
        println!("{}", i);
    }
}
