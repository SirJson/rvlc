use anyhow::Result;
use env_logger;
use rvlc::{self, Media, Player, media::{DUMMY_INTERFACE, DUMMY_VIDEO, NO_VIDEO, ParseFlags, ParseStatus}};
use std::{sync::Arc};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;


fn main() -> Result<()> {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;

    std::env::set_var("RUST_LOG", "DEBUG"); // We kinda force this here, sorry
    env_logger::try_init()?;
    log::info!("New libvlc instance");
    let vlc = rvlc::VLCInstance::new()?.with_interface(rvlc::VLCInterface::Dummy);

    let media = Media::by_location("https://www.youtube.com/watch?v=NJLfCBBcZAo", &vlc)?;
    log::info!("Youtube video parsing: https://www.youtube.com/watch?v=NJLfCBBcZAo");
    media.parse_async(ParseFlags::LOCAL | ParseFlags::FETCH_NETWORK | ParseFlags::NETWORK, 1000*10)?;
    let parseresult = media.block_until_parsed();
    if parseresult != ParseStatus::Done {
        return Err(anyhow::Error::msg("Parsing failed"));
    }
    log::info!("Submedia count = {}", media.subitem_count());
    let target = if media.subitem_count() > 0 {
        log::info!("Grab first result");
        media.grab_subitem(0)?
    } else {
        media
    }.add_option(NO_VIDEO).add_option(DUMMY_INTERFACE).add_option(DUMMY_VIDEO);
    let list = rvlc::TrackList::from_media(&target)?;
    for i in 0..list.length() {
        log::debug!("TRACK: {:#?}",list.get(i));
    }
    log::info!("Starting with: ({:?})\n", target);

    let player = Player::by_media(&target)?;
    player.play();
    let tsleep = std::time::Duration::from_millis(100);
    log::info!("Started play! Cancel with CTRL+C");
    while !term.load(Ordering::Relaxed) {
        // TODO: Since libvlc runs independent from us it's kind of hard right now to detect if the song is still loading but will be played or is already finished. Solution?
        std::thread::sleep(tsleep);
    }
    log::info!("Player exit");
    Ok(())
}
