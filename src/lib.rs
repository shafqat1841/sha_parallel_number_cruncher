use std::thread;

const THREADS_NUMBER: usize = 4;

fn run1(){

    let number_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let number_array_chunk_size = (number_array.len() + THREADS_NUMBER - 1) / THREADS_NUMBER;
    println!("Chunk size is: {}", number_array_chunk_size);
    
    let mut start_chunk = 0;
    
    let mut end_chunk = start_chunk + number_array_chunk_size;
    
    let mut handlers = Vec::new();
    
    for thread_number in 0..THREADS_NUMBER {
        let func = move || {
            let mut total = 0;
            for i in start_chunk..end_chunk {
                println!(
                    "thread number: {}, start chunk: {} and end chunk: {}, number: {}",
                    thread_number, start_chunk, end_chunk, number_array[i]
                );
                total += number_array[i];
            }
            println!("Total for thread number {} is: {}", thread_number, total);
            total
        };
    
        start_chunk += number_array_chunk_size;
        end_chunk += number_array_chunk_size;
        if end_chunk > number_array.len() {
            end_chunk = number_array.len();
        }
    
        let handle = thread::spawn(func);
        handlers.push(handle);
    }
    
    let mut total_sum = 0;
    
    for handle in handlers {
        let res = handle.join();
        let res = match res {
            Ok(value) => value,
            Err(err) => {
                println!("Thread error: {:?}", err);
                continue;
            }
        };
        println!("Thread result is: {}", res);
        total_sum += res;
    }
    
    println!("Total sum is: {}", total_sum);
} 

pub fn run() {
    run1();
}
