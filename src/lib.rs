use std::{
    sync::{Arc, mpsc},
    thread,
};

const THREADS_NUMBER: usize = 4;

fn run2() {
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

fn run3() {
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

fn run1() {
    let number_array: Vec<usize> =
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let arc_number_array = Arc::new(number_array);

    let chunk_size = (Arc::clone(&arc_number_array).len() + THREADS_NUMBER - 1) / THREADS_NUMBER;
    println!("Chunk size is: {}", chunk_size);

    let (tx, rx) = mpsc::channel::<usize>();

    let mut handles = Vec::new();

    for i in 0..THREADS_NUMBER {
        let thread_arc = Arc::clone(&arc_number_array); // Clone the POINTER
        let sender = tx.clone();

        let start = i * chunk_size;
        let mut end = start + chunk_size;
        if end > thread_arc.len() {
            end = thread_arc.len();
        }

        let handle = thread::spawn(move || {
            let chunk = &thread_arc[start..end];
            let sum: usize = chunk.iter().sum();
            sender.send(sum).unwrap();
        });
        handles.push(handle);
    }

    drop(tx);

    let mut sum: usize = 0;

    for received in rx {
        println!("Got: {received}");
        sum += received;
    }

    println!("Final total sum: {}", sum);

    handles.into_iter().for_each(|h| {
        h.join().unwrap_or_else(|err| {
            println!("Thread error: {:?}", err);
        })
    });
}

pub fn run() {
    run1();
}
