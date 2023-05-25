// @generated
#![cfg(feature = "icu_normalizer")]
type DataStruct =
    <icu_normalizer::provider::CanonicalCompositionsV1Marker as icu_provider::DataMarker>::Yokeable;
pub fn lookup(locale: &icu_provider::DataLocale) -> Option<&'static DataStruct> {
    locale.is_empty().then(|| &UND)
}
static UND: DataStruct = include!("und.rs.data");
