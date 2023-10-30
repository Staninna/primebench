use std::time::{Duration, Instant};

fn primes(chunk: Vec<usize>) {
    let n = chunk[0];
    let max_n = chunk[chunk.len() - 1];

    let mut is_prime = vec![true; max_n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for p in 2..=((max_n as f64).sqrt() as usize) {
        if is_prime[p] {
            for i in ((n / p) * p..=max_n).step_by(p) {
                is_prime[i] = false;
            }
        }
    }
}

mod encode;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let start_n = envvar!("START_N", usize);
    let max_n = envvar!("MAX_N", usize);
    let min_threads = envvar!("MIN_THREADS", usize);
    let max_threads = envvar!("MAX_THREADS", usize);
    let tries = envvar!("TRIES", usize);

    println!(
        "N's: {} ({} to {})
Threads: {} to {}
Tries: {}",
        encode::encode((max_n - start_n) as u64),
        encode::encode(start_n as u64),
        encode::encode(max_n as u64),
        min_threads,
        max_threads,
        tries
    );

    let mut best_threads = min_threads;
    let mut best_time = Duration::from_secs(u64::MAX);
    let total_tries = tries * (min_threads..=max_threads).count();
    let now = Instant::now();

    for i in 0..tries {
        println!("Try {}/{}", i + 1, tries);
        for threads in min_threads..=max_threads {
            let mut handles = vec![];
            for _ in 0..threads {
                let ns = (start_n..=max_n).collect::<Vec<usize>>();
                let chunk_size = ns.len() / threads;
                let chunk = ns.chunks(chunk_size).nth(threads - 1).unwrap().to_vec();

                handles.push(tokio::spawn(async move { primes(chunk) }));
            }

            let start = Instant::now();
            for handle in handles {
                handle.await.unwrap();
            }
            let elapsed = start.elapsed();

            if elapsed < best_time {
                best_time = elapsed;
                best_threads = threads;

                println!(
                    "New best time: {}s ({:?}) with {} threads",
                    best_time.as_secs_f64(),
                    best_time,
                    best_threads
                );
            }
        }
    }

    print!("\x1B[2J\x1B[1;1H");
    println!(
        "Best number of threads: {}
Best time: {}s ({:?})
Total time: {:?}
N's: {} ({} to {})
Time per N: {}s
Total tries: {}",
        best_threads,                                         // Best number of threads
        best_time.as_secs_f64(),                              // Best time
        best_time,                                            // Best time
        now.elapsed(),                                        // Total time
        encode::encode((max_n - start_n) as u64),             // N's
        encode::encode(start_n as u64),                       // N's
        encode::encode(max_n as u64),                         // N's
        (best_time.as_secs_f64() / (max_n - start_n) as f64), // Time per N
        total_tries                                           // Total tries
    );
}

#[macro_export]
macro_rules! envvar {
    ($name:expr, $type:ty) => {
        dotenvy::var($name)
            .expect(format!("{} not set", $name).as_str())
            .parse::<$type>()
            .expect(format!("{} is not a {}", $name, stringify!($type)).as_str())
    };
}
