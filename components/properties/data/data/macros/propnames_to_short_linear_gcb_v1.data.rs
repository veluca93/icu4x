// @generated
/// Implement `DataProvider<GraphemeClusterBreakValueToShortNameV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_short_linear_gcb_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.66"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.66"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_TO_SHORT_LINEAR_GCB_V1: &'static <icu::properties::provider::GraphemeClusterBreakValueToShortNameV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::names::PropertyEnumToValueNameLinearMapV1 { map: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x11\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\t\0\x0B\0\r\0\x10\0\x11\0\x12\0\x14\0\x16\0\x18\0\x1A\0\x1D\0\x1F\0XXCNCREXLLFLVLVTTVSMPPRIEBEBGEMGAZ") } };
        }
        #[clippy::msrv = "1.66"]
        impl icu_provider::DataProvider<icu::properties::provider::GraphemeClusterBreakValueToShortNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeClusterBreakValueToShortNameV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPNAMES_TO_SHORT_LINEAR_GCB_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::GraphemeClusterBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
