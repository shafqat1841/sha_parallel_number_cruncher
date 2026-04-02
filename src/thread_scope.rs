use std::thread;

use crate::constants::THREADS_NUMBER;


pub fn use_thread_scope() {
    let number_array: Vec<usize> =
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let chunk_size = (number_array.len() + THREADS_NUMBER - 1) / THREADS_NUMBER;
    println!("Chunk size is: {}", chunk_size);

    let results = thread::scope(|s| {
        let mut handlers = Vec::new();

        for (i, chunk) in number_array.chunks(chunk_size).enumerate() {
            let handle = s.spawn(move || {
                let sum = chunk.iter().sum();
                println!(
                    "Thread number: {}, chunk length: {}, sum: {}",
                    i,
                    chunk.len(),
                    sum
                );
                sum
            });

            handlers.push(handle);
        }

        let total: usize = handlers
            .into_iter()
            .map(|h| {
                h.join().unwrap_or_else(|err| {
                    println!("Thread error: {:?}", err);
                    0
                })
            })
            .sum();

        total
    });

    println!("Final total sum: {}", results);
}
