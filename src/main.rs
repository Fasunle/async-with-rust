use futures::executor::block_on;

fn main() {
    block_on(async_main());
}

struct Song {
    title: String,
    singer: String,
}

async fn learn_song() -> Song {
    Song {
        title: "Sell it on me!".to_string(),
        singer: "Kehinde Fasunle".to_string(),
    }
}

async fn sing_song(song: Song) {
    println!(
        "I am singing a song titled {} by {}",
        song.title, song.singer
    );
}

async fn dance() {
    println!("I am dancing right now");
}

async fn learn_and_sing() {
    // using await keyword
    let song = learn_song().await; // returns a value
    sing_song(song).await;
}

async fn async_main() {
    let future_one = learn_and_sing(); // returns a future
    let future_two = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.

    futures::join!(future_one, future_two); // simontaneously poll the futures
}
