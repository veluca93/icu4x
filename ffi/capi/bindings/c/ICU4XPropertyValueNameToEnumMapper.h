#ifndef ICU4XPropertyValueNameToEnumMapper_H
#define ICU4XPropertyValueNameToEnumMapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XPropertyValueNameToEnumMapper_ICU4XDataError.d.h"

#include "ICU4XPropertyValueNameToEnumMapper.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


int16_t ICU4XPropertyValueNameToEnumMapper_get_strict(const ICU4XPropertyValueNameToEnumMapper* self, const char* name_data, size_t name_len);

int16_t ICU4XPropertyValueNameToEnumMapper_get_loose(const ICU4XPropertyValueNameToEnumMapper* self, const char* name_data, size_t name_len);

diplomat_result_box_ICU4XPropertyValueNameToEnumMapper_ICU4XDataError ICU4XPropertyValueNameToEnumMapper_load_general_category(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XPropertyValueNameToEnumMapper_ICU4XDataError ICU4XPropertyValueNameToEnumMapper_load_hangul_syllable_type(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XPropertyValueNameToEnumMapper_ICU4XDataError ICU4XPropertyValueNameToEnumMapper_load_east_asian_width(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XPropertyValueNameToEnumMapper_ICU4XDataError ICU4XPropertyValueNameToEnumMapper_load_bidi_class(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XPropertyValueNameToEnumMapper_ICU4XDataError ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XPropertyValueNameToEnumMapper_ICU4XDataError ICU4XPropertyValueNameToEnumMapper_load_line_break(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XPropertyValueNameToEnumMapper_ICU4XDataError ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XPropertyValueNameToEnumMapper_ICU4XDataError ICU4XPropertyValueNameToEnumMapper_load_word_break(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XPropertyValueNameToEnumMapper_ICU4XDataError ICU4XPropertyValueNameToEnumMapper_load_sentence_break(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XPropertyValueNameToEnumMapper_ICU4XDataError ICU4XPropertyValueNameToEnumMapper_load_script(const ICU4XDataProvider* provider);

void ICU4XPropertyValueNameToEnumMapper_destroy(ICU4XPropertyValueNameToEnumMapper* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XPropertyValueNameToEnumMapper_H
