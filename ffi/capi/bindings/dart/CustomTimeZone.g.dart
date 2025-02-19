// generated by diplomat-tool

part of 'lib.g.dart';

/// See the [Rust documentation for `CustomTimeZone`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html) for more information.
final class CustomTimeZone implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _ffi;

  // These are "used" in the sense that they keep dependencies alive
  // ignore: unused_field
  final core.List<Object> _selfEdge;

  // This takes in a list of lifetime edges (including for &self borrows)
  // corresponding to data this may borrow from. These should be flat arrays containing
  // references to objects, and this object will hold on to them to keep them alive and
  // maintain borrow validity.
  CustomTimeZone._fromFfi(this._ffi, this._selfEdge) {
    if (_selfEdge.isEmpty) {
      _finalizer.attach(this, _ffi.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ICU4XCustomTimeZone_destroy));

  /// Creates a time zone from an offset string.
  ///
  /// See the [Rust documentation for `from_str`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.from_str) for more information.
  ///
  /// Throws [TimeZoneInvalidOffsetError] on failure.
  factory CustomTimeZone.fromString(String s) {
    final temp = ffi2.Arena();
    final sView = s.utf8View;
    final result = _ICU4XCustomTimeZone_create_from_string(sView.allocIn(temp), sView.length);
    temp.releaseAll();
    if (!result.isOk) {
      throw TimeZoneInvalidOffsetError.values[result.union.err];
    }
    return CustomTimeZone._fromFfi(result.union.ok, []);
  }

  /// Creates a time zone with no information.
  ///
  /// See the [Rust documentation for `new_empty`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.new_empty) for more information.
  factory CustomTimeZone.empty() {
    final result = _ICU4XCustomTimeZone_create_empty();
    return CustomTimeZone._fromFfi(result, []);
  }

  /// Creates a time zone for UTC.
  ///
  /// See the [Rust documentation for `utc`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.utc) for more information.
  factory CustomTimeZone.utc() {
    final result = _ICU4XCustomTimeZone_create_utc();
    return CustomTimeZone._fromFfi(result, []);
  }

  /// Creates a time zone for GMT (London winter time).
  ///
  /// See the [Rust documentation for `gmt`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.gmt) for more information.
  factory CustomTimeZone.gmt() {
    final result = _ICU4XCustomTimeZone_create_gmt();
    return CustomTimeZone._fromFfi(result, []);
  }

  /// Creates a time zone for BST (London summer time).
  ///
  /// See the [Rust documentation for `bst`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.bst) for more information.
  factory CustomTimeZone.bst() {
    final result = _ICU4XCustomTimeZone_create_bst();
    return CustomTimeZone._fromFfi(result, []);
  }

  /// Sets the `gmt_offset` field from offset seconds.
  ///
  /// Errors if the offset seconds are out of range.
  ///
  /// See the [Rust documentation for `try_from_offset_seconds`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.try_from_offset_seconds) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html)
  ///
  /// Throws [TimeZoneInvalidOffsetError] on failure.
  void trySetGmtOffsetSeconds(int offsetSeconds) {
    final result = _ICU4XCustomTimeZone_try_set_gmt_offset_seconds(_ffi, offsetSeconds);
    if (!result.isOk) {
      throw TimeZoneInvalidOffsetError.values[result.union.err];
    }
    
  }

  /// Clears the `gmt_offset` field.
  ///
  /// See the [Rust documentation for `offset_seconds`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.offset_seconds) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html)
  void clearGmtOffset() {
    _ICU4XCustomTimeZone_clear_gmt_offset(_ffi);
  }

  /// Returns the value of the `gmt_offset` field as offset seconds.
  ///
  /// Returns null if the `gmt_offset` field is empty.
  ///
  /// See the [Rust documentation for `offset_seconds`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.offset_seconds) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html)
  int? get gmtOffsetSeconds {
    final result = _ICU4XCustomTimeZone_gmt_offset_seconds(_ffi);
    if (!result.isOk) {
      return null;
    }
    return result.union.ok;
  }

  /// Returns whether the `gmt_offset` field is positive.
  ///
  /// Returns null if the `gmt_offset` field is empty.
  ///
  /// See the [Rust documentation for `is_positive`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.is_positive) for more information.
  bool? get isGmtOffsetPositive {
    final result = _ICU4XCustomTimeZone_is_gmt_offset_positive(_ffi);
    if (!result.isOk) {
      return null;
    }
    return result.union.ok;
  }

  /// Returns whether the `gmt_offset` field is zero.
  ///
  /// Returns null if the `gmt_offset` field is empty (which is not the same as zero).
  ///
  /// See the [Rust documentation for `is_zero`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.is_zero) for more information.
  bool? get isGmtOffsetZero {
    final result = _ICU4XCustomTimeZone_is_gmt_offset_zero(_ffi);
    if (!result.isOk) {
      return null;
    }
    return result.union.ok;
  }

  /// Returns whether the `gmt_offset` field has nonzero minutes.
  ///
  /// Returns null if the `gmt_offset` field is empty.
  ///
  /// See the [Rust documentation for `has_minutes`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.has_minutes) for more information.
  bool? get gmtOffsetHasMinutes {
    final result = _ICU4XCustomTimeZone_gmt_offset_has_minutes(_ffi);
    if (!result.isOk) {
      return null;
    }
    return result.union.ok;
  }

  /// Returns whether the `gmt_offset` field has nonzero seconds.
  ///
  /// Returns null if the `gmt_offset` field is empty.
  ///
  /// See the [Rust documentation for `has_seconds`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.has_seconds) for more information.
  bool? get gmtOffsetHasSeconds {
    final result = _ICU4XCustomTimeZone_gmt_offset_has_seconds(_ffi);
    if (!result.isOk) {
      return null;
    }
    return result.union.ok;
  }

  /// Sets the `time_zone_id` field from a BCP-47 string.
  ///
  /// Errors if the string is not a valid BCP-47 time zone ID.
  ///
  /// See the [Rust documentation for `time_zone_id`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.TimeZoneBcp47Id.html)
  ///
  /// Throws [TimeZoneInvalidIdError] on failure.
  void trySetTimeZoneId(String id) {
    final temp = ffi2.Arena();
    final idView = id.utf8View;
    final result = _ICU4XCustomTimeZone_try_set_time_zone_id(_ffi, idView.allocIn(temp), idView.length);
    temp.releaseAll();
    if (!result.isOk) {
      throw TimeZoneInvalidIdError.values[result.union.err];
    }
    
  }

  /// Sets the `time_zone_id` field from an IANA string by looking up
  /// the corresponding BCP-47 string.
  ///
  /// Errors if the string is not a valid BCP-47 time zone ID.
  ///
  /// Throws [TimeZoneInvalidIdError] on failure.
  void trySetIanaTimeZoneId(TimeZoneIdMapper mapper, String id) {
    final temp = ffi2.Arena();
    final idView = id.utf8View;
    final result = _ICU4XCustomTimeZone_try_set_iana_time_zone_id(_ffi, mapper._ffi, idView.allocIn(temp), idView.length);
    temp.releaseAll();
    if (!result.isOk) {
      throw TimeZoneInvalidIdError.values[result.union.err];
    }
    
  }

  /// Clears the `time_zone_id` field.
  ///
  /// See the [Rust documentation for `time_zone_id`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.TimeZoneBcp47Id.html)
  void clearTimeZoneId() {
    _ICU4XCustomTimeZone_clear_time_zone_id(_ffi);
  }

  /// Writes the value of the `time_zone_id` field as a string.
  ///
  /// Returns null if the `time_zone_id` field is empty.
  ///
  /// See the [Rust documentation for `time_zone_id`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.TimeZoneBcp47Id.html)
  String? get timeZoneId {
    final write = _Write();
    final result = _ICU4XCustomTimeZone_time_zone_id(_ffi, write._ffi);
    if (!result.isOk) {
      return null;
    }
    return write.finalize();
  }

  /// Sets the `metazone_id` field from a string.
  ///
  /// Returns null if the string is not a valid BCP-47 metazone ID.
  ///
  /// See the [Rust documentation for `metazone_id`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.metazone_id) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.MetazoneId.html)
  ///
  /// Throws [TimeZoneInvalidIdError] on failure.
  void trySetMetazoneId(String id) {
    final temp = ffi2.Arena();
    final idView = id.utf8View;
    final result = _ICU4XCustomTimeZone_try_set_metazone_id(_ffi, idView.allocIn(temp), idView.length);
    temp.releaseAll();
    if (!result.isOk) {
      throw TimeZoneInvalidIdError.values[result.union.err];
    }
    
  }

  /// Clears the `metazone_id` field.
  ///
  /// See the [Rust documentation for `metazone_id`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.metazone_id) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.MetazoneId.html)
  void clearMetazoneId() {
    _ICU4XCustomTimeZone_clear_metazone_id(_ffi);
  }

  /// Writes the value of the `metazone_id` field as a string.
  ///
  /// Returns null if the `metazone_id` field is empty.
  ///
  /// See the [Rust documentation for `metazone_id`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.metazone_id) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.MetazoneId.html)
  String? get metazoneId {
    final write = _Write();
    final result = _ICU4XCustomTimeZone_metazone_id(_ffi, write._ffi);
    if (!result.isOk) {
      return null;
    }
    return write.finalize();
  }

  /// Sets the `zone_variant` field from a string.
  ///
  /// Returns null if the string is not a valid zone variant.
  ///
  /// See the [Rust documentation for `zone_variant`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html)
  bool trySetZoneVariant(String id) {
    final temp = ffi2.Arena();
    final idView = id.utf8View;
    final result = _ICU4XCustomTimeZone_try_set_zone_variant(_ffi, idView.allocIn(temp), idView.length);
    temp.releaseAll();
    return result.isOk;
  }

  /// Clears the `zone_variant` field.
  ///
  /// See the [Rust documentation for `zone_variant`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html)
  void clearZoneVariant() {
    _ICU4XCustomTimeZone_clear_zone_variant(_ffi);
  }

  /// Writes the value of the `zone_variant` field as a string.
  ///
  /// Returns null if the `zone_variant` field is empty.
  ///
  /// See the [Rust documentation for `zone_variant`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html)
  String? get zoneVariant {
    final write = _Write();
    final result = _ICU4XCustomTimeZone_zone_variant(_ffi, write._ffi);
    if (!result.isOk) {
      return null;
    }
    return write.finalize();
  }

  /// Sets the `zone_variant` field to "standard" time, which may or may
  /// not correspond to a display name with "Standard" in its name.
  ///
  /// See the [Rust documentation for `standard`](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html#method.standard) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant)
  void setStandardTime() {
    _ICU4XCustomTimeZone_set_standard_time(_ffi);
  }

  /// Sets the `zone_variant` field to "daylight" time, which may or may
  /// not correspond to a display name with "Daylight" in its name.
  ///
  /// See the [Rust documentation for `daylight`](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html#method.daylight) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant)
  void setDaylightTime() {
    _ICU4XCustomTimeZone_set_daylight_time(_ffi);
  }

  /// Returns whether the `zone_variant` field is standard time.
  ///
  /// Returns null if the `zone_variant` field is empty.
  ///
  /// See the [Rust documentation for `standard`](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html#method.standard) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant)
  bool? get isStandardTime {
    final result = _ICU4XCustomTimeZone_is_standard_time(_ffi);
    if (!result.isOk) {
      return null;
    }
    return result.union.ok;
  }

  /// Returns whether the `zone_variant` field is daylight time.
  ///
  /// Returns null if the `zone_variant` field is empty.
  ///
  /// See the [Rust documentation for `daylight`](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html#method.daylight) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant)
  bool? get isDaylightTime {
    final result = _ICU4XCustomTimeZone_is_daylight_time(_ffi);
    if (!result.isOk) {
      return null;
    }
    return result.union.ok;
  }

  /// Sets the metazone based on the time zone and the local timestamp.
  ///
  /// See the [Rust documentation for `maybe_calculate_metazone`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.maybe_calculate_metazone) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.MetazoneCalculator.html#method.compute_metazone_from_time_zone)
  void maybeCalculateMetazone(MetazoneCalculator metazoneCalculator, IsoDateTime localDatetime) {
    _ICU4XCustomTimeZone_maybe_calculate_metazone(_ffi, metazoneCalculator._ffi, localDatetime._ffi);
  }
}

@meta.ResourceIdentifier('ICU4XCustomTimeZone_destroy')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_destroy')
// ignore: non_constant_identifier_names
external void _ICU4XCustomTimeZone_destroy(ffi.Pointer<ffi.Void> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_create_from_string')
@ffi.Native<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_create_from_string')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ICU4XCustomTimeZone_create_from_string(ffi.Pointer<ffi.Uint8> sData, int sLength);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_create_empty')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_create_empty')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XCustomTimeZone_create_empty();

@meta.ResourceIdentifier('ICU4XCustomTimeZone_create_utc')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_create_utc')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XCustomTimeZone_create_utc();

@meta.ResourceIdentifier('ICU4XCustomTimeZone_create_gmt')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_create_gmt')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XCustomTimeZone_create_gmt();

@meta.ResourceIdentifier('ICU4XCustomTimeZone_create_bst')
@ffi.Native<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_create_bst')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _ICU4XCustomTimeZone_create_bst();

@meta.ResourceIdentifier('ICU4XCustomTimeZone_try_set_gmt_offset_seconds')
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_try_set_gmt_offset_seconds')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XCustomTimeZone_try_set_gmt_offset_seconds(ffi.Pointer<ffi.Opaque> self, int offsetSeconds);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_clear_gmt_offset')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_clear_gmt_offset')
// ignore: non_constant_identifier_names
external void _ICU4XCustomTimeZone_clear_gmt_offset(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_gmt_offset_seconds')
@ffi.Native<_ResultInt32Void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_gmt_offset_seconds')
// ignore: non_constant_identifier_names
external _ResultInt32Void _ICU4XCustomTimeZone_gmt_offset_seconds(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_is_gmt_offset_positive')
@ffi.Native<_ResultBoolVoid Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_is_gmt_offset_positive')
// ignore: non_constant_identifier_names
external _ResultBoolVoid _ICU4XCustomTimeZone_is_gmt_offset_positive(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_is_gmt_offset_zero')
@ffi.Native<_ResultBoolVoid Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_is_gmt_offset_zero')
// ignore: non_constant_identifier_names
external _ResultBoolVoid _ICU4XCustomTimeZone_is_gmt_offset_zero(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_gmt_offset_has_minutes')
@ffi.Native<_ResultBoolVoid Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_gmt_offset_has_minutes')
// ignore: non_constant_identifier_names
external _ResultBoolVoid _ICU4XCustomTimeZone_gmt_offset_has_minutes(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_gmt_offset_has_seconds')
@ffi.Native<_ResultBoolVoid Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_gmt_offset_has_seconds')
// ignore: non_constant_identifier_names
external _ResultBoolVoid _ICU4XCustomTimeZone_gmt_offset_has_seconds(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_try_set_time_zone_id')
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_try_set_time_zone_id')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XCustomTimeZone_try_set_time_zone_id(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint8> idData, int idLength);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_try_set_iana_time_zone_id')
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_try_set_iana_time_zone_id')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XCustomTimeZone_try_set_iana_time_zone_id(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> mapper, ffi.Pointer<ffi.Uint8> idData, int idLength);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_clear_time_zone_id')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_clear_time_zone_id')
// ignore: non_constant_identifier_names
external void _ICU4XCustomTimeZone_clear_time_zone_id(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_time_zone_id')
@ffi.Native<_ResultVoidVoid Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_time_zone_id')
// ignore: non_constant_identifier_names
external _ResultVoidVoid _ICU4XCustomTimeZone_time_zone_id(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> write);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_try_set_metazone_id')
@ffi.Native<_ResultVoidInt32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_try_set_metazone_id')
// ignore: non_constant_identifier_names
external _ResultVoidInt32 _ICU4XCustomTimeZone_try_set_metazone_id(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint8> idData, int idLength);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_clear_metazone_id')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_clear_metazone_id')
// ignore: non_constant_identifier_names
external void _ICU4XCustomTimeZone_clear_metazone_id(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_metazone_id')
@ffi.Native<_ResultVoidVoid Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_metazone_id')
// ignore: non_constant_identifier_names
external _ResultVoidVoid _ICU4XCustomTimeZone_metazone_id(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> write);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_try_set_zone_variant')
@ffi.Native<_ResultVoidVoid Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_try_set_zone_variant')
// ignore: non_constant_identifier_names
external _ResultVoidVoid _ICU4XCustomTimeZone_try_set_zone_variant(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Uint8> idData, int idLength);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_clear_zone_variant')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_clear_zone_variant')
// ignore: non_constant_identifier_names
external void _ICU4XCustomTimeZone_clear_zone_variant(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_zone_variant')
@ffi.Native<_ResultVoidVoid Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_zone_variant')
// ignore: non_constant_identifier_names
external _ResultVoidVoid _ICU4XCustomTimeZone_zone_variant(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> write);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_set_standard_time')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_set_standard_time')
// ignore: non_constant_identifier_names
external void _ICU4XCustomTimeZone_set_standard_time(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_set_daylight_time')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_set_daylight_time')
// ignore: non_constant_identifier_names
external void _ICU4XCustomTimeZone_set_daylight_time(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_is_standard_time')
@ffi.Native<_ResultBoolVoid Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_is_standard_time')
// ignore: non_constant_identifier_names
external _ResultBoolVoid _ICU4XCustomTimeZone_is_standard_time(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_is_daylight_time')
@ffi.Native<_ResultBoolVoid Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_is_daylight_time')
// ignore: non_constant_identifier_names
external _ResultBoolVoid _ICU4XCustomTimeZone_is_daylight_time(ffi.Pointer<ffi.Opaque> self);

@meta.ResourceIdentifier('ICU4XCustomTimeZone_maybe_calculate_metazone')
@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'ICU4XCustomTimeZone_maybe_calculate_metazone')
// ignore: non_constant_identifier_names
external void _ICU4XCustomTimeZone_maybe_calculate_metazone(ffi.Pointer<ffi.Opaque> self, ffi.Pointer<ffi.Opaque> metazoneCalculator, ffi.Pointer<ffi.Opaque> localDatetime);
