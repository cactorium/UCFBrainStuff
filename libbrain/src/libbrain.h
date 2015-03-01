#pragma once
#ifndef LIBBRAIN_H
#define LIBBRAIN_H

/** A higher-level abstraction of emokit.
 * This library provides a wrapper that can be easily extended to different
 * EEG data sources while providing a homogenous method of accessing the
 * data from those sources. This should all be thread safe.
 */

typedef unsigned long eeg_time_t;

/* A single data point. */
typedef float eeg_val;

/* A set of EEG channels. */
struct eeg_mask_t {
    unsigned char mask[3];
};

/* A single frame of EEG data. It is unannotated, and thus the EEG electrodes
 * the data corresponds to is unknown. 
 */
struct eeg_frame_t {
    struct eeg_val_t* vals;
    eeg_time_t ts;
    unsigned int nvals;
};

/* An annotated frame of EEG data. The data is stored in the same order as the
 * mask lists them.
 */
struct eeg_frame_m_t {
    struct eeg_frame_t frame;
    struct eeg_mask_t mask;
};

/* A homogenous store of EEG frame data. Since all the data is ordered
 * identically, the channel mask is stored once for the entire dataset.
 */
struct eeg_buffer_t {
    struct eeg_frame_t* frames;
    int len;
    struct eeg_mask_t mask;
};

typedef int eeg_err_t;

/* EEG error values */
#define EEG_UNIMPL      -1
#define EEG_OK          0

/* Struct that all stream generators need to include. See emotiv.c for how
 * this is used with the Emotiv EPOC headset.
 */
struct eeg_stream_t {
    struct eeg_stream_impl* impl;
};


/* This is a struct that basically acts as a V-table for EEG data streams.
 * This abstraction is in place so that multiple EEG devices can be supported
 * using the same set of functions. */
struct eeg_stream_impl {
    /* Destroy a stream. */
    eeg_err_t (*destroy)(struct eeg_stream_t* self);

    /* Start collecting data from the EEG helmet. */
    eeg_err_t (*try_connect)(struct eeg_stream_t* self);
    /* Suspend data collection. */
    eeg_err_t (*disconnect)(struct eeg_stream_t *self);

    /* Provide a list of channels this stream can provide. */
    const struct eeg_mask_t* (*get_cap)(struct eeg_stream_t *self);

    /* Pick which channels this stream will provide. */
    eeg_err_t (*mask_channels)(struct eeg_stream_t *self,
            const struct eeg_mask_t* cm);
    /* Provide active channels. */
    const struct eeg_mask_t* (*get_active_channels)(struct eeg_stream_t *self);
    /* Hide all channels. */
    eeg_err_t (*unmask_all_channels)(struct eeg_stream_t *self);
    /* Show all channels. */
    eeg_err_t (*mask_all_channels)(struct eeg_stream_t *self);

    /* TODO: Add in stream reading/zero copy/signalling functions. */
};

/* Polymorphic wrappers around all streams! See eeg_stream_impl for API details.
 */
eeg_err_t eeg_stream_destroy(struct eeg_stream_t* s);
eeg_err_t eeg_try_connect(struct eeg_stream_t* self);
eeg_err_t eeg_disconnect(struct eeg_stream_t *self);
const struct eeg_mask_t* eeg_get_cap(struct eeg_stream_t *self);
eeg_err_t eeg_mask_channels(struct eeg_stream_t *self, 
        const struct eeg_mask_t* cm);
const struct eeg_mask_t* eeg_get_active_channels(struct eeg_stream_t *self);
eeg_err_t eeg_unmask_all_channels(struct eeg_stream_t *self);
eeg_err_t eeg_mask_all_channels(struct eeg_stream_t *self);

#endif
