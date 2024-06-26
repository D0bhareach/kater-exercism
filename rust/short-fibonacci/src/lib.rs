/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    // unimplemented!("create a zeroized buffer of {} bytes", count)
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    let mut cur: u8 = 1;
    let mut prev: u8 = 0;
    #[allow(unused_assignments)]
    let mut tmp: u8 = 0;
    for _ in 0..5 {
        res.push(cur);
        tmp = prev;
        prev = cur;
        cur = tmp + cur;
    }
    res
}
