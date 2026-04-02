use std::{sync::mpsc, thread};

use crate::constants::THREADS_NUMBER;

pub fn use_channel_with_scope() {
    let number_array: Vec<usize> =
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let chunk_size = (number_array.len() + THREADS_NUMBER - 1) / THREADS_NUMBER;
    println!("Chunk size is: {}", chunk_size);

    let (tx, rx) = mpsc::channel::<usize>();
    thread::scope(|s| {
        for (i, chunk) in number_array.chunks(chunk_size).enumerate() {
            let sender = tx.clone();

            s.spawn(move || {
                let sum = chunk.iter().sum();
                println!(
                    "Thread number: {}, chunk length: {}, sum: {}",
                    i,
                    chunk.len(),
                    sum
                );
                sender.send(sum).unwrap();
            });
        }
    });
    drop(tx);

    let mut sum: usize = 0;

    for received in rx {
        println!("Got: {received}");
        sum += received;
    }

    println!("Final total sum: {}", sum);
}
