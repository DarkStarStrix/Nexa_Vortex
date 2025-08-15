use nexa_vortex_core::integrations::VortexWorkQueue;
use std::sync::Arc;
use std::thread;

#[test]
fn test_spsc_queue_integrity() {
    let q = Arc::new(VortexWorkQueue::new(1024).unwrap());
    let item_count = 5000;

    let producer = {
        let q_clone = Arc::clone(&q);
        thread::spawn(move || {
            for i in 0..item_count {
                q_clone.push(i).unwrap();
            }
        })
    };

    let consumer = {
        let q_clone = Arc::clone(&q);
        thread::spawn(move || {
            let mut seen_count = 0;
            let mut next_expected = 0;
            // Loop until all items are received
            while seen_count < item_count {
                if let Some(v) = q_clone.pop() {
                    assert_eq!(v, next_expected, "Items should be received in order");
                    next_expected += 1;
                    seen_count += 1;
                }
            }
            assert_eq!(seen_count, item_count);
        })
    };

    producer.join().unwrap();
    consumer.join().unwrap();
}

