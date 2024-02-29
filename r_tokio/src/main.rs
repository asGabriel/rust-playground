// single thread
// #[tokio::main(flavor = "current_thread")] 
#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() {
    // single thread test.
    test_something().await;

    let racer01 = F1Racer::new();
    println!("{:#?}", &racer01);
    let best_lap_time = racer01.await;
    println!("Best lap time was {}", best_lap_time);
}

async fn test_something() {
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("Hello from Tokio");
}

#[derive(Debug)]
struct F1Racer {
    name: String,
    completed_laps: u8,
    laps: u8,
    best_lap_time: u8,
    lap_times: Vec<u8>
}

impl F1Racer {
    fn new() -> F1Racer {
        F1Racer { name: "Max Verstapen".to_string(), completed_laps: 0, laps: 5, best_lap_time: 255, lap_times: vec![87, 64, 126, 95, 76] }
    }

    fn do_lap(&mut self) {
        println!("{} is doing a new lap", self.name);
        let lap_time = self.lap_times.pop();

        if lap_time.is_some() && lap_time.unwrap() < self.best_lap_time {
            self.best_lap_time = lap_time.unwrap();
        }

        self.completed_laps  += 1;
    }
}

impl std::future::Future for F1Racer {
    type Output = u8;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.completed_laps < self.laps {
            self.get_mut().do_lap();
            cx.waker().wake_by_ref();
            return  std::task::Poll::Pending;

        }
        println!("{} has completed all laps", self.name);
        return std::task::Poll::Ready(self.best_lap_time);
    }
}
