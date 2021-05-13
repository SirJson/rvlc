use std::ffi::CString;

use crate::traits::PointerAccess;
use crate::{media::Media, not_null};
use libvlc_sys::*;

type PlayerPointer = *mut libvlc_media_player_t;
use crate::internal_error::VLCResult;

pub struct Player(PlayerPointer);

impl Player {
    pub fn by_media(source: &Media) -> VLCResult<Player> {
        let ptr = unsafe { libvlc_media_player_new_from_media(source.get_ptr()) };
        not_null!(ptr, "New player by media");
        Ok(Player(ptr))
    }

    pub fn stop(&self) {
        unsafe {
            libvlc_media_player_stop(self.0);
        }
    }

    pub fn play(&self) {
        unsafe {
            libvlc_media_player_play(self.0);
        }
    }

    pub fn playing(&self) -> bool {
        unsafe {
            if libvlc_media_player_is_playing(self.0) >= 1 {
                true
            } else {
                false
            }
        }
    }

    pub fn position(&self) -> f32 {
        unsafe { libvlc_media_player_get_position(self.0) }
    }

    pub fn length(&self) -> i64 {
        unsafe { libvlc_media_player_get_length(self.0) }
    }

    pub fn time(&self) -> i64 {
        unsafe { libvlc_media_player_get_time(self.0) }
    }
    pub fn audio_out(&self, psz_name: &str) {
        unsafe {
            let str = CString::new(psz_name.as_bytes()).unwrap();
            libvlc_audio_output_set(self.0, str.as_ptr());
        }
    }

    pub fn enumerate_output_devices(&self) {
        unsafe {
            let devices = libvlc_audio_output_device_enum(self.0);
            let slice = std::slice::from_raw_parts(devices, 1);
            let s = slice[0];
            loop {
                let device_string = CString::from_raw(s.psz_device);
                let descr_string: CString = CString::from_raw(s.psz_description);
                log::info!("{:?}                {:?}", &device_string, &descr_string);
                if s.p_next == std::ptr::null_mut() {
                    break;
                }
            }
        }
    }
}
