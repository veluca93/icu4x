#ifndef ICU4XCodePointMapData8_H
#define ICU4XCodePointMapData8_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XCodePointRangeIterator.d.h"
#include "ICU4XCodePointRangeIterator.h"
#include "ICU4XCodePointSetData.d.h"
#include "ICU4XCodePointSetData.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XCodePointMapData8_ICU4XDataError.d.h"

#include "ICU4XCodePointMapData8.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


uint8_t ICU4XCodePointMapData8_get(const ICU4XCodePointMapData8* self, char32_t cp);

uint8_t ICU4XCodePointMapData8_get32(const ICU4XCodePointMapData8* self, uint32_t cp);

uint32_t ICU4XCodePointMapData8_general_category_to_mask(uint8_t gc);

ICU4XCodePointRangeIterator* ICU4XCodePointMapData8_iter_ranges_for_value(const ICU4XCodePointMapData8* self, uint8_t value);

ICU4XCodePointRangeIterator* ICU4XCodePointMapData8_iter_ranges_for_value_complemented(const ICU4XCodePointMapData8* self, uint8_t value);

ICU4XCodePointRangeIterator* ICU4XCodePointMapData8_iter_ranges_for_mask(const ICU4XCodePointMapData8* self, uint32_t mask);

ICU4XCodePointSetData* ICU4XCodePointMapData8_get_set_for_value(const ICU4XCodePointMapData8* self, uint8_t value);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XDataError ICU4XCodePointMapData8_load_general_category(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XDataError ICU4XCodePointMapData8_load_bidi_class(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XDataError ICU4XCodePointMapData8_load_east_asian_width(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XDataError ICU4XCodePointMapData8_load_hangul_syllable_type(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XDataError ICU4XCodePointMapData8_load_indic_syllabic_category(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XDataError ICU4XCodePointMapData8_load_line_break(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XDataError ICU4XCodePointMapData8_try_grapheme_cluster_break(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XDataError ICU4XCodePointMapData8_load_word_break(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XDataError ICU4XCodePointMapData8_load_sentence_break(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XDataError ICU4XCodePointMapData8_load_joining_type(const ICU4XDataProvider* provider);

void ICU4XCodePointMapData8_destroy(ICU4XCodePointMapData8* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XCodePointMapData8_H
