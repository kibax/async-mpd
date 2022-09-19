// To use tokio you would do:
// use tokio as runtime;
use async_std as runtime;

#[runtime::main]
async fn main() -> Result<(), async_mpd::Error> {
    femme::with_level(log::LevelFilter::Debug);

    // Connect to server
    let mut mpd = async_mpd::MpdClient::new();
    mpd.connect("localhost:6600").await?;

    // Get all tracks in the play queue
    let mut queue = mpd.queue().await?;

    // Print the queue
    for track in queue {
        println!(
            "{:3}: {} - {}",
            track.id.unwrap_or(0),
            track.artist.unwrap_or_else(|| "<NoArtist>".to_string()),
            track.title.unwrap_or_else(|| "<NoTitle>".to_string()),
        );
    }

    // move a song in the queue
    mpd.queue_moveid(13, "2").await?;

    // Get all tracks in the play queue
    queue = mpd.queue().await?;

    // Print the queue
    for track in queue {
        println!(
            "{:3}: {} - {}",
            track.id.unwrap_or(0),
            track.artist.unwrap_or_else(|| "<NoArtist>".to_string()),
            track.title.unwrap_or_else(|| "<NoTitle>".to_string()),
        );
    }

    Ok(())
}
