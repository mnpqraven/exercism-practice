/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    let v = Vec::new();
    v
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    let mut index = 0;
    while index < count {
        v.push(0);
       index = index + 1;
    }
    v
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut v = create_buffer(5);
    let mut first = 1;
    let mut second = 1;
    let mut next = first + second;

    let mut index = 0;
    while index + 1< v.len() {
        v[index]  = first;
        v[index+1]= second;
        first = second;
        second = next;
        next = first + second;
        println!("{:?}",v);
        index += 1;
    }
        dbg!(v)
}
