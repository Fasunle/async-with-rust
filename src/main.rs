use tokio::join;

fn main() {
    // async
    let _books = async {
        get_books_async().await;
    };
    // thread
    get_books_thread();
}

fn download(url: &str) {
    println!("Downloading {} now...", url);
}

async fn download_async(url: &str) {
    println!("Downloading {} now...", url);
}

fn get_books_thread() {
    // create thread
    let thread_one = std::thread::spawn(|| download("Jupiter Janitor"));
    let thread_two = std::thread::spawn(|| download("Nigeria is cool"));

    // wait for each thread to complete by joining them
    thread_one.join().expect("Thread one paniced!");
    thread_two.join().expect("Thread two paniced!");
}

async fn get_books_async() {
    let future_one = download_async("Galaxy 9");
    let future_two = download_async("Not again!");

    // run all future concurrently and return a tuple of futures in order
    join!(future_one, future_two);
}
