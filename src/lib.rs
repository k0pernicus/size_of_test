#[cfg(test)]
mod tests {
    use std::mem;
    #[test]
    // (capacity * sizeof(T) + sizeof(usize) * 3) --> 3 = pointer to the heap data, capacity, length
    fn size_of_test() {
        let v0: Vec<u8> = Vec::with_capacity(0);
        let v128: Vec<u8> = Vec::with_capacity(128);
        let w: Vec<u8> = (0..128).collect();
        let x = [0u8; 128];
        println!("*******************************");
        println!("POINTERS");
        println!("Size of <usize> type is {}", mem::size_of::<usize>());
        println!(
            "Size of empty vec (without capacity of 0) is {}",
            mem::size_of_val(&v0)
        );
        println!(
            "Size of empty vec (without capacity of 128) is {}",
            mem::size_of_val(&v128)
        );
        println!("Size of initialized vec is {}", mem::size_of_val(&w));
        println!("Size of initialized array is {}", mem::size_of_val(&x));
        println!("*******************************");
        println!("CONTENT");
        println!("Size of <usize> type is {}", mem::size_of::<usize>());
        println!(
            "Size of empty vec (without capacity of 0) is {}",
            mem::size_of_val(&v0[..])
        );
        println!(
            "Size of empty vec (without capacity of 128) is {}",
            mem::size_of_val(&v128[..])
        );
        println!("Size of initialized vec is {}", mem::size_of_val(&w[..]));
        println!("Size of initialized array is {}", mem::size_of_val(&x[..]));
    }
}
