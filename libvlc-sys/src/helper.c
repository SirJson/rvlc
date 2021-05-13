
#include "helper.h"
#include <stdlib.h>

struct rvlc_tracklist *rvlc_alloc_tracklist(int size) {
  struct rvlc_tracklist *out =
      (struct rvlc_tracklist *)calloc(1, sizeof(struct rvlc_tracklist));
  out->tracks = (struct rvlc_track **)calloc(size, sizeof(struct rvlc_track));
}

struct rvlc_track *rvlc_tracklist_get(void *list, int i) {
  if (list == NULL)
    return NULL;

  struct rvlc_tracklist *output = (struct rvlc_tracklist *)list;
  return output->tracks[i];
}

int rvlc_tracklist_len(void *list) {
  if (list == NULL)
    return -1;

  struct rvlc_tracklist *output = (struct rvlc_tracklist *)list;
  return output->length;
}

void *rvlc_tracklist(libvlc_media_t *media) {
  libvlc_media_track_t **tracks;
  unsigned int tracksCount;
  tracksCount = libvlc_media_tracks_get(media, &tracks);
  struct rvlc_tracklist *output = NULL;
  if (tracksCount > 0) {
    output = rvlc_alloc_tracklist(tracksCount);
    output->length = tracksCount;
    for (unsigned i = 0; i < tracksCount; i++) {
      libvlc_media_track_t *track = tracks[i];
      struct rvlc_track *safetrack =
          (struct rvlc_track *)malloc(sizeof(struct rvlc_track));
      safetrack->channels = 0;
      safetrack->rate = 0;
      safetrack->filetype = track->i_type;
      safetrack->codec =
          libvlc_media_get_codec_description(track->i_type, track->i_codec);
      if (track->i_type == libvlc_track_audio) {
        libvlc_audio_track_t *audioTrack = track->audio;
        safetrack->channels = audioTrack->i_channels;
        safetrack->rate = audioTrack->i_rate;
      }

      output->tracks[i] = safetrack;
    }
    libvlc_media_tracks_release(tracks, tracksCount);
  }
  return (void *)output;
}

void rvlc_tracklist_drop(void *list) {
  if (list == NULL)
    return;

  struct rvlc_tracklist *output = (struct rvlc_tracklist *)list;
  for (unsigned int i = 0; i < output->length; i++) {
    struct rvlc_track *t = (struct rvlc_track *)output->tracks[i];
    free(t);
    output->tracks[i] = NULL;
  }
  free(output);
  output = NULL;
}