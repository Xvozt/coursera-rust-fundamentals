fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]

    add_to_both_ends_of_vector(&mut v, 9);
    println!("{:?}", v);

    let mut add_to_v = vec![10, 11];
    add_two_vectors(&mut v, &mut add_to_v);
    println!("{:?}", v);
}

fn add_to_both_ends_of_vector(v: &mut Vec<i32>, value: i32) {
    v.insert(0, value);
    v.push(value)
}

fn add_two_vectors(v1: &mut Vec<i32>, v2: &mut Vec<i32>) {
    v1.extend(v2.drain(..));
}
