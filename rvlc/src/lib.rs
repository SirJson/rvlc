mod ffi_macros;
pub mod internal_error;
pub mod media;
pub mod player;
pub mod prelude;
mod traits;
pub mod vlc;

pub use vlc::VLCInstance;
pub use vlc::VLCInterface;
pub use media::{Media, TrackList};
pub use player::Player;