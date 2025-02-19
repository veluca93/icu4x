// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_core_helloworld_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.70"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, marker: icu_provider::DataMarkerInfo, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match marker.path.hashed() {
                    h if h == <icu_provider::hello_world::HelloWorldV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu_provider::hello_world::HelloWorldV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataMarker.with_req(marker, req)),
                }
            }
        }
    };
}
#[clippy::msrv = "1.70"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
