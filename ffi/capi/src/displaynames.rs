// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::{ICU4XDataError, ICU4XLocaleParseError};
    use crate::locale_core::ffi::ICU4XLocale;
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    #[allow(unused_imports)] // feature-specific
    use icu_experimental::displaynames::{DisplayNamesOptions, Fallback, LanguageDisplay};
    use icu_experimental::displaynames::{LocaleDisplayNamesFormatter, RegionDisplayNames};
    use icu_locale_core::subtags::Region;
    use writeable::Writeable;

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::displaynames::LocaleDisplayNamesFormatter, Struct)]
    pub struct ICU4XLocaleDisplayNamesFormatter(pub LocaleDisplayNamesFormatter);

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::displaynames::RegionDisplayNames, Struct)]
    pub struct ICU4XRegionDisplayNames(pub RegionDisplayNames);

    #[diplomat::rust_link(icu::displaynames::options::DisplayNamesOptions, Struct)]
    #[diplomat::attr(dart, rename = "DisplayNamesOptions")]
    pub struct ICU4XDisplayNamesOptionsV1 {
        /// The optional formatting style to use for display name.
        pub style: ICU4XDisplayNamesStyle,
        /// The fallback return when the system does not have the
        /// requested display name, defaults to "code".
        pub fallback: ICU4XDisplayNamesFallback,
        /// The language display kind, defaults to "dialect".
        pub language_display: ICU4XLanguageDisplay,
    }

    #[diplomat::rust_link(icu::displaynames::options::Style, Enum)]
    pub enum ICU4XDisplayNamesStyle {
        Auto,
        Narrow,
        Short,
        Long,
        Menu,
    }

    #[diplomat::rust_link(icu::displaynames::options::Fallback, Enum)]
    #[diplomat::enum_convert(Fallback, needs_wildcard)]
    pub enum ICU4XDisplayNamesFallback {
        Code,
        None,
    }

    #[diplomat::rust_link(icu::displaynames::options::LanguageDisplay, Enum)]
    #[diplomat::enum_convert(LanguageDisplay, needs_wildcard)]
    pub enum ICU4XLanguageDisplay {
        Dialect,
        Standard,
    }

    impl ICU4XLocaleDisplayNamesFormatter {
        /// Creates a new `LocaleDisplayNamesFormatter` from locale data and an options bag.
        #[diplomat::rust_link(icu::displaynames::LocaleDisplayNamesFormatter::try_new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            options: ICU4XDisplayNamesOptionsV1,
        ) -> Result<Box<ICU4XLocaleDisplayNamesFormatter>, ICU4XDataError> {
            let locale = locale.to_datalocale();
            let options = DisplayNamesOptions::from(options);

            Ok(Box::new(ICU4XLocaleDisplayNamesFormatter(
                call_constructor!(
                    LocaleDisplayNamesFormatter::try_new,
                    LocaleDisplayNamesFormatter::try_new_with_any_provider,
                    LocaleDisplayNamesFormatter::try_new_with_buffer_provider,
                    provider,
                    &locale,
                    options,
                )?,
            )))
        }

        /// Returns the locale-specific display name of a locale.
        #[diplomat::rust_link(icu::displaynames::LocaleDisplayNamesFormatter::of, FnInStruct)]
        pub fn of(&self, locale: &ICU4XLocale, write: &mut DiplomatWrite) {
            let _infallible = self.0.of(&locale.0).write_to(write);
        }
    }

    impl ICU4XRegionDisplayNames {
        /// Creates a new `RegionDisplayNames` from locale data and an options bag.
        #[diplomat::rust_link(icu::displaynames::RegionDisplayNames::try_new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
        ) -> Result<Box<ICU4XRegionDisplayNames>, ICU4XDataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ICU4XRegionDisplayNames(call_constructor!(
                RegionDisplayNames::try_new,
                RegionDisplayNames::try_new_with_any_provider,
                RegionDisplayNames::try_new_with_buffer_provider,
                provider,
                &locale,
                Default::default()
            )?)))
        }

        /// Returns the locale specific display name of a region.
        /// Note that the function returns an empty string in case the display name for a given
        /// region code is not found.
        #[diplomat::rust_link(icu::displaynames::RegionDisplayNames::of, FnInStruct)]
        pub fn of(
            &self,
            region: &DiplomatStr,
            write: &mut DiplomatWrite,
        ) -> Result<(), ICU4XLocaleParseError> {
            let _infallible = self
                .0
                .of(Region::try_from_bytes(region)?)
                .unwrap_or("")
                .write_to(write);
            Ok(())
        }
    }
}

#[allow(unused_imports)] // feature-specific
use icu_experimental::displaynames::{DisplayNamesOptions, Fallback, LanguageDisplay, Style};

impl From<ffi::ICU4XDisplayNamesStyle> for Option<Style> {
    fn from(style: ffi::ICU4XDisplayNamesStyle) -> Option<Style> {
        match style {
            ffi::ICU4XDisplayNamesStyle::Auto => None,
            ffi::ICU4XDisplayNamesStyle::Narrow => Some(Style::Narrow),
            ffi::ICU4XDisplayNamesStyle::Short => Some(Style::Short),
            ffi::ICU4XDisplayNamesStyle::Long => Some(Style::Long),
            ffi::ICU4XDisplayNamesStyle::Menu => Some(Style::Menu),
        }
    }
}

impl From<ffi::ICU4XDisplayNamesOptionsV1> for DisplayNamesOptions {
    fn from(other: ffi::ICU4XDisplayNamesOptionsV1) -> DisplayNamesOptions {
        let mut options = DisplayNamesOptions::default();
        options.style = other.style.into();
        options.fallback = other.fallback.into();
        options.language_display = other.language_display.into();
        options
    }
}
