use futures::executor::block_on;

fn main() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
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
