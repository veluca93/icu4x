// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::{errors::ffi::ICU4XDataError, provider::ffi::ICU4XDataProvider};
    use alloc::boxed::Box;
    use icu_normalizer::properties::{
        CanonicalCombiningClassMap, CanonicalComposition, CanonicalDecomposition, Decomposed,
    };

    /// Lookup of the Canonical_Combining_Class Unicode property
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::normalizer::properties::CanonicalCombiningClassMap, Struct)]
    pub struct ICU4XCanonicalCombiningClassMap(pub CanonicalCombiningClassMap);

    impl ICU4XCanonicalCombiningClassMap {
        /// Construct a new ICU4XCanonicalCombiningClassMap instance for NFC
        #[diplomat::rust_link(
            icu::normalizer::properties::CanonicalCombiningClassMap::new,
            FnInStruct
        )]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCanonicalCombiningClassMap>, ICU4XDataError> {
            Ok(Box::new(ICU4XCanonicalCombiningClassMap(
                call_constructor!(
                    CanonicalCombiningClassMap::new [r => Ok(r)],
                    CanonicalCombiningClassMap::try_new_with_any_provider,
                    CanonicalCombiningClassMap::try_new_with_buffer_provider,
                    provider
                )?,
            )))
        }

        #[diplomat::rust_link(
            icu::normalizer::properties::CanonicalCombiningClassMap::get,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::properties::properties::CanonicalCombiningClass,
            Struct,
            compact
        )]
        #[diplomat::attr(supports = indexing, indexer)]
        pub fn get(&self, ch: DiplomatChar) -> u8 {
            self.0.get32(ch).0
        }
        #[diplomat::rust_link(
            icu::normalizer::properties::CanonicalCombiningClassMap::get32,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::properties::properties::CanonicalCombiningClass,
            Struct,
            compact
        )]
        #[diplomat::attr(dart, disable)]
        pub fn get32(&self, ch: u32) -> u8 {
            self.0.get32(ch).0
        }
    }

    /// The raw canonical composition operation.
    ///
    /// Callers should generally use ICU4XComposingNormalizer unless they specifically need raw composition operations
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::normalizer::properties::CanonicalComposition, Struct)]
    pub struct ICU4XCanonicalComposition(pub CanonicalComposition);

    impl ICU4XCanonicalComposition {
        /// Construct a new ICU4XCanonicalComposition instance for NFC
        #[diplomat::rust_link(icu::normalizer::properties::CanonicalComposition::new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCanonicalComposition>, ICU4XDataError> {
            Ok(Box::new(ICU4XCanonicalComposition(call_constructor!(
                CanonicalComposition::new [r => Ok(r)],
                CanonicalComposition::try_new_with_any_provider,
                CanonicalComposition::try_new_with_buffer_provider,
                provider,
            )?)))
        }

        /// Performs canonical composition (including Hangul) on a pair of characters
        /// or returns NUL if these characters don’t compose. Composition exclusions are taken into account.
        #[diplomat::rust_link(
            icu::normalizer::properties::CanonicalComposition::compose,
            FnInStruct
        )]
        pub fn compose(&self, starter: DiplomatChar, second: DiplomatChar) -> DiplomatChar {
            match (char::from_u32(starter), char::from_u32(second)) {
                (Some(starter), Some(second)) => self.0.compose(starter, second),
                _ => None,
            }
            .unwrap_or('\0') as DiplomatChar
        }
    }

    /// The outcome of non-recursive canonical decomposition of a character.
    /// `second` will be NUL when the decomposition expands to a single character
    /// (which may or may not be the original one)
    #[diplomat::rust_link(icu::normalizer::properties::Decomposed, Enum)]
    #[diplomat::out]
    pub struct ICU4XDecomposed {
        first: DiplomatChar,
        second: DiplomatChar,
    }

    /// The raw (non-recursive) canonical decomposition operation.
    ///
    /// Callers should generally use ICU4XDecomposingNormalizer unless they specifically need raw composition operations
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::normalizer::properties::CanonicalDecomposition, Struct)]
    pub struct ICU4XCanonicalDecomposition(pub CanonicalDecomposition);

    impl ICU4XCanonicalDecomposition {
        /// Construct a new ICU4XCanonicalDecomposition instance for NFC
        #[diplomat::rust_link(icu::normalizer::properties::CanonicalDecomposition::new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCanonicalDecomposition>, ICU4XDataError> {
            Ok(Box::new(ICU4XCanonicalDecomposition(call_constructor!(
                CanonicalDecomposition::new [r => Ok(r)],
                CanonicalDecomposition::try_new_with_any_provider,
                CanonicalDecomposition::try_new_with_buffer_provider,
                provider,
            )?)))
        }

        /// Performs non-recursive canonical decomposition (including for Hangul).
        #[diplomat::rust_link(
            icu::normalizer::properties::CanonicalDecomposition::decompose,
            FnInStruct
        )]
        pub fn decompose(&self, c: DiplomatChar) -> ICU4XDecomposed {
            match char::from_u32(c) {
                Some(c) => match self.0.decompose(c) {
                    Decomposed::Default => ICU4XDecomposed {
                        first: c as DiplomatChar,
                        second: '\0' as DiplomatChar,
                    },
                    Decomposed::Singleton(s) => ICU4XDecomposed {
                        first: s as DiplomatChar,
                        second: '\0' as DiplomatChar,
                    },
                    Decomposed::Expansion(first, second) => ICU4XDecomposed {
                        first: first as DiplomatChar,
                        second: second as DiplomatChar,
                    },
                },
                _ => ICU4XDecomposed {
                    first: c,
                    second: '\0' as DiplomatChar,
                },
            }
        }
    }
}
