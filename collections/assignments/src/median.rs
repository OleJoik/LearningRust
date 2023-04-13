pub fn print(array: &[i32]) {
    let mut mutable_integers: Vec<i32> = array.to_owned();
    mutable_integers.sort();
    println!("Sorted integers are {:?}", mutable_integers);

    let length = mutable_integers.len();

    let median: i32;
    if length % 2 == 0 {
        let middle_right = mutable_integers[length / 2];
        let middle_left = mutable_integers[(length / 2) - 1];
        median = (middle_left + middle_right) / 2;
    } else {
        median = mutable_integers[length / 2];
    }

    println!("Median is {median}");
}
