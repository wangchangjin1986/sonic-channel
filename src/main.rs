use sonic_channel::{IngestChannel, SearchChannel, SonicChannel};
use sonic_channel::result::*;
fn main() -> Result<()> {
    let ingestChannel = IngestChannel::start("127.0.0.1:1491","test")?;
    ingestChannel.push("collection", "bucket", "object:1", "my best recipe")?;
    let searcgChannel = SearchChannel::start("127.0.0.1:1491", "test")?;
    let objects = searcgChannel.query("collection", "bucket", "recipe")?;
    return Ok(());
}