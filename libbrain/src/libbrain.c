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


