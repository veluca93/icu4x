#ifndef ICU4XLocale_H
#define ICU4XLocale_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "diplomat_result_box_ICU4XLocale_ICU4XLocaleParseError.d.h"
#include "diplomat_result_void_ICU4XLocaleParseError.d.h"
#include "diplomat_result_void_void.d.h"

#include "ICU4XLocale.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XLocale_ICU4XLocaleParseError ICU4XLocale_create_from_string(const char* name_data, size_t name_len);

ICU4XLocale* ICU4XLocale_create_und();

ICU4XLocale* ICU4XLocale_clone(const ICU4XLocale* self);

void ICU4XLocale_basename(const ICU4XLocale* self, DiplomatWrite* write);

diplomat_result_void_void ICU4XLocale_get_unicode_extension(const ICU4XLocale* self, const char* bytes_data, size_t bytes_len, DiplomatWrite* write);

void ICU4XLocale_language(const ICU4XLocale* self, DiplomatWrite* write);

diplomat_result_void_ICU4XLocaleParseError ICU4XLocale_set_language(ICU4XLocale* self, const char* bytes_data, size_t bytes_len);

diplomat_result_void_void ICU4XLocale_region(const ICU4XLocale* self, DiplomatWrite* write);

diplomat_result_void_ICU4XLocaleParseError ICU4XLocale_set_region(ICU4XLocale* self, const char* bytes_data, size_t bytes_len);

diplomat_result_void_void ICU4XLocale_script(const ICU4XLocale* self, DiplomatWrite* write);

diplomat_result_void_ICU4XLocaleParseError ICU4XLocale_set_script(ICU4XLocale* self, const char* bytes_data, size_t bytes_len);

diplomat_result_void_ICU4XLocaleParseError ICU4XLocale_canonicalize(const char* bytes_data, size_t bytes_len, DiplomatWrite* write);

void ICU4XLocale_to_string(const ICU4XLocale* self, DiplomatWrite* write);

bool ICU4XLocale_normalizing_eq(const ICU4XLocale* self, const char* other_data, size_t other_len);

int8_t ICU4XLocale_strict_cmp_(const ICU4XLocale* self, const char* other_data, size_t other_len);

int8_t ICU4XLocale_total_cmp_(const ICU4XLocale* self, const ICU4XLocale* other);

void ICU4XLocale_destroy(ICU4XLocale* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XLocale_H
