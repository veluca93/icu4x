// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collection of predicate traits and functions for forking providers.

use icu_provider::prelude::*;

/// The predicate trait used by [`ForkByErrorProvider`].
///
/// [`ForkByErrorProvider`]: super::ForkByErrorProvider
pub trait ForkByErrorPredicate {
    /// The error to return if there are zero providers.
    const UNIT_ERROR: DataErrorKind = DataErrorKind::MissingDataMarker;

    /// This function is called when a data request fails and there are additional providers
    /// that could possibly fulfill the request.
    ///
    /// Arguments:
    ///
    /// - `&self` = Reference to the struct implementing the trait (for data capture)
    /// - `marker` = The [`DataMarkerInfo`] associated with the request
    /// - `req` = The [`DataRequest`]. This may be `None` if there is no request, such as
    ///           inside [`IterableDynamicDataProvider`].
    /// - `err` = The error that occurred.
    ///
    /// Return value:
    ///
    /// - `true` to discard the error and attempt the request with the next provider.
    /// - `false` to return the error and not perform any additional requests.
    ///
    /// [`DataMarkerInfo`]: icu_provider::DataMarkerInfo
    /// [`DataRequest`]: icu_provider::DataRequest
    /// [`IterableDynamicDataProvider`]: icu_provider::datagen::IterableDynamicDataProvider
    fn test(&self, marker: DataMarkerInfo, req: Option<DataRequest>, err: DataError) -> bool;
}

/// A predicate that allows forking providers to search for a provider that supports a
/// particular data marker.
///
/// This is normally used implicitly by [`ForkByMarkerProvider`].
///
/// [`ForkByMarkerProvider`]: super::ForkByMarkerProvider
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive] // Not intended to be constructed
pub struct MissingDataMarkerPredicate;

impl ForkByErrorPredicate for MissingDataMarkerPredicate {
    const UNIT_ERROR: DataErrorKind = DataErrorKind::MissingDataMarker;

    #[inline]
    fn test(&self, _: DataMarkerInfo, _: Option<DataRequest>, err: DataError) -> bool {
        matches!(
            err,
            DataError {
                kind: DataErrorKind::MissingDataMarker,
                ..
            }
        )
    }
}

/// A predicate that allows forking providers to search for a provider that supports a
/// particular locale, based on whether it returns [`DataErrorKind::MissingLocale`].
///
/// # Examples
///
/// Configure a multi-language data provider pointing at two language packs:
///
/// ```
/// use icu_provider_adapters::fork::ForkByErrorProvider;
/// use icu_provider_adapters::fork::predicates::MissingLocalePredicate;
/// use icu_provider_fs::FsDataProvider;
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::HelloWorldV1Marker;
/// use icu_locale_core::langid;
///
/// // The `tests` directory contains two separate "language packs" for Hello World data.
/// let provider_de = FsDataProvider::try_new("tests/data/langtest/de").unwrap();
/// let provider_ro = FsDataProvider::try_new("tests/data/langtest/ro").unwrap();
///
/// // Create the forking provider:
/// let provider = ForkByErrorProvider::new_with_predicate(
///     provider_de,
///     provider_ro,
///     MissingLocalePredicate
/// );
///
/// // Test that we can load both "de" and "ro" data:
///
/// let german_hello_world: DataPayload<HelloWorldV1Marker> = provider
///     .as_deserializing()
///     .load(DataRequest {
///         locale: &langid!("de").into(),
///         ..Default::default()
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("Hallo Welt", german_hello_world.get().message);
///
/// let romanian_hello_world: DataPayload<HelloWorldV1Marker> = provider
///     .as_deserializing()
///     .load(DataRequest {
///         locale: &langid!("ro").into(),
///         ..Default::default()
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("Salut, lume", romanian_hello_world.get().message);
///
/// // We should not be able to load "en" data because it is not in the provider:
///
/// DataProvider::<HelloWorldV1Marker>::load(
///     &provider.as_deserializing(),
///     DataRequest {
///         locale: &langid!("en").into(),
///         ..Default::default()
///     }
/// )
/// .expect_err("No English data");
/// ```
#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // empty type
pub struct MissingLocalePredicate;

impl ForkByErrorPredicate for MissingLocalePredicate {
    const UNIT_ERROR: DataErrorKind = DataErrorKind::MissingLocale;

    #[inline]
    fn test(&self, _: DataMarkerInfo, _: Option<DataRequest>, err: DataError) -> bool {
        matches!(
            err,
            DataError {
                kind: DataErrorKind::MissingLocale,
                ..
            }
        )
    }
}
