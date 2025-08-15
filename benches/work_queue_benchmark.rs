use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nexa_vortex_core::integrations::VortexWorkQueue;
use std::sync::Arc;
use std::thread;

fn work_queue_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("VortexWorkQueue");

    group.bench_function("single_thread_push_pop", |b| {
        let queue = VortexWorkQueue::<i32>::new(1024).unwrap();
        b.iter(|| {
            queue.push(black_box(1)).unwrap();
            queue.pop();
        })
    });

    group.bench_function("multi_thread_spsc", |b| {
        b.iter_with_setup(
            || {
                let queue = Arc::new(VortexWorkQueue::<i32>::new(1024).unwrap());
                (queue, 1000)
            },
            |(queue, count)| {
                let producer_queue = queue.clone();
                let producer = thread::spawn(move || {
                    for i in 0..count {
                        producer_queue.push(i).unwrap();
                    }
                });

                let consumer_queue = queue.clone();
                let consumer = thread::spawn(move || {
                    let mut received = 0;
                    while received < count {
                        if consumer_queue.pop().is_some() {
                            received += 1;
                        }
                    }
                });

                producer.join().unwrap();
                consumer.join().unwrap();
            },
        );
    });

    group.finish();
}

criterion_group!(benches, work_queue_benchmark);
criterion_main!(benches);

