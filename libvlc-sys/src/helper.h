#ifndef HELPER_H
#define HELPER_H
#include <vlc/vlc.h>
struct rvlc_track
{
    int filetype;
    const char *codec;
    int channels;
    int rate;
};

struct rvlc_tracklist
{
    struct rvlc_track **tracks;
    unsigned int length;
};

struct rvlc_tracklist *rvlc_alloc_tracklist(int size);
struct rvlc_track* rvlc_tracklist_get(void* list, int i);
int rvlc_tracklist_len(void* list);
void* rvlc_tracklist(libvlc_media_t *media);
void rvlc_tracklist_drop(void *list);

#endif