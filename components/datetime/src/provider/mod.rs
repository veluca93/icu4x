// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

pub mod calendar;
pub(crate) mod date_time;
pub mod time_zones;

/// Module for experimental new DateSymbols design
/// <https://github.com/unicode-org/icu4x/issues/3865>
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
//
//
// WHEN THIS GRADUATES; be sure to update the check for "neo" in baked_exporter!
#[cfg(any(feature = "datagen", feature = "experimental"))]
pub mod neo;

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub struct Baked;

#[cfg(feature = "compiled_data")]
#[allow(unused_imports)]
const _: () = {
    use icu_datetime_data::*;
    pub mod icu {
        pub use crate as datetime;
        pub use icu_datetime_data::icu_locale as locale;
    }
    make_provider!(Baked);
    impl_datetime_buddhist_datelengths_v1!(Baked);
    impl_datetime_buddhist_datesymbols_v1!(Baked);
    impl_datetime_chinese_datelengths_v1!(Baked);
    impl_datetime_chinese_datesymbols_v1!(Baked);
    impl_datetime_coptic_datelengths_v1!(Baked);
    impl_datetime_coptic_datesymbols_v1!(Baked);
    impl_datetime_dangi_datelengths_v1!(Baked);
    impl_datetime_dangi_datesymbols_v1!(Baked);
    impl_datetime_ethiopic_datelengths_v1!(Baked);
    impl_datetime_ethiopic_datesymbols_v1!(Baked);
    impl_datetime_gregory_datelengths_v1!(Baked);
    impl_datetime_gregory_datesymbols_v1!(Baked);
    impl_datetime_hebrew_datelengths_v1!(Baked);
    impl_datetime_hebrew_datesymbols_v1!(Baked);
    impl_datetime_indian_datelengths_v1!(Baked);
    impl_datetime_indian_datesymbols_v1!(Baked);
    impl_datetime_islamic_datelengths_v1!(Baked);
    impl_datetime_islamic_datesymbols_v1!(Baked);
    impl_datetime_japanese_datelengths_v1!(Baked);
    impl_datetime_japanese_datesymbols_v1!(Baked);
    impl_datetime_japanext_datelengths_v1!(Baked);
    impl_datetime_japanext_datesymbols_v1!(Baked);
    impl_datetime_persian_datelengths_v1!(Baked);
    impl_datetime_persian_datesymbols_v1!(Baked);
    impl_datetime_roc_datelengths_v1!(Baked);
    impl_datetime_roc_datesymbols_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_skeletons_v1!(Baked);
    impl_datetime_timelengths_v1!(Baked);
    impl_datetime_timesymbols_v1!(Baked);
    impl_time_zone_exemplar_cities_v1!(Baked);
    impl_time_zone_formats_v1!(Baked);
    impl_time_zone_generic_long_v1!(Baked);
    impl_time_zone_generic_short_v1!(Baked);
    impl_time_zone_specific_long_v1!(Baked);
    impl_time_zone_specific_short_v1!(Baked);

    #[cfg(feature = "experimental")]
    impl_datetime_symbols_weekdays_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_dayperiods_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_datetime_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_time_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_time_skeleton_v1!(Baked);

    #[cfg(feature = "experimental")]
    impl_datetime_patterns_buddhist_skeleton_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_chinese_skeleton_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_coptic_skeleton_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_dangi_skeleton_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_ethiopic_skeleton_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_gregory_skeleton_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_hebrew_skeleton_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_indian_skeleton_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_islamic_skeleton_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_japanese_skeleton_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_japanext_skeleton_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_persian_skeleton_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_roc_skeleton_v1!(Baked);

    #[cfg(feature = "experimental")]
    impl_datetime_symbols_buddhist_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_buddhist_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_chinese_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_chinese_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_coptic_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_coptic_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_dangi_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_dangi_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_ethiopic_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_ethiopic_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_gregory_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_gregory_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_hebrew_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_hebrew_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_indian_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_indian_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_islamic_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_islamic_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_japanese_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_japanese_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_japanext_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_japanext_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_persian_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_persian_years_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_roc_months_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_symbols_roc_years_v1!(Baked);

    #[cfg(feature = "experimental")]
    impl_datetime_patterns_buddhist_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_chinese_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_coptic_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_dangi_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_ethiopic_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_gregory_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_hebrew_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_indian_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_islamic_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_japanese_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_japanext_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_persian_date_v1!(Baked);
    #[cfg(feature = "experimental")]
    impl_datetime_patterns_roc_date_v1!(Baked);
};

#[cfg(feature = "datagen")]
use icu_provider::prelude::*;

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[
    calendar::BuddhistDateLengthsV1Marker::INFO,
    calendar::BuddhistDateSymbolsV1Marker::INFO,
    calendar::ChineseDateLengthsV1Marker::INFO,
    calendar::ChineseDateSymbolsV1Marker::INFO,
    calendar::CopticDateLengthsV1Marker::INFO,
    calendar::CopticDateSymbolsV1Marker::INFO,
    calendar::DangiDateLengthsV1Marker::INFO,
    calendar::DangiDateSymbolsV1Marker::INFO,
    calendar::EthiopianDateLengthsV1Marker::INFO,
    calendar::EthiopianDateSymbolsV1Marker::INFO,
    calendar::GregorianDateLengthsV1Marker::INFO,
    calendar::GregorianDateSymbolsV1Marker::INFO,
    calendar::HebrewDateLengthsV1Marker::INFO,
    calendar::HebrewDateSymbolsV1Marker::INFO,
    calendar::IndianDateLengthsV1Marker::INFO,
    calendar::IndianDateSymbolsV1Marker::INFO,
    calendar::IslamicDateLengthsV1Marker::INFO,
    calendar::IslamicDateSymbolsV1Marker::INFO,
    calendar::JapaneseDateLengthsV1Marker::INFO,
    calendar::JapaneseDateSymbolsV1Marker::INFO,
    calendar::JapaneseExtendedDateLengthsV1Marker::INFO,
    calendar::JapaneseExtendedDateSymbolsV1Marker::INFO,
    calendar::PersianDateLengthsV1Marker::INFO,
    calendar::PersianDateSymbolsV1Marker::INFO,
    calendar::RocDateLengthsV1Marker::INFO,
    calendar::RocDateSymbolsV1Marker::INFO,
    calendar::TimeLengthsV1Marker::INFO,
    calendar::TimeSymbolsV1Marker::INFO,
    time_zones::ExemplarCitiesV1Marker::INFO,
    time_zones::MetazoneGenericNamesLongV1Marker::INFO,
    time_zones::MetazoneGenericNamesShortV1Marker::INFO,
    time_zones::MetazoneSpecificNamesLongV1Marker::INFO,
    time_zones::MetazoneSpecificNamesShortV1Marker::INFO,
    time_zones::TimeZoneFormatsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    calendar::DateSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::WeekdayNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::DayPeriodNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::DateTimePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::TimePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::TimeNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::BuddhistYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::ChineseYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::CopticYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::DangiYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::EthiopianYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::GregorianYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::HebrewYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IndianYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IslamicYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseExtendedYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::PersianYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::RocYearNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::BuddhistMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::ChineseMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::CopticMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::DangiMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::EthiopianMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::GregorianMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::HebrewMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IndianMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IslamicMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseExtendedMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::PersianMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::RocMonthNamesV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::BuddhistDatePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::ChineseDatePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::CopticDatePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::DangiDatePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::EthiopianDatePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::GregorianDatePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::HebrewDatePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IndianDatePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IslamicDatePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseDatePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseExtendedDatePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::PersianDatePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::RocDatePatternV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::BuddhistDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::ChineseDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::CopticDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::DangiDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::EthiopianDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::GregorianDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::HebrewDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IndianDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::IslamicDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::JapaneseExtendedDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::PersianDateNeoSkeletonPatternsV1Marker::INFO,
    #[cfg(feature = "experimental")]
    neo::RocDateNeoSkeletonPatternsV1Marker::INFO,
];
