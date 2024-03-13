// This crate imlements simple example of TimeFuture, Task, Executor implementation.
// See main func comment to get general description
use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use std::{
    future::Future,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    task::Context,
    time::Duration,
};
use bks_asyncbk_ch2_2to4::TimerFuture;

// Task executor that receives tasks off of a channel and runs them.
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

// Spawner spawns new futures onto the task channel.
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    // spawn task from future and put to executor queue
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

// A future that can reschedule itself to be polled by an Executor.
struct Task {
    //  In-progress future that should be pushed to completion.
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    // Handle to place the task itself back onto the task queue.
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    // Implement wake by sending this task back onto the task channel
    // so that it will be polled again by the executor.
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // Take the future, and if it has not yet completed (is still Some),
            // poll it in an attempt to complete it.
            let mut future_slot = task.future.lock().unwrap();
            // Takes the value out of the option, leaving a None in its place. 
            if let Some(mut future) = future_slot.take() {
                // Create a LocalWaker from the task itself
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                // BoxFuture<T> is a type alias for Pin<Box<dyn Future<Output = T> + Send + 'static>>.
                // We can get a Pin<&mut dyn Future + Send + 'static> from it by calling
                // the Pin::as_mut method.
                if future.as_mut().poll(context).is_pending() {
                    // We're not done processing the future, so put it
                    // back in its task to be run again in the future.
                    *future_slot = Some(future);
                }
            }
        }
    }
}

// This crate imlements simple example of TimeFuture, Task, Executor implementation.
// Async code or Future run/compute as task (wraps Future) on executor
// Future implement poll() to advance itself as long as posible (called by executor)
// poll accept wake() (wrapped to context). Calling wake Future notify Executor about readiness (polled).
// One thing not clear for me how future.poll know which of nested future's poll need call.
fn main() {
    let (executor, spawner) = new_executor_and_spawner();
    // Spawn a task.
    spawner.spawn(async {
        println!("howdy!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });
    // drop spawner (will drop sender) will notify executor that channel is closed
    drop(spawner);
    println!("before executer run");
    // Run the executor until the task queue is empty.
    executor.run();
}
