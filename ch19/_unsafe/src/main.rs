use core::slice;

fn main() {
    // dereferencing a raw pointer
    let mut num = 5;

    // creating raw pointers from references
    let r1 = &num as *const i32;
    let r2 = &mut num as &mut i32;

    // creating a raw pointer to an arbitrary memory address
    let address = 0x12345678usize;
    #[allow(unused_variables)]
    let r = address as *const i32;

    // dereferencing raw pointers
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // println!("r is: {}", *r); // segfault
    }
    println!();

    // calling an unsafe function or method
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // creating a safe abstraction over unsafe code
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(&mut v, 3);
    println!("{:?} {:?}", a, b);

    // using extern functions to call external code
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!();

    // accessing or modifying a mutable static variable
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // implementing an unsafe trait
    unsafe trait Foo {}

    unsafe impl Foo for i32 {}
}
