#ifndef ICU4XCollator_H
#define ICU4XCollator_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XCollatorOptionsV1.d.h"
#include "ICU4XCollatorOptionsV1.h"
#include "ICU4XCollatorResolvedOptionsV1.d.h"
#include "ICU4XCollatorResolvedOptionsV1.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "diplomat_result_box_ICU4XCollator_ICU4XDataError.d.h"

#include "ICU4XCollator.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XCollator_ICU4XDataError ICU4XCollator_create_v1(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XCollatorOptionsV1 options);

int8_t ICU4XCollator_compare_utf16_(const ICU4XCollator* self, const char16_t* left_data, size_t left_len, const char16_t* right_data, size_t right_len);

int8_t ICU4XCollator_compare_(const ICU4XCollator* self, const char* left_data, size_t left_len, const char* right_data, size_t right_len);

ICU4XCollatorResolvedOptionsV1 ICU4XCollator_resolved_options(const ICU4XCollator* self);

void ICU4XCollator_destroy(ICU4XCollator* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XCollator_H
