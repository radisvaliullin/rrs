use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};


// Shared state between the future and thread with time sleep.
struct SharedState {
    // Flag about status of sleep timer done or not.
    completed: bool,
    // The waker for the task that TimerFuture is running on.
    // The thread can use this to tell TimerFuture's task to wake up.
    waker: Option<Waker>,
}

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completed.
        let mut shared_state = self.shared_state.lock().unwrap();
        // time status
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // share waker (we need clone waker each time because Future can moved another task)
            // waker gives to thread wake up current task of TimeFuture when the timer done 
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    // new TimerFuture which will complete after the provided timeout
    pub fn new(duration: Duration) -> Self {
        // new shared_state to interact with spawn thread
        let shared_state = Arc::new(Mutex::new(SharedState{
            completed: false,
            waker: None,
        }));

        // spawn new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // Set timer has completed flag
            // Wake up the last task on which the future was polled, if one exists
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}
