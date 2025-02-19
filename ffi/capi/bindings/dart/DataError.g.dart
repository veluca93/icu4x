// generated by diplomat-tool

part of 'lib.g.dart';

/// Additional information: [1](https://docs.rs/icu/latest/icu/provider/struct.DataError.html), [2](https://docs.rs/icu/latest/icu/provider/enum.DataErrorKind.html)
enum DataError {
  unknown,

  missingDataMarker,

  missingVariant,

  missingLocale,

  needsVariant,

  needsLocale,

  extraneousLocale,

  filteredResource,

  mismatchedType,

  missingPayload,

  invalidState,

  custom,

  io,

  unavailableBufferFormat,

  mismatchedAnyBuffer,

  dataStructValidityError;
}
