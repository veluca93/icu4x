// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_timezone::CustomTimeZone;

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::ICU4XTimeZoneInvalidIdError;
    use crate::errors::ffi::ICU4XTimeZoneInvalidOffsetError;
    use alloc::boxed::Box;
    use core::fmt::Write;
    use icu_timezone::CustomTimeZone;
    use icu_timezone::GmtOffset;
    use icu_timezone::ZoneVariant;

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::timezone::CustomTimeZone, Struct)]
    pub struct ICU4XCustomTimeZone(pub CustomTimeZone);

    impl ICU4XCustomTimeZone {
        /// Creates a time zone from an offset string.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::from_str, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::try_from_bytes, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::timezone::GmtOffset::from_str, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::timezone::GmtOffset::try_from_bytes, FnInStruct, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "from_string")]
        pub fn create_from_string(
            s: &DiplomatStr,
        ) -> Result<Box<ICU4XCustomTimeZone>, ICU4XTimeZoneInvalidOffsetError> {
            Ok(Box::new(ICU4XCustomTimeZone::from(
                CustomTimeZone::try_from_bytes(s)?,
            )))
        }

        /// Creates a time zone with no information.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::new_empty, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "empty")]
        pub fn create_empty() -> Box<ICU4XCustomTimeZone> {
            Box::new(CustomTimeZone::new_empty().into())
        }

        /// Creates a time zone for UTC.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::utc, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::GmtOffset::utc, FnInStruct, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "utc")]
        pub fn create_utc() -> Box<ICU4XCustomTimeZone> {
            Box::new(CustomTimeZone::utc().into())
        }

        /// Creates a time zone for GMT (London winter time).
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::gmt, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "gmt")]
        pub fn create_gmt() -> Box<ICU4XCustomTimeZone> {
            Box::new(CustomTimeZone::gmt().into())
        }

        /// Creates a time zone for BST (London summer time).
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::bst, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "bst")]
        pub fn create_bst() -> Box<ICU4XCustomTimeZone> {
            Box::new(CustomTimeZone::bst().into())
        }

        /// Sets the `gmt_offset` field from offset seconds.
        ///
        /// Errors if the offset seconds are out of range.
        #[diplomat::rust_link(icu::timezone::GmtOffset::try_from_offset_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::GmtOffset, Struct, compact)]
        #[diplomat::rust_link(
            icu::timezone::GmtOffset::from_offset_seconds_unchecked,
            FnInStruct,
            hidden
        )]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::new_with_offset, FnInStruct, hidden)]
        pub fn try_set_gmt_offset_seconds(
            &mut self,
            offset_seconds: i32,
        ) -> Result<(), ICU4XTimeZoneInvalidOffsetError> {
            self.0.gmt_offset = Some(GmtOffset::try_from_offset_seconds(offset_seconds)?);
            Ok(())
        }

        /// Clears the `gmt_offset` field.
        #[diplomat::rust_link(icu::timezone::GmtOffset::offset_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::GmtOffset, Struct, compact)]
        pub fn clear_gmt_offset(&mut self) {
            self.0.gmt_offset.take();
        }

        /// Returns the value of the `gmt_offset` field as offset seconds.
        ///
        /// Returns null if the `gmt_offset` field is empty.
        #[diplomat::rust_link(icu::timezone::GmtOffset::offset_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::GmtOffset, Struct, compact)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn gmt_offset_seconds(&self) -> Option<i32> {
            self.0.gmt_offset.map(GmtOffset::offset_seconds)
        }

        /// Returns whether the `gmt_offset` field is positive.
        ///
        /// Returns null if the `gmt_offset` field is empty.
        #[diplomat::rust_link(icu::timezone::GmtOffset::is_positive, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn is_gmt_offset_positive(&self) -> Option<bool> {
            self.0.gmt_offset.map(GmtOffset::is_positive)
        }

        /// Returns whether the `gmt_offset` field is zero.
        ///
        /// Returns null if the `gmt_offset` field is empty (which is not the same as zero).
        #[diplomat::rust_link(icu::timezone::GmtOffset::is_zero, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn is_gmt_offset_zero(&self) -> Option<bool> {
            self.0.gmt_offset.map(GmtOffset::is_zero)
        }

        /// Returns whether the `gmt_offset` field has nonzero minutes.
        ///
        /// Returns null if the `gmt_offset` field is empty.
        #[diplomat::rust_link(icu::timezone::GmtOffset::has_minutes, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn gmt_offset_has_minutes(&self) -> Option<bool> {
            self.0.gmt_offset.map(GmtOffset::has_minutes)
        }

        /// Returns whether the `gmt_offset` field has nonzero seconds.
        ///
        /// Returns null if the `gmt_offset` field is empty.
        #[diplomat::rust_link(icu::timezone::GmtOffset::has_seconds, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn gmt_offset_has_seconds(&self) -> Option<bool> {
            self.0.gmt_offset.map(GmtOffset::has_seconds)
        }

        /// Sets the `time_zone_id` field from a BCP-47 string.
        ///
        /// Errors if the string is not a valid BCP-47 time zone ID.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::time_zone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id, Struct, compact)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id::from_str, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id::deref, FnInStruct, hidden)]
        #[diplomat::rust_link(
            icu::timezone::TimeZoneBcp47Id::Target,
            AssociatedTypeInStruct,
            hidden
        )]
        pub fn try_set_time_zone_id(
            &mut self,
            id: &DiplomatStr,
        ) -> Result<(), ICU4XTimeZoneInvalidIdError> {
            self.0.time_zone_id = Some(icu_timezone::TimeZoneBcp47Id(
                tinystr::TinyAsciiStr::try_from_utf8(id)
                    .map_err(|_| ICU4XTimeZoneInvalidIdError::TodoZst)?,
            ));
            Ok(())
        }

        /// Sets the `time_zone_id` field from an IANA string by looking up
        /// the corresponding BCP-47 string.
        ///
        /// Errors if the string is not a valid BCP-47 time zone ID.
        pub fn try_set_iana_time_zone_id(
            &mut self,
            mapper: &crate::timezone_mapper::ffi::ICU4XTimeZoneIdMapper,
            id: &DiplomatStr,
        ) -> Result<(), ICU4XTimeZoneInvalidIdError> {
            self.0.time_zone_id = Some(
                mapper
                    .0
                    .as_borrowed()
                    .iana_bytes_to_bcp47(id)
                    .ok_or(ICU4XTimeZoneInvalidIdError::TodoZst)?,
            );
            Ok(())
        }

        /// Clears the `time_zone_id` field.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::time_zone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id, Struct, compact)]
        pub fn clear_time_zone_id(&mut self) {
            self.0.time_zone_id.take();
        }

        /// Writes the value of the `time_zone_id` field as a string.
        ///
        /// Returns null if the `time_zone_id` field is empty.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::time_zone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id, Struct, compact)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn time_zone_id(&self, write: &mut diplomat_runtime::DiplomatWrite) -> Option<()> {
            let _infallible = write.write_str(self.0.time_zone_id?.0.as_str());
            Some(())
        }

        /// Sets the `metazone_id` field from a string.
        ///
        /// Returns null if the string is not a valid BCP-47 metazone ID.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::metazone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::MetazoneId, Struct, compact)]
        #[diplomat::rust_link(icu::timezone::MetazoneId::from_str, FnInStruct, hidden)]
        pub fn try_set_metazone_id(
            &mut self,
            id: &DiplomatStr,
        ) -> Result<(), ICU4XTimeZoneInvalidIdError> {
            self.0.metazone_id = Some(icu_timezone::MetazoneId(
                tinystr::TinyAsciiStr::try_from_utf8(id)
                    .map_err(|_| ICU4XTimeZoneInvalidIdError::TodoZst)?,
            ));
            Ok(())
        }

        /// Clears the `metazone_id` field.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::metazone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::MetazoneId, Struct, compact)]
        pub fn clear_metazone_id(&mut self) {
            self.0.metazone_id.take();
        }

        /// Writes the value of the `metazone_id` field as a string.
        ///
        /// Returns null if the `metazone_id` field is empty.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::metazone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::MetazoneId, Struct, compact)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn metazone_id(&self, write: &mut diplomat_runtime::DiplomatWrite) -> Option<()> {
            let _infallible = write.write_str(self.0.metazone_id?.0.as_str());
            Some(())
        }

        /// Sets the `zone_variant` field from a string.
        ///
        /// Returns null if the string is not a valid zone variant.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField)]
        #[diplomat::rust_link(icu::timezone::ZoneVariant, Struct, compact)]
        #[diplomat::rust_link(icu::timezone::ZoneVariant::from_str, FnInStruct, hidden)]
        pub fn try_set_zone_variant(&mut self, id: &DiplomatStr) -> Option<()> {
            self.0.zone_variant = Some(icu_timezone::ZoneVariant(
                tinystr::TinyAsciiStr::try_from_utf8(id).ok()?,
            ));
            Some(())
        }

        /// Clears the `zone_variant` field.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField)]
        #[diplomat::rust_link(icu::timezone::ZoneVariant, Struct, compact)]
        pub fn clear_zone_variant(&mut self) {
            self.0.zone_variant.take();
        }

        /// Writes the value of the `zone_variant` field as a string.
        ///
        /// Returns null if the `zone_variant` field is empty.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField)]
        #[diplomat::rust_link(icu::timezone::ZoneVariant, Struct, compact)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn zone_variant(&self, write: &mut diplomat_runtime::DiplomatWrite) -> Option<()> {
            let _infallible = write.write_str(self.0.zone_variant?.0.as_str());
            Some(())
        }

        /// Sets the `zone_variant` field to "standard" time, which may or may
        /// not correspond to a display name with "Standard" in its name.
        #[diplomat::rust_link(icu::timezone::ZoneVariant::standard, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField, compact)]
        pub fn set_standard_time(&mut self) {
            self.0.zone_variant = Some(ZoneVariant::standard())
        }

        /// Sets the `zone_variant` field to "daylight" time, which may or may
        /// not correspond to a display name with "Daylight" in its name.
        #[diplomat::rust_link(icu::timezone::ZoneVariant::daylight, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField, compact)]
        pub fn set_daylight_time(&mut self) {
            self.0.zone_variant = Some(ZoneVariant::daylight())
        }

        /// Returns whether the `zone_variant` field is standard time.
        ///
        /// Returns null if the `zone_variant` field is empty.
        #[diplomat::rust_link(icu::timezone::ZoneVariant::standard, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField, compact)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn is_standard_time(&self) -> Option<bool> {
            Some(self.0.zone_variant? == ZoneVariant::standard())
        }

        /// Returns whether the `zone_variant` field is daylight time.
        ///
        /// Returns null if the `zone_variant` field is empty.
        #[diplomat::rust_link(icu::timezone::ZoneVariant::daylight, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField, compact)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn is_daylight_time(&self) -> Option<bool> {
            Some(self.0.zone_variant? == ZoneVariant::daylight())
        }

        /// Sets the metazone based on the time zone and the local timestamp.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::maybe_calculate_metazone, FnInStruct)]
        #[diplomat::rust_link(
            icu::timezone::MetazoneCalculator::compute_metazone_from_time_zone,
            FnInStruct,
            compact
        )]
        #[cfg(feature = "icu_timezone")]
        pub fn maybe_calculate_metazone(
            &mut self,
            metazone_calculator: &crate::metazone_calculator::ffi::ICU4XMetazoneCalculator,
            local_datetime: &crate::datetime::ffi::ICU4XIsoDateTime,
        ) {
            self.0
                .maybe_calculate_metazone(&metazone_calculator.0, &local_datetime.0);
        }
    }
}

impl From<CustomTimeZone> for ffi::ICU4XCustomTimeZone {
    fn from(other: CustomTimeZone) -> Self {
        Self(other)
    }
}

impl From<ffi::ICU4XCustomTimeZone> for CustomTimeZone {
    fn from(other: ffi::ICU4XCustomTimeZone) -> Self {
        other.0
    }
}
