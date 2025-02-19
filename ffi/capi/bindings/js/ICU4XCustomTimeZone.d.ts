import { i32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XIsoDateTime } from "./ICU4XIsoDateTime";
import { ICU4XMetazoneCalculator } from "./ICU4XMetazoneCalculator";
import { ICU4XTimeZoneIdMapper } from "./ICU4XTimeZoneIdMapper";
import { ICU4XTimeZoneInvalidIdError } from "./ICU4XTimeZoneInvalidIdError";
import { ICU4XTimeZoneInvalidOffsetError } from "./ICU4XTimeZoneInvalidOffsetError";

/**

 * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html Rust documentation for `CustomTimeZone`} for more information.
 */
export class ICU4XCustomTimeZone {

  /**

   * Creates a time zone from an offset string.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.from_str Rust documentation for `from_str`} for more information.
   * @throws {@link FFIError}<{@link ICU4XTimeZoneInvalidOffsetError}>
   */
  static create_from_string(s: string): ICU4XCustomTimeZone | never;

  /**

   * Creates a time zone with no information.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.new_empty Rust documentation for `new_empty`} for more information.
   */
  static create_empty(): ICU4XCustomTimeZone;

  /**

   * Creates a time zone for UTC.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.utc Rust documentation for `utc`} for more information.
   */
  static create_utc(): ICU4XCustomTimeZone;

  /**

   * Creates a time zone for GMT (London winter time).

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.gmt Rust documentation for `gmt`} for more information.
   */
  static create_gmt(): ICU4XCustomTimeZone;

  /**

   * Creates a time zone for BST (London summer time).

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.bst Rust documentation for `bst`} for more information.
   */
  static create_bst(): ICU4XCustomTimeZone;

  /**

   * Sets the `gmt_offset` field from offset seconds.

   * Errors if the offset seconds are out of range.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.try_from_offset_seconds Rust documentation for `try_from_offset_seconds`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html 1}
   * @throws {@link FFIError}<{@link ICU4XTimeZoneInvalidOffsetError}>
   */
  try_set_gmt_offset_seconds(offset_seconds: i32): void | never;

  /**

   * Clears the `gmt_offset` field.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.offset_seconds Rust documentation for `offset_seconds`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html 1}
   */
  clear_gmt_offset(): void;

  /**

   * Returns the value of the `gmt_offset` field as offset seconds.

   * Returns null if the `gmt_offset` field is empty.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.offset_seconds Rust documentation for `offset_seconds`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html 1}
   */
  gmt_offset_seconds(): i32 | undefined;

  /**

   * Returns whether the `gmt_offset` field is positive.

   * Returns null if the `gmt_offset` field is empty.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.is_positive Rust documentation for `is_positive`} for more information.
   */
  is_gmt_offset_positive(): boolean | undefined;

  /**

   * Returns whether the `gmt_offset` field is zero.

   * Returns null if the `gmt_offset` field is empty (which is not the same as zero).

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.is_zero Rust documentation for `is_zero`} for more information.
   */
  is_gmt_offset_zero(): boolean | undefined;

  /**

   * Returns whether the `gmt_offset` field has nonzero minutes.

   * Returns null if the `gmt_offset` field is empty.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.has_minutes Rust documentation for `has_minutes`} for more information.
   */
  gmt_offset_has_minutes(): boolean | undefined;

  /**

   * Returns whether the `gmt_offset` field has nonzero seconds.

   * Returns null if the `gmt_offset` field is empty.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.has_seconds Rust documentation for `has_seconds`} for more information.
   */
  gmt_offset_has_seconds(): boolean | undefined;

  /**

   * Sets the `time_zone_id` field from a BCP-47 string.

   * Errors if the string is not a valid BCP-47 time zone ID.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id Rust documentation for `time_zone_id`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.TimeZoneBcp47Id.html 1}
   * @throws {@link FFIError}<{@link ICU4XTimeZoneInvalidIdError}>
   */
  try_set_time_zone_id(id: string): void | never;

  /**

   * Sets the `time_zone_id` field from an IANA string by looking up the corresponding BCP-47 string.

   * Errors if the string is not a valid BCP-47 time zone ID.
   * @throws {@link FFIError}<{@link ICU4XTimeZoneInvalidIdError}>
   */
  try_set_iana_time_zone_id(mapper: ICU4XTimeZoneIdMapper, id: string): void | never;

  /**

   * Clears the `time_zone_id` field.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id Rust documentation for `time_zone_id`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.TimeZoneBcp47Id.html 1}
   */
  clear_time_zone_id(): void;

  /**

   * Writes the value of the `time_zone_id` field as a string.

   * Returns null if the `time_zone_id` field is empty.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id Rust documentation for `time_zone_id`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.TimeZoneBcp47Id.html 1}
   */
  time_zone_id(): string | undefined;

  /**

   * Sets the `metazone_id` field from a string.

   * Returns null if the string is not a valid BCP-47 metazone ID.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.metazone_id Rust documentation for `metazone_id`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.MetazoneId.html 1}
   * @throws {@link FFIError}<{@link ICU4XTimeZoneInvalidIdError}>
   */
  try_set_metazone_id(id: string): void | never;

  /**

   * Clears the `metazone_id` field.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.metazone_id Rust documentation for `metazone_id`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.MetazoneId.html 1}
   */
  clear_metazone_id(): void;

  /**

   * Writes the value of the `metazone_id` field as a string.

   * Returns null if the `metazone_id` field is empty.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.metazone_id Rust documentation for `metazone_id`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.MetazoneId.html 1}
   */
  metazone_id(): string | undefined;

  /**

   * Sets the `zone_variant` field from a string.

   * Returns null if the string is not a valid zone variant.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant Rust documentation for `zone_variant`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html 1}
   */
  try_set_zone_variant(id: string): void | undefined;

  /**

   * Clears the `zone_variant` field.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant Rust documentation for `zone_variant`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html 1}
   */
  clear_zone_variant(): void;

  /**

   * Writes the value of the `zone_variant` field as a string.

   * Returns null if the `zone_variant` field is empty.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant Rust documentation for `zone_variant`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html 1}
   */
  zone_variant(): string | undefined;

  /**

   * Sets the `zone_variant` field to "standard" time, which may or may not correspond to a display name with "Standard" in its name.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html#method.standard Rust documentation for `standard`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant 1}
   */
  set_standard_time(): void;

  /**

   * Sets the `zone_variant` field to "daylight" time, which may or may not correspond to a display name with "Daylight" in its name.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html#method.daylight Rust documentation for `daylight`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant 1}
   */
  set_daylight_time(): void;

  /**

   * Returns whether the `zone_variant` field is standard time.

   * Returns null if the `zone_variant` field is empty.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html#method.standard Rust documentation for `standard`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant 1}
   */
  is_standard_time(): boolean | undefined;

  /**

   * Returns whether the `zone_variant` field is daylight time.

   * Returns null if the `zone_variant` field is empty.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html#method.daylight Rust documentation for `daylight`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant 1}
   */
  is_daylight_time(): boolean | undefined;

  /**

   * Sets the metazone based on the time zone and the local timestamp.

   * See the {@link https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.maybe_calculate_metazone Rust documentation for `maybe_calculate_metazone`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/timezone/struct.MetazoneCalculator.html#method.compute_metazone_from_time_zone 1}
   */
  maybe_calculate_metazone(metazone_calculator: ICU4XMetazoneCalculator, local_datetime: ICU4XIsoDateTime): void;
}
