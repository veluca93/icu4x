#ifndef ICU4XCaseMapCloser_H
#define ICU4XCaseMapCloser_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XCodePointSetBuilder.d.h"
#include "ICU4XCodePointSetBuilder.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XCaseMapCloser_ICU4XDataError.d.h"

#include "ICU4XCaseMapCloser.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XCaseMapCloser_ICU4XDataError ICU4XCaseMapCloser_create(const ICU4XDataProvider* provider);

void ICU4XCaseMapCloser_add_case_closure_to(const ICU4XCaseMapCloser* self, char32_t c, ICU4XCodePointSetBuilder* builder);

bool ICU4XCaseMapCloser_add_string_case_closure_to(const ICU4XCaseMapCloser* self, const char* s_data, size_t s_len, ICU4XCodePointSetBuilder* builder);

void ICU4XCaseMapCloser_destroy(ICU4XCaseMapCloser* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XCaseMapCloser_H
