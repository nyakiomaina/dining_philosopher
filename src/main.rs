use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork {
    id: usize,
}

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn new(
        name: impl Into<String>,
        left_fork: Arc<Mutex<Fork>>,
        right_fork: Arc<Mutex<Fork>>,
        thoughts: mpsc::Sender<String>,
    ) -> Philosopher {
        Philosopher {
            name: name.into(),
            left_fork,
            right_fork,
            thoughts,
        }
    }

    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
        thread::sleep(Duration::from_millis(10));
    }

    fn eat(&self) {
        let _left_fork = self.left_fork.lock().unwrap();
        let _right_fork = self.right_fork.lock().unwrap();

        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];
fn main() {
    let (tx, rx) = mpsc::channel();

    let forks: Vec<_> = (0..5)
        .map(|id| Arc::new(Mutex::new(Fork { id })))
        .collect();

    let philosophers: Vec<_> = PHILOSOPHERS
        .iter()
        .enumerate()
        .map(|(i, &name)| {
            let left_fork = forks[i].clone();
            let right_fork = forks[(i + 1) % forks.len()].clone();

            Philosopher::new(name, left_fork, right_fork, tx.clone())
        })
        .collect();

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|philosopher| {
            thread::spawn(move || {
                for _ in 0..100 {
                    philosopher.think();
                    philosopher.eat();
                }
            })
        })
        .collect();

    for thought in rx.iter().take(500) {
        println!("{}", thought);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
