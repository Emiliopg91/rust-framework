use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use crate::event_bus::EventBus;

#[tokio::test]
async fn test_01_on_event() {
    let val = Arc::new(AtomicUsize::new(0));
    EventBus::on(
        "test_event",
        Arc::new(move |data| {
            Box::pin(async move {
                data[0]
                    .downcast_ref::<Arc<AtomicUsize>>()
                    .unwrap()
                    .fetch_add(1, Ordering::SeqCst);
            })
        }),
    );

    EventBus::emit("test_event", &[Box::new(val.clone())]).await;
    assert!(val.load(Ordering::SeqCst) == 1);
}
