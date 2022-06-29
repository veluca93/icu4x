// @generated
use icu_provider::prelude::*;
impl ResourceProvider<icu_properties::provider::CasedV1Marker> for super::super::BakedDataProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<icu_properties::provider::CasedV1Marker>, DataError> {
        static VALUES: &[(&str, DataStruct)] = &[("und", UND)];
        #[allow(clippy::unwrap_used)]
        let value = VALUES
            .binary_search_by(|(k, _)| req.options.strict_cmp(k.as_bytes()).reverse())
            .map(|i| VALUES.get(i).unwrap().1)
            .map_err(|_| {
                DataErrorKind::MissingResourceOptions
                    .with_req(<icu_properties::provider::CasedV1Marker>::KEY, req)
            })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                value,
            ))),
        })
    }
}
type DataStruct = &'static <icu_properties::provider::CasedV1Marker as DataMarker>::Yokeable;
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1::InversionList(unsafe {
    #[allow(unused_unsafe)]
    ::icu_uniset::UnicodeSet::from_parts_unchecked(
        unsafe {
            ::zerovec::ZeroVec::from_bytes_unchecked(&[
                65u8, 0u8, 0u8, 0u8, 91u8, 0u8, 0u8, 0u8, 97u8, 0u8, 0u8, 0u8, 123u8, 0u8, 0u8,
                0u8, 170u8, 0u8, 0u8, 0u8, 171u8, 0u8, 0u8, 0u8, 181u8, 0u8, 0u8, 0u8, 182u8, 0u8,
                0u8, 0u8, 186u8, 0u8, 0u8, 0u8, 187u8, 0u8, 0u8, 0u8, 192u8, 0u8, 0u8, 0u8, 215u8,
                0u8, 0u8, 0u8, 216u8, 0u8, 0u8, 0u8, 247u8, 0u8, 0u8, 0u8, 248u8, 0u8, 0u8, 0u8,
                187u8, 1u8, 0u8, 0u8, 188u8, 1u8, 0u8, 0u8, 192u8, 1u8, 0u8, 0u8, 196u8, 1u8, 0u8,
                0u8, 148u8, 2u8, 0u8, 0u8, 149u8, 2u8, 0u8, 0u8, 185u8, 2u8, 0u8, 0u8, 192u8, 2u8,
                0u8, 0u8, 194u8, 2u8, 0u8, 0u8, 224u8, 2u8, 0u8, 0u8, 229u8, 2u8, 0u8, 0u8, 69u8,
                3u8, 0u8, 0u8, 70u8, 3u8, 0u8, 0u8, 112u8, 3u8, 0u8, 0u8, 116u8, 3u8, 0u8, 0u8,
                118u8, 3u8, 0u8, 0u8, 120u8, 3u8, 0u8, 0u8, 122u8, 3u8, 0u8, 0u8, 126u8, 3u8, 0u8,
                0u8, 127u8, 3u8, 0u8, 0u8, 128u8, 3u8, 0u8, 0u8, 134u8, 3u8, 0u8, 0u8, 135u8, 3u8,
                0u8, 0u8, 136u8, 3u8, 0u8, 0u8, 139u8, 3u8, 0u8, 0u8, 140u8, 3u8, 0u8, 0u8, 141u8,
                3u8, 0u8, 0u8, 142u8, 3u8, 0u8, 0u8, 162u8, 3u8, 0u8, 0u8, 163u8, 3u8, 0u8, 0u8,
                246u8, 3u8, 0u8, 0u8, 247u8, 3u8, 0u8, 0u8, 130u8, 4u8, 0u8, 0u8, 138u8, 4u8, 0u8,
                0u8, 48u8, 5u8, 0u8, 0u8, 49u8, 5u8, 0u8, 0u8, 87u8, 5u8, 0u8, 0u8, 96u8, 5u8, 0u8,
                0u8, 137u8, 5u8, 0u8, 0u8, 160u8, 16u8, 0u8, 0u8, 198u8, 16u8, 0u8, 0u8, 199u8,
                16u8, 0u8, 0u8, 200u8, 16u8, 0u8, 0u8, 205u8, 16u8, 0u8, 0u8, 206u8, 16u8, 0u8,
                0u8, 208u8, 16u8, 0u8, 0u8, 251u8, 16u8, 0u8, 0u8, 253u8, 16u8, 0u8, 0u8, 0u8,
                17u8, 0u8, 0u8, 160u8, 19u8, 0u8, 0u8, 246u8, 19u8, 0u8, 0u8, 248u8, 19u8, 0u8,
                0u8, 254u8, 19u8, 0u8, 0u8, 128u8, 28u8, 0u8, 0u8, 137u8, 28u8, 0u8, 0u8, 144u8,
                28u8, 0u8, 0u8, 187u8, 28u8, 0u8, 0u8, 189u8, 28u8, 0u8, 0u8, 192u8, 28u8, 0u8,
                0u8, 0u8, 29u8, 0u8, 0u8, 192u8, 29u8, 0u8, 0u8, 0u8, 30u8, 0u8, 0u8, 22u8, 31u8,
                0u8, 0u8, 24u8, 31u8, 0u8, 0u8, 30u8, 31u8, 0u8, 0u8, 32u8, 31u8, 0u8, 0u8, 70u8,
                31u8, 0u8, 0u8, 72u8, 31u8, 0u8, 0u8, 78u8, 31u8, 0u8, 0u8, 80u8, 31u8, 0u8, 0u8,
                88u8, 31u8, 0u8, 0u8, 89u8, 31u8, 0u8, 0u8, 90u8, 31u8, 0u8, 0u8, 91u8, 31u8, 0u8,
                0u8, 92u8, 31u8, 0u8, 0u8, 93u8, 31u8, 0u8, 0u8, 94u8, 31u8, 0u8, 0u8, 95u8, 31u8,
                0u8, 0u8, 126u8, 31u8, 0u8, 0u8, 128u8, 31u8, 0u8, 0u8, 181u8, 31u8, 0u8, 0u8,
                182u8, 31u8, 0u8, 0u8, 189u8, 31u8, 0u8, 0u8, 190u8, 31u8, 0u8, 0u8, 191u8, 31u8,
                0u8, 0u8, 194u8, 31u8, 0u8, 0u8, 197u8, 31u8, 0u8, 0u8, 198u8, 31u8, 0u8, 0u8,
                205u8, 31u8, 0u8, 0u8, 208u8, 31u8, 0u8, 0u8, 212u8, 31u8, 0u8, 0u8, 214u8, 31u8,
                0u8, 0u8, 220u8, 31u8, 0u8, 0u8, 224u8, 31u8, 0u8, 0u8, 237u8, 31u8, 0u8, 0u8,
                242u8, 31u8, 0u8, 0u8, 245u8, 31u8, 0u8, 0u8, 246u8, 31u8, 0u8, 0u8, 253u8, 31u8,
                0u8, 0u8, 113u8, 32u8, 0u8, 0u8, 114u8, 32u8, 0u8, 0u8, 127u8, 32u8, 0u8, 0u8,
                128u8, 32u8, 0u8, 0u8, 144u8, 32u8, 0u8, 0u8, 157u8, 32u8, 0u8, 0u8, 2u8, 33u8,
                0u8, 0u8, 3u8, 33u8, 0u8, 0u8, 7u8, 33u8, 0u8, 0u8, 8u8, 33u8, 0u8, 0u8, 10u8,
                33u8, 0u8, 0u8, 20u8, 33u8, 0u8, 0u8, 21u8, 33u8, 0u8, 0u8, 22u8, 33u8, 0u8, 0u8,
                25u8, 33u8, 0u8, 0u8, 30u8, 33u8, 0u8, 0u8, 36u8, 33u8, 0u8, 0u8, 37u8, 33u8, 0u8,
                0u8, 38u8, 33u8, 0u8, 0u8, 39u8, 33u8, 0u8, 0u8, 40u8, 33u8, 0u8, 0u8, 41u8, 33u8,
                0u8, 0u8, 42u8, 33u8, 0u8, 0u8, 46u8, 33u8, 0u8, 0u8, 47u8, 33u8, 0u8, 0u8, 53u8,
                33u8, 0u8, 0u8, 57u8, 33u8, 0u8, 0u8, 58u8, 33u8, 0u8, 0u8, 60u8, 33u8, 0u8, 0u8,
                64u8, 33u8, 0u8, 0u8, 69u8, 33u8, 0u8, 0u8, 74u8, 33u8, 0u8, 0u8, 78u8, 33u8, 0u8,
                0u8, 79u8, 33u8, 0u8, 0u8, 96u8, 33u8, 0u8, 0u8, 128u8, 33u8, 0u8, 0u8, 131u8,
                33u8, 0u8, 0u8, 133u8, 33u8, 0u8, 0u8, 182u8, 36u8, 0u8, 0u8, 234u8, 36u8, 0u8,
                0u8, 0u8, 44u8, 0u8, 0u8, 229u8, 44u8, 0u8, 0u8, 235u8, 44u8, 0u8, 0u8, 239u8,
                44u8, 0u8, 0u8, 242u8, 44u8, 0u8, 0u8, 244u8, 44u8, 0u8, 0u8, 0u8, 45u8, 0u8, 0u8,
                38u8, 45u8, 0u8, 0u8, 39u8, 45u8, 0u8, 0u8, 40u8, 45u8, 0u8, 0u8, 45u8, 45u8, 0u8,
                0u8, 46u8, 45u8, 0u8, 0u8, 64u8, 166u8, 0u8, 0u8, 110u8, 166u8, 0u8, 0u8, 128u8,
                166u8, 0u8, 0u8, 158u8, 166u8, 0u8, 0u8, 34u8, 167u8, 0u8, 0u8, 136u8, 167u8, 0u8,
                0u8, 139u8, 167u8, 0u8, 0u8, 143u8, 167u8, 0u8, 0u8, 144u8, 167u8, 0u8, 0u8, 203u8,
                167u8, 0u8, 0u8, 208u8, 167u8, 0u8, 0u8, 210u8, 167u8, 0u8, 0u8, 211u8, 167u8, 0u8,
                0u8, 212u8, 167u8, 0u8, 0u8, 213u8, 167u8, 0u8, 0u8, 218u8, 167u8, 0u8, 0u8, 245u8,
                167u8, 0u8, 0u8, 247u8, 167u8, 0u8, 0u8, 248u8, 167u8, 0u8, 0u8, 251u8, 167u8, 0u8,
                0u8, 48u8, 171u8, 0u8, 0u8, 91u8, 171u8, 0u8, 0u8, 92u8, 171u8, 0u8, 0u8, 105u8,
                171u8, 0u8, 0u8, 112u8, 171u8, 0u8, 0u8, 192u8, 171u8, 0u8, 0u8, 0u8, 251u8, 0u8,
                0u8, 7u8, 251u8, 0u8, 0u8, 19u8, 251u8, 0u8, 0u8, 24u8, 251u8, 0u8, 0u8, 33u8,
                255u8, 0u8, 0u8, 59u8, 255u8, 0u8, 0u8, 65u8, 255u8, 0u8, 0u8, 91u8, 255u8, 0u8,
                0u8, 0u8, 4u8, 1u8, 0u8, 80u8, 4u8, 1u8, 0u8, 176u8, 4u8, 1u8, 0u8, 212u8, 4u8,
                1u8, 0u8, 216u8, 4u8, 1u8, 0u8, 252u8, 4u8, 1u8, 0u8, 112u8, 5u8, 1u8, 0u8, 123u8,
                5u8, 1u8, 0u8, 124u8, 5u8, 1u8, 0u8, 139u8, 5u8, 1u8, 0u8, 140u8, 5u8, 1u8, 0u8,
                147u8, 5u8, 1u8, 0u8, 148u8, 5u8, 1u8, 0u8, 150u8, 5u8, 1u8, 0u8, 151u8, 5u8, 1u8,
                0u8, 162u8, 5u8, 1u8, 0u8, 163u8, 5u8, 1u8, 0u8, 178u8, 5u8, 1u8, 0u8, 179u8, 5u8,
                1u8, 0u8, 186u8, 5u8, 1u8, 0u8, 187u8, 5u8, 1u8, 0u8, 189u8, 5u8, 1u8, 0u8, 128u8,
                7u8, 1u8, 0u8, 129u8, 7u8, 1u8, 0u8, 131u8, 7u8, 1u8, 0u8, 134u8, 7u8, 1u8, 0u8,
                135u8, 7u8, 1u8, 0u8, 177u8, 7u8, 1u8, 0u8, 178u8, 7u8, 1u8, 0u8, 187u8, 7u8, 1u8,
                0u8, 128u8, 12u8, 1u8, 0u8, 179u8, 12u8, 1u8, 0u8, 192u8, 12u8, 1u8, 0u8, 243u8,
                12u8, 1u8, 0u8, 160u8, 24u8, 1u8, 0u8, 224u8, 24u8, 1u8, 0u8, 64u8, 110u8, 1u8,
                0u8, 128u8, 110u8, 1u8, 0u8, 0u8, 212u8, 1u8, 0u8, 85u8, 212u8, 1u8, 0u8, 86u8,
                212u8, 1u8, 0u8, 157u8, 212u8, 1u8, 0u8, 158u8, 212u8, 1u8, 0u8, 160u8, 212u8, 1u8,
                0u8, 162u8, 212u8, 1u8, 0u8, 163u8, 212u8, 1u8, 0u8, 165u8, 212u8, 1u8, 0u8, 167u8,
                212u8, 1u8, 0u8, 169u8, 212u8, 1u8, 0u8, 173u8, 212u8, 1u8, 0u8, 174u8, 212u8, 1u8,
                0u8, 186u8, 212u8, 1u8, 0u8, 187u8, 212u8, 1u8, 0u8, 188u8, 212u8, 1u8, 0u8, 189u8,
                212u8, 1u8, 0u8, 196u8, 212u8, 1u8, 0u8, 197u8, 212u8, 1u8, 0u8, 6u8, 213u8, 1u8,
                0u8, 7u8, 213u8, 1u8, 0u8, 11u8, 213u8, 1u8, 0u8, 13u8, 213u8, 1u8, 0u8, 21u8,
                213u8, 1u8, 0u8, 22u8, 213u8, 1u8, 0u8, 29u8, 213u8, 1u8, 0u8, 30u8, 213u8, 1u8,
                0u8, 58u8, 213u8, 1u8, 0u8, 59u8, 213u8, 1u8, 0u8, 63u8, 213u8, 1u8, 0u8, 64u8,
                213u8, 1u8, 0u8, 69u8, 213u8, 1u8, 0u8, 70u8, 213u8, 1u8, 0u8, 71u8, 213u8, 1u8,
                0u8, 74u8, 213u8, 1u8, 0u8, 81u8, 213u8, 1u8, 0u8, 82u8, 213u8, 1u8, 0u8, 166u8,
                214u8, 1u8, 0u8, 168u8, 214u8, 1u8, 0u8, 193u8, 214u8, 1u8, 0u8, 194u8, 214u8, 1u8,
                0u8, 219u8, 214u8, 1u8, 0u8, 220u8, 214u8, 1u8, 0u8, 251u8, 214u8, 1u8, 0u8, 252u8,
                214u8, 1u8, 0u8, 21u8, 215u8, 1u8, 0u8, 22u8, 215u8, 1u8, 0u8, 53u8, 215u8, 1u8,
                0u8, 54u8, 215u8, 1u8, 0u8, 79u8, 215u8, 1u8, 0u8, 80u8, 215u8, 1u8, 0u8, 111u8,
                215u8, 1u8, 0u8, 112u8, 215u8, 1u8, 0u8, 137u8, 215u8, 1u8, 0u8, 138u8, 215u8, 1u8,
                0u8, 169u8, 215u8, 1u8, 0u8, 170u8, 215u8, 1u8, 0u8, 195u8, 215u8, 1u8, 0u8, 196u8,
                215u8, 1u8, 0u8, 204u8, 215u8, 1u8, 0u8, 0u8, 223u8, 1u8, 0u8, 10u8, 223u8, 1u8,
                0u8, 11u8, 223u8, 1u8, 0u8, 31u8, 223u8, 1u8, 0u8, 0u8, 233u8, 1u8, 0u8, 68u8,
                233u8, 1u8, 0u8, 48u8, 241u8, 1u8, 0u8, 74u8, 241u8, 1u8, 0u8, 80u8, 241u8, 1u8,
                0u8, 106u8, 241u8, 1u8, 0u8, 112u8, 241u8, 1u8, 0u8, 138u8, 241u8, 1u8, 0u8,
            ])
        },
        4453usize,
    )
});
