#ifndef ICU4XUnicodeSetData_H
#define ICU4XUnicodeSetData_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "diplomat_result_box_ICU4XUnicodeSetData_ICU4XDataError.d.h"

#include "ICU4XUnicodeSetData.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


bool ICU4XUnicodeSetData_contains(const ICU4XUnicodeSetData* self, const char* s_data, size_t s_len);

bool ICU4XUnicodeSetData_contains_char(const ICU4XUnicodeSetData* self, char32_t cp);

bool ICU4XUnicodeSetData_contains32(const ICU4XUnicodeSetData* self, uint32_t cp);

diplomat_result_box_ICU4XUnicodeSetData_ICU4XDataError ICU4XUnicodeSetData_load_basic_emoji(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XUnicodeSetData_ICU4XDataError ICU4XUnicodeSetData_load_exemplars_main(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

diplomat_result_box_ICU4XUnicodeSetData_ICU4XDataError ICU4XUnicodeSetData_load_exemplars_auxiliary(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

diplomat_result_box_ICU4XUnicodeSetData_ICU4XDataError ICU4XUnicodeSetData_load_exemplars_punctuation(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

diplomat_result_box_ICU4XUnicodeSetData_ICU4XDataError ICU4XUnicodeSetData_load_exemplars_numbers(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

diplomat_result_box_ICU4XUnicodeSetData_ICU4XDataError ICU4XUnicodeSetData_load_exemplars_index(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

void ICU4XUnicodeSetData_destroy(ICU4XUnicodeSetData* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XUnicodeSetData_H
