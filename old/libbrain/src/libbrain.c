#include <assert.h>

#include "libbrain.h"

#define CHECK_IMPL_NOARGS(s, fname, expr) \
    do { \
        if (s && s -> impl) { \
            expr = s -> impl -> fname (s); \
        } \
    } while(0)


#define CHECK_IMPL(s, fname, expr, ...) \
    do { \
        if (s && s -> impl) { \
            expr = s -> impl -> fname (s, __VA_ARGS__); \
        } \
    } while(0)


/* Mask utility functions */
struct eeg_mask_t eeg_mask_create() 
{
    const struct eeg_mask_t ret = { .mask = {0, 0, 0} };
    return ret;
}

void eeg_mask_clone(const struct eeg_mask_t* src, struct eeg_mask_t* dst)
{
    int i;
    assert (src && dst);
    for (i = 0; i < 3; i++) dst -> mask[i] = src -> mask [i];
}

void eeg_mask_add_idx(struct eeg_mask_t* self, eeg_mask_idx_t idx)
{
    assert (self);
    self -> mask[idx >> 3] |= 0x01 << (idx & 7);
}

void eeg_mask_remove_idx(struct eeg_mask_t* self, eeg_mask_idx_t idx)
{
    assert (self);
    self -> mask[idx >> 3] &= 0xff ^ 0x01 << (idx & 7);
}

void eeg_mask_toggle_idx(struct eeg_mask_t* self, eeg_mask_idx_t idx)
{
    assert (self);
    self -> mask[idx >> 3] ^= 0x01 << (idx & 7);
}

struct eeg_mask_itr eeg_mask_itr(struct eeg_mask_t* self)
{
    assert (self);
    struct eeg_mask_itr ret = {
        .parent = self, 
        .cur_pos = 0
    };
    return ret;
}

eeg_mask_idx_t eeg_mask_itr_next(struct eeg_mask_itr* itr)
{
    unsigned char mask = 0x01;
    eeg_mask_idx_t ret = -1;

    assert (itr && (itr -> cur_pos >= 0));
    if (itr -> cur_pos >= EEG_MAX_CHANNELS) itr -> cur_pos = 0;
    while (itr -> cur_pos < EEG_MAX_CHANNELS && 
            !(itr -> parent -> mask[itr -> cur_pos >> 3] & mask))
    {
        itr -> cur_pos++;

        mask = mask << 1;
        if (!mask) mask = 0x01;
    }
    if (itr -> cur_pos < EEG_MAX_CHANNELS) ret = itr -> cur_pos;

    itr -> cur_pos = ret + 1;
    return ret;
}

int eeg_mask_has_next(struct eeg_mask_itr* itr)
{
    int i;
    unsigned char mask = 0x01;
    assert (itr && (itr -> cur_pos >= 0) && (itr -> cur_pos <= 17));
    for (i = itr -> cur_pos; i < EEG_MAX_CHANNELS; i++)
    {
        if (mask & (itr -> parent -> mask [i >> 3])) return 1;

        mask = mask << 1;
        if (!mask) mask = 0x01;
    }
    return 0;
}
const char* electrode_names[] = {"Fz", "FP1", "FP2","F3", "F4", "F7", "F8",
    "Cz", "C3", "C4", "A1", "A2", "T3", "T4", "T5", "T6", "O1", "O2"};

const char* eeg_mask_idx_string(eeg_mask_idx_t idx)
{
    assert (idx >= 0 && idx < EEG_MAX_CHANNELS);
    return electrode_names[idx];
}

/* Currently eeg_time_t just stores the time since start in microseconds */
double get_real_time(eeg_time_t t)
{
    return t/1000000.0;
}

double get_time_diff(eeg_time_t s, eeg_time_t e)
{
    return (e - s)/1000000.0;
}

eeg_time_t from_real_time(double t)
{
    return t * 1000000;
}

eeg_err_t eeg_stream_destroy(struct eeg_stream_t* s) 
{
    eeg_err_t ret = EEG_UNIMPL;
    CHECK_IMPL_NOARGS (s, destroy, ret);
    return ret;
}

eeg_err_t eeg_try_connect(struct eeg_stream_t* self)
{
    eeg_err_t ret = EEG_UNIMPL;
    CHECK_IMPL_NOARGS (self, try_connect, ret);
    return ret;
}

eeg_err_t eeg_disconnect(struct eeg_stream_t *self)
{
    eeg_err_t ret = EEG_UNIMPL;
    CHECK_IMPL_NOARGS (self, disconnect, ret);
    return ret;
}

const struct eeg_mask_t* eeg_get_cap(struct eeg_stream_t *self)
{
    const struct eeg_mask_t* ret = 0;
    CHECK_IMPL_NOARGS (self, get_cap, ret);
    return ret;
}

eeg_err_t eeg_mask_channels(struct eeg_stream_t *self, 
        const struct eeg_mask_t* cm)
{
    eeg_err_t ret = EEG_UNIMPL;
    CHECK_IMPL (self, mask_channels, ret, cm);
    return ret;
}

const struct eeg_mask_t* eeg_get_active_channels(struct eeg_stream_t *self)
{
    const struct eeg_mask_t* ret = 0;
    CHECK_IMPL_NOARGS (self, get_active_channels, ret);
    return ret;
}

eeg_err_t eeg_unmask_all_channels(struct eeg_stream_t *self) {
    eeg_err_t ret = EEG_UNIMPL;
    CHECK_IMPL_NOARGS (self, unmask_all_channels, ret);
    return ret;
}

eeg_err_t eeg_mask_all_channels(struct eeg_stream_t *self) {
    eeg_err_t ret = EEG_UNIMPL;
    CHECK_IMPL_NOARGS (self, mask_all_channels, ret);
    return ret;
}

eeg_err_t eeg_get_frame(struct eeg_stream_t* self, struct eeg_frame_m_t* f)
{
    eeg_err_t ret = EEG_UNIMPL;
    CHECK_IMPL (self, get_frame, ret, f);
    return ret;
}
eeg_err_t eeg_get_channel(struct eeg_stream_t* self,
        const struct eeg_mask_t* m, eeg_val_t* c, eeg_time_t* ts)
{
    eeg_err_t ret = EEG_UNIMPL;
    CHECK_IMPL (self, get_channel, ret, m, c, ts);
    return ret;
}
