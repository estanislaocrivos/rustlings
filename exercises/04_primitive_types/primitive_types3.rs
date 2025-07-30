fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a: [i32; 100] = [0; 100];

    let mut index: i32 = 0;
    for element in a.iter() {
        println!("a[{}] = {}", index, element);
        index += 1;
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
