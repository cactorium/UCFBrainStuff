#pragma once
#ifndef EMOTIV_H
#define EMOTIV_H
/** Provides a EEG datastream for Emotiv EPOC headsets.
 */
#include "emokit/emokit.h"

#include "libbrain.h"

// Creates an EEG stream for an Emotiv headset using the emokit library.
eeg_err_t eeg_create_emotiv_stream(eeg_stream_t *dst);

#endif
