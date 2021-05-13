use crate::{expect_cpath, expect_cstring, internal_error::VLCResult, not_null};
use crate::{traits::PointerAccess, vlc::VLCInstance};
use libvlc_sys::*;
use log::debug;
use std::ffi::{CString};
use std::{os::raw::c_int, path::Path};
type MediaPointer = *mut libvlc_media_t;

#[derive(Debug)]
pub struct Media(MediaPointer);

pub type MediaHandle = VLCResult<Media>;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ParseStatus {
    Running,
    Done,
    Failed,
    Skipped,
    Timeout,
}

pub mod media_options;
pub use media_options::*;

bitflags::bitflags! {
   pub struct ParseFlags: u32 {
        const LOCAL = 0b00000001;
        const NETWORK = 0b00000010;
        const FETCH_LOCAL = 0b00000100;
        const FETCH_NETWORK = 0b00000100;
        //const all = Self::Local.bits | Self::Network.bits | Self::FetchLocal.bits | Self::FetchNetwork.bits;
    }
}

pub struct TrackList(*mut std::ffi::c_void);

impl TrackList {
    pub fn from_media(media: &Media) -> VLCResult<TrackList> {
        let ptr: *mut std::ffi::c_void = unsafe { rvlc_tracklist(media.get_ptr()) };
        Ok(TrackList(ptr))
    }

    pub fn length(&self) -> i32 {
        unsafe {
            return rvlc_tracklist_len(self.0);
        }
    }

    pub fn get(&self, index: i32) -> Track {
        unsafe {
            let raw = rvlc_tracklist_get(self.0, index);
            let item = *raw;
            let codec = match item.codec.as_ref() {
                Some(s) => s.to_string(),
                None => String::default(),
            };
            Track {
                filetype: item.filetype,
                codec: codec,
                channels: item.channels,
                bitrate: item.rate,
            }
        }
    }
}

impl std::ops::Drop for TrackList {
    fn drop(&mut self) {
        unsafe {
            rvlc_tracklist_drop(self.0);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    pub filetype: i32,
    pub channels: i32,
    pub bitrate: i32,
    pub codec: String,
}

impl Media {
    pub fn by_location(location: &str, vlc: &VLCInstance) -> MediaHandle {
        let cstr = expect_cstring!(location);
        let ptr = unsafe { libvlc_media_new_location(vlc.get_ptr(), cstr.as_ptr()) };
        not_null!(ptr, "Media allocation by location returned null pointer");
        Ok(Media(ptr))
    }

    pub fn by_path(path: &Path, vlc: &VLCInstance) -> MediaHandle {
        let pas = expect_cpath!(path);
        let ptr = unsafe { libvlc_media_new_path(vlc.get_ptr(), pas.as_ptr()) };
        not_null!(ptr, "Media allocation by path returned null pointer");
        Ok(Media(ptr))
    }

    pub fn by_fd(fd: i32, vlc: &VLCInstance) -> MediaHandle {
        let ptr = unsafe { libvlc_media_new_fd(vlc.get_ptr(), c_int::from(fd)) };
        not_null!(
            ptr,
            "Media allocation by file descriptor returned null pointer"
        );
        Ok(Media(ptr))
    }

    pub fn poll_parse_status(&self) -> ParseStatus {
        unsafe {
            let status = libvlc_media_get_parsed_status(self.0);
            match status {
                libvlc_media_parsed_status_t::libvlc_media_parsed_status_done => ParseStatus::Done,
                libvlc_media_parsed_status_t::libvlc_media_parsed_status_failed => {
                    ParseStatus::Failed
                }
                libvlc_media_parsed_status_t::libvlc_media_parsed_status_skipped => {
                    ParseStatus::Skipped
                }
                libvlc_media_parsed_status_t::libvlc_media_parsed_status_timeout => {
                    ParseStatus::Timeout
                }
                _ => ParseStatus::Running,
            }
        }
    }

    pub fn add_option(self, opt: &str) -> Self {
        unsafe {
            let optstr = CString::new(opt.to_string()).unwrap();
            libvlc_media_add_option(self.0, optstr.as_ptr());
        }
        self
    }

    pub fn parse_async(&self, flags: ParseFlags, timeout_ms: i32) -> std::io::Result<()> {
        unsafe {
            let cflag = libvlc_media_parse_flag_t(flags.bits());
            let cresult = libvlc_media_parse_with_options(self.0, cflag, timeout_ms);

            if cresult == -1 {
                Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
            } else {
                Ok(())
            }
        }
    }

    pub fn block_until_parsed(&self) -> ParseStatus {
        loop {
            let status = self.poll_parse_status();
            if status != ParseStatus::Running {
                return status;
            }
        }
    }

    pub fn grab_subitem(&self, index: i32) -> std::io::Result<Media> {
        log::debug!("Get subitem with index {0}", index);
        unsafe {
            let listptr = libvlc_media_subitems(self.0);
            if index >= libvlc_media_list_count(listptr) {
                return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof));
            }
            let mediaptr = libvlc_media_list_item_at_index(listptr, index);
            Ok(Media::from(mediaptr))
        }
    }

    pub fn subitem_count(&self) -> usize {
        unsafe {
            let listptr = libvlc_media_subitems(self.0);
            libvlc_media_list_count(listptr) as usize
        }
    }

    // pub fn subitems(&self) -> Vec<Media> {
    //     unsafe {
    //         let mut out: Vec<Media> = Vec::new();
    //         let listptr = libvlc_media_subitems(self.0);
    //         let count = libvlc_media_list_count(listptr);
    //         for i in 0..count {
    //             let item = libvlc_media_list_item_at_index(listptr, i);
    //             out.push(Media::from(item));
    //         }
    //         out
    //     }
    // }

    pub fn test(&self) {
        unsafe {
            let list = rvlc_tracklist(self.0);
            let len = rvlc_tracklist_len(list);
            if len > 0 {
                let data = rvlc_tracklist_get(list, 0);
                debug!("len = {}", len);
                let raw = *data;
                debug!("channels = {}", raw.channels);
            }
        }
    }

    /*
    unsafe fn trackfactory(&self) -> libvlc_media_track_t {
        let mut nullstr = [0];
        libvlc_media_track_t { i_codec: 0, i_original_fourcc: 0, i_id: 0, i_type: libvlc_track_type_t::libvlc_track_unknown, i_profile: 0, i_level: 0,  __bindgen_anon_1: libvlc_media_track_t__bindgen_ty_1, i_bitrate: 0, psz_language: nullstr.as_mut_ptr(), psz_description: nullstr.as_mut_ptr() }
    }

    pub fn track_info(&self) -> Vec<Track> {

        let mut tracks: Vec<Track> = Vec::new();
        unsafe {
            let mut proto = vec![self.trackfactory()];
            let p = proto.as_mut_ptr();

            let s = std::mem::size_of::<libvlc_track_type_t>();
            let x = Vec::from_raw_parts(p, s, s*128);

            let mut track_array = Vec::<*mut *mut libvlc_media_track_t>::new();

            let t = std::slice::from_raw_parts(, 1);;
            track_array.push(std::ptr::);
            let length = libvlc_media_tracks_get(self.0, track_array.as_mut_ptr_range().start);
            log::debug!("Track info elements: {}",length);
            for item in track_array {
                let raw = **item;
                let track = Track {
                    bitrate: raw.i_bitrate,
                    codec: raw.i_codec,
                    original_fourcc: raw.i_original_fourcc,
                    id: raw.i_id,
                    track_type: raw.i_type,
                    profile: raw.i_profile,
                    level: raw.i_level,
                    language: CString::from_raw(raw.psz_language),
                    description:  CString::from_raw(raw.psz_description),
                };
                tracks.push(track);
            }
        }
        tracks
    }
    */
}

impl From<MediaPointer> for Media {
    fn from(p: MediaPointer) -> Self {
        Media(p)
    }
}

impl Drop for Media {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                libvlc_media_release(self.0);
            }
        }
    }
}

impl PointerAccess<MediaPointer> for Media {
    fn get_ptr(&self) -> MediaPointer {
        self.0
    }
}
