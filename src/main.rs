use std::thread;

const THREADS_NUMBER: usize = 4;

fn main() {
    let number_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut handlers = Vec::new();

    for thread_number in 0..=THREADS_NUMBER {
        let f = move || {
            for i in number_array.iter() {
                println!(
                    "Thread number is: {} and the number is: {}",
                    thread_number, i
                );
            }
        };
        let handle = thread::spawn(f);
        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    // println!("{}", number_array);
}
