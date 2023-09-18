// @generated
/// Implement `DataProvider<WordBreakV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_wb_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.66"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.66"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_WB_V1: &'static <icu::properties::provider::WordBreakV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::PropertyCodePointMapV1::CodePointTrie(icu::collections::codepointtrie::CodePointTrie::from_parts(icu::collections::codepointtrie::CodePointTrieHeader { high_start: 918016u32, shifted12_high_start: 225u16, index3_null_offset: 636u16, data_null_offset: 14u32, null_value: 0u32, trie_type: icu::collections::codepointtrie::TrieType::Small }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\0@\0{\0\xBB\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\x1B\x01[\x01k\x01\xAB\x01\xCE\x01\xF3\0\xF3\0\x0C\x02\xF3\0\x1C\x02M\x02\x84\x02\xC2\x02\x02\x037\x03\xF3\0h\x03\xA8\x03\xDD\x03\xF7\x037\x04w\x04\xB7\x04\xEF\x04%\x05a\x05\x9F\x05\xDE\x05\x1C\x06[\x06\x99\x06\xD8\x06\x16\x07V\x07\x94\x07\xD2\x07\x10\x08P\x08\x8E\x08\xCE\x08\x0C\tL\t\x8A\t\xCA\t\n\n>\ny\n\x93\n\xD0\n\x10\x0BP\x0B\x8B\x0B\xC8\x0B\x19\t3\tC\tY\ty\t\x97\t\xAF\t\xCE\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t3\t\xEE\t\0\0\x10\0 \x000\0@\0P\0`\0p\0{\0\x8B\0\x9B\0\xAB\0\xBB\0\xCB\0\xDB\0\xEB\0\xF3\0\x03\x01\x13\x01#\x01\xF3\0\x03\x01\x13\x01#\x01\xF3\0\x03\x01\x13\x01#\x01\xF3\0\x03\x01\x13\x01#\x01\x1B\x01+\x01;\x01K\x01[\x01k\x01{\x01\x8B\x01k\x01{\x01\x8B\x01\x9B\x01\xAB\x01\xBB\x01\xCB\x01\xDB\x01\xCE\x01\xDE\x01\xEE\x01\xFE\x01\xF3\0\x03\x01\x13\x01#\x01\xF3\0\x03\x01\x13\x01#\x01\x0C\x02\x1C\x02,\x02<\x02\xF3\0\x03\x01\x13\x01#\x01\x1C\x02,\x02<\x02L\x02M\x02]\x02m\x02}\x02\x84\x02\x94\x02\xA4\x02\xB4\x02\xC2\x02\xD2\x02\xE2\x02\xF2\x02\x02\x03\x12\x03\"\x032\x037\x03G\x03W\x03g\x03\xF3\0\x03\x01\x13\x01#\x01h\x03x\x03\x88\x03\x98\x03\xA8\x03\xB8\x03\xC8\x03\xD8\x03\xDD\x03\xED\x03\xFD\x03\r\x04\xF7\x03\x07\x04\x17\x04'\x047\x04G\x04W\x04g\x04w\x04\x87\x04\x97\x04\xA7\x04\xB7\x04\xC7\x04\xD7\x04\xE7\x04\xEF\x04\xFF\x04\x0F\x05\x1F\x05%\x055\x05E\x05U\x05a\x05q\x05\x81\x05\x91\x05\x9F\x05\xAF\x05\xBF\x05\xCF\x05\xDE\x05\xEE\x05\xFE\x05\x0E\x06\x1C\x06,\x06<\x06L\x06[\x06k\x06{\x06\x8B\x06\x99\x06\xA9\x06\xB9\x06\xC9\x06\xD8\x06\xE8\x06\xF8\x06\x08\x07\x16\x07&\x076\x07F\x07V\x07f\x07v\x07\x86\x07\x94\x07\xA4\x07\xB4\x07\xC4\x07\xD2\x07\xE2\x07\xF2\x07\x02\x08\x10\x08 \x080\x08@\x08P\x08`\x08p\x08\x80\x08\x8E\x08\x9E\x08\xAE\x08\xBE\x08\xCE\x08\xDE\x08\xEE\x08\xFE\x08\x0C\t\x1C\t,\t<\tL\t\\\tl\t|\t\x8A\t\x9A\t\xAA\t\xBA\t\xCA\t\xDA\t\xEA\t\xFA\t\n\n\x1A\n*\n:\n>\nN\n^\nn\ny\n\x89\n\x99\n\xA9\n\x93\n\xA3\n\xB3\n\xC3\n\xD0\n\xE0\n\xF0\n\0\x0B\x10\x0B \x0B0\x0B@\x0BP\x0B`\x0Bp\x0B\x80\x0B\x8B\x0B\x9B\x0B\xAB\x0B\xBB\x0B\xC8\x0B\xD8\x0B\xE8\x0B\xF8\x0B\x0E\0\x0E\0E\x08\xB3\x02<\x07\x02\x0C\x11\x0C \x0C.\x0C>\x0C\xF3\0\xF3\0N\x0C\xF3\0\xF3\0\xC7\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0^\x0Cn\x0C\xF3\0\xF3\0^\x0C\xF3\0\xF3\0f\x0Cv\x0C\xCB\0\xF3\0\xF3\0\xF3\0v\x0C\xF3\0\xF3\0\xF3\0~\x0C\x0E\0\x0E\0\xF3\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\x8E\x0C@\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0W\x02\xF3\0\x9E\x0Cp\0\xF3\0\xF3\0\xF3\0\xF3\0\xD6\t\xA5\x0C\xF3\0\xB5\x0C\xF3\0\xC4\x0C\xF3\0\xD4\x0C\xC5\0\xE4\x0C\x0E\0\x0E\0\x0E\0\x91\x02[\x01\xF4\x0C<\x07\x0E\0\x02\r<\x07\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xA5\x0C\x12\r\xF3\0\x19\r\xF3\0\xF3\0\xF3\0\xF3\0)\r\xF3\0L\0\xBC\x0B\xBC\x0B1\x04\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0<\x07\x0E\0\x0E\0\xF3\09\r\x0E\0\x0E\0\x0E\0E\r[\x01U\r<\x07<\x07\x0E\0[\x01\xB3\x02\x0E\0\x0E\0\x0E\0\x11\x02\xF3\0\xF3\0W\x01d\r<\x07E\x08\xA1\x04\x13\x02\xF3\0t\r7\x04\xF3\0\xF3\0U\x01\xA1\x04\xF3\0\xF3\0W\x01\xC8\n\x84\r7\x04\xF3\0M\0\xA5\x0C\xF3\0\xF3\0Y\x02\x0E\0\xA0\x0B\x94\r\x9D\r\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0[\x01[\x01[\x01[\x01\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\x8E\x0C\xF3\0\xF3\0\x8E\x0C\xAD\r\xF3\0M\0\xF3\0\xF3\0\xF3\0\x02\x06\xBD\r\xCD\rN\0\xBD\r\xDD\r\xED\r\xF9\r\t\x0E\x18\x0E$\x0E4\x0ED\x0E\x0E\0N\0\x0E\0\x0E\0\x0E\0[\x01[\x01\xA4\x04Q\x0E]\x0Ek\x0EZ\x02{\x0E\x0E\0\xF3\0\xF3\0\xA5\x0C\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\x003\x01\xF3\0\xF3\0)\x01\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\x8B\x0E\x9A\x0E\xF3\0\xF3\0N\x0C\xF3\0\xF3\0\xF3\0\xA6\x0CA\x08\xF3\0\xAA\x0E\0\x06\0\x06\0\x06\0\x06[\x01[\x01\x0E\0\x0E\0\x96\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xBA\x0E\x0E\0&\x0C\xC9\x0E\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xD6\x0E\xE6\x0E\xE6\x0E\xE6\x0E\xE6\x0E\xE6\x0E\xEB\x0E<\0\xF3\0\xF3\0@\0\xF3\0\xF3\0\xF3\0\xF3\0L\0\x0E\0\xF3\0\xF3\0\x0E\0\x0E\0\x0E\0\xE6\x0E\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xE6\x0E\xE6\x0E\xE7\x0E\xE6\x0E\xE6\x0E\xE6\x0E\xE6\x0E\xE6\x0E\xF7\x0E\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0N\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0M\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0N\0\xF3\0B\x06\x0E\0\xF3\0\xF3\0L\x01\x07\x0F\xF3\0M\x01\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xA3\x04\xAF\x04\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0p\0\x16\x0F\x0E\0?\0&\x0F\xF3\x003\x0F\x0E\0\xF3\0\xF3\0\xF3\0C\x0F\x14\x02\xF3\0\xF3\0W\x01S\x0F<\x07[\x01c\x0F7\x04\xF3\0s\x0F\xF3\0T\x01\xA1\x04\xF3\0N\0\x12\x02\xF3\0\xF3\0X\x01\x80\x0F<\x07\"\x08<\x07\xF3\0\xF3\0R\x01z\x0F\x8F\x0F<\x07\x0E\0\x9D\x0F\x0E\0\x0E\0\x0E\0\xAD\x0F&\x08\x0E\0P\x01\xBD\x0F\xCC\x0F(\r\0\x06\xF3\0\xF3\0\xF3\0)\x01\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xDC\x0F<\x07\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0C\x0F\xF3\0\xEC\x0F\xF3\0\xF3\0O\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xAA\x0E\xF4\x0F\x03\x10\r\x10\x1D\x10\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0-\x10\x0E\0>\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0M\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0?\0\xF3\0\xF3\0=\x10\x0E\0\x0E\0O\0[\x01M\x10[\x01Z\x10_\x10o\x10\x0E\0\xCD\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\x7F\x10\x8F\x100\0@\0P\0@\0p\0\x9E\x10\xE6\x0E\xE6\x0E\xA4\x10\xF3\0L\0\xCB\x0F\xB4\x10\x0E\0\xC1\x10\xC6\0\xF3\0\xCB\0\xD1\x10M\0M\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0p\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xE0\x10\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0b\n\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0N\0\xF3\0\xF3\0\xF3\0\xA7\x03\xA4\x04\x0E\0\xF3\0\xF3\0\xAA\x04\xF3\0p\0\xF3\0\xF3\0:\r\xF3\0M\0\xF3\0\xF3\0\xF0\x10\xDF\x10\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0M\0<\x07\xF3\0\xF3\0\xF0\x10\xF3\0O\0\xF3\0\xF3\0=\x10\xF3\0\xF3\0\xF3\0C\x0F\xC7\0\xC7\0\xFD\x10\xD0\0\x0B\x11\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xAA\x0E\xF3\0)\r=\x10\x0E\0\xCC\0\xF3\0\xF3\0\x1B\x11\x0E\0\x0E\0\x0E\0\x0E\0+\x11\xF3\0\xF3\x005\x11\xF3\0)\r\xF3\0\xAA\x0E\xF3\0L\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0D\x11\xF3\0)\r\xF3\0)\x01\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0+\x01\x0E\0\x0E\0\x0E\0\x0E\0T\x11\xE2\x06\xF3\0d\x11\x0E\0\x0E\0\xF3\0N\0\xF3\0N\0\x0E\0\x0E\0\xCA\0\xF3\0\x96\x0F\x0E\0\xF3\0\xF3\0\xF3\0)\r\xF3\0)\r\xF3\0t\x11\xF3\0-\x10\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xA5\x0C\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0t\x11\xF3\0\xF3\0\xF3\0t\x11\xF3\0\xF3\0\x84\x11<\x07\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\x94\x11-\x10\x0E\0\x0E\0\x0E\0C\x08\xF3\0N\0\t\x0B\xF3\0U\x01\xA4\x04\x0E\0\xF3\0\xA4\x11\x0E\0\x0E\0\xF3\0\xE0\x10\x0E\0\xF3\0\xAA\x0E\x13\x02\xF3\0\xF3\0S\x01z\x0F\x0E\x001\x04\xB4\x11\x13\x02\xF3\0\xF3\0\xC3\x11\xD1\x11\xF3\0\xA5\x0C<\x07\x13\x02\xF3\0T\x01\x83\n\xDF\x11\xF3\0\xF3\0\xEF\x11\x13\x02\xF3\0\xF3\0X\x01\xFF\x11\x0F\x12\x0E\0\x0E\0\xF3\0\xD0\0O\x01\x1F\x12.\x12\x0E\0\x0E\0\x0E\0>\x12\xC4\0\xA5\x0C\xF3\0\xF3\0L\x01\xBD\x0B<\x07N\x12\xEE\x05\xC9\0]\x12\x94\x07m\x12{\x12\xA0\x04\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0V\x01\x8B\x12\x9B\x12-\x10\x0E\0\xF3\0\xF3\0\xF3\0[\x01\xAB\x12<\x07\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0L\x01\xBB\x12\xA4\x04\xCB\x12\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0[\x01\xD8\x12<\x07\x0E\0\x0E\0\xF3\0\xF3\0P\x01 \x04<\x07\x0E\0\x0E\0\x0E\0\x0E\0C\x08\xBC\x0B<\x07\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0O\x01\xBD\x0B\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0<\x07\x96\0\xE8\x12\xFC\x10\xF3\0\xF8\x12\x06\x13<\x07\x0E\0\x0E\0\x0E\0\x0E\0\x16\x13\xF3\0\xF3\0%\x134\x13\x0E\0D\x13\xF3\0\xF3\0Q\x13\xFA\x0Ca\x13\xF3\0\xF3\0Q\x01q\x13\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xA5\x0C\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xC9\0\xF3\0L\x01\x9C\x0B\xA7\x03<\x07\x0E\0?\0\xF3\0\x93\x02\x9B\x0Bz\x0F\x0E\0\x0E\0\x0E\0\x0E\0\x02\x07\xF3\0\xF3\0\x81\x13\x90\x13<\x07\xFA\x10\xF3\0\xA0\x13\xAD\x13<\x07\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xBD\x13\xCD\x13\xD1\0\xF3\0\xD9\x13\xA2\x04<\x07\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xA7\x03\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0)\x01\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0L\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0C\x0F\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xA7\x03\xF3\0\xF3\0\xF3\0\xE9\x13\xF9\x13S\x0F\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xAA\x0E\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xA5\x0C\xF3\0L\0<\x07\xF3\0\xF3\0\xF3\0\xF3\0L\0<\x07\xF3\0M\0\xA0\x04\xF3\0\xF3\0\xF3\0z\x0FC\x0F<\x07>\0\xDA\x04\xF3\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\x01\x08Z\x01[\x01[\x01\x01\x14\x13\x02\x0E\0\x0E\0\x0E\0\x0E\0\x11\x14\xA3\x04\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0!\x14/\x14\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0?\x14\x0E\0\x0E\0J\x14V\x14\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0p\0N\0\xA5\x0Cf\x14v\x14\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0[\x01[\x01\xDA\x03[\x01z\x0F\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x81\x14\x8E\x14\x9B\x14\x0E\0\xFE\x0B\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xA9\x14\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xCD\0\xF3\0\xF3\0\xF3\0\xC5\0\xB7\x14\xC5\x14\xCE\0\xF3\0\xF3\0\xF3\0a\x0C\x7F\x06\xF3\0\xD2\x14\xE2\x14\xD1\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xEC\x14\xF3\0\xD1\0\xC7\0\xF3\0\xC7\0\xF3\0\xCD\0\xF3\0\xCD\0L\0\xF3\0L\0\xF3\0\xC9\0\xF3\0\xC9\0\xF3\0\xF9\x14\x07\x15\x07\x15\x07\x15[\x01[\x01[\x01\x17\x15[\x01[\x01\xBB\x0B\"\x08#\x08E\x08\x94\x02\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0L\0$\r\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x9C\x0BY\r%\x15\xF3\0\xF3\0\xF3\0M\0\x0E\0A\x08\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0\xF3\0N\x005\x15E\x15\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0U\x15\x0E\0\xF3\0\xF3\0O\x01<\x07\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0O\x01<\x07\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0e\x15L\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xF3\0\xE0\x10z\x0F\x0E\0\x0E\0\xF3\0\xF3\0\xF3\0\xF3\0u\x15<\x07\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xCE\0\xF3\0\x84\x15\x91\x15\x9F\x15\xAF\x15\xBD\x15\xC5\x15\xC8\0O\0\xD4\x15O\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xF3\0)\x01\xF3\0)\x01\xF3\0)\x01\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\xE4\x15\xEA\x15\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0E\x08\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0<\x07\xFA\x15\x0E\0[\x01[\x01[\x01[\x01[\x01[\x01\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0\x0E\0[\x01[\x01[\x01[\x01[\x01[\x01[\x01[\x01[\x01[\x01[\x01[\x01[\x01[\x01[\x01\x0E\0y\0\x8D\0\xAD\0\xCD\0\xED\0\r\x01-\x01M\x01m\x01\x89\x01\xA9\x01\xC3\x01\xE3\x01\x03\x02#\x02C\x02c\x02|\x02\x91\x02|\x02|\x02|\x02\xB1\x02\xD1\x02\xF1\x02\x11\x03|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x021\x031\x03I\x03i\x03\x89\x03\xA9\x031\x031\x031\x031\x031\x031\x031\x031\x031\x031\x031\x03\xC9\x03|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02\xE9\x03\x07\x04'\x04G\x04g\x04\x87\x04\xA7\x04\xC7\x04\xE7\x04\x07\x05\x1F\x05?\x05_\x05\x7F\x05\x9F\x05\xBF\x05\xDF\x05\xFF\x05\x1A\x061\x03:\x06Z\x06|\x02|\x02|\x02|\x02o\x061\x031\x03\x8F\x06|\x02|\x02|\x02|\x02|\x021\x03\xAF\x06|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x021\x03\xCF\x06|\x02\xEB\x06|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02|\x02\x0B\x07+\x07|\x02|\x02|\x02|\x02|\x02K\x07|\x02|\x02|\x02|\x02|\x02|\x02|\x02[\x07p\x07\x8C\x07\xAC\x07\xC2\x07|\x02\xE2\x07|\x02\xF2\x07\x12\x08)\x08<\x08L\x08l\x08|\x02|\x02\x8C\x08\x99\x08\xB9\x08|\x02|\x02|\x02\xD9\x08|\x02|\x02\xF9\x08") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\0\0\0\0\0\0\0\0\0\n\x0C\x0C\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\0\x10\0\0\0\0\x0F\0\0\0\0\x05\0\x0B\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x04\x05\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\x07\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\0\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\x02\0\0\0\0\0\0\0\x01\0\x04\0\0\x01\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\x01\x01\x01\x01\x01\0\x01\x01\0\0\x01\x01\x01\x01\x05\x01\0\0\0\0\0\0\x01\x04\x01\x01\x01\0\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\t\t\t\t\t\t\t\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\x01\x01\x01\x01\0\x01\x04\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x05\x01\0\0\0\0\0\0\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\0\t\t\0\t\t\0\t\0\0\0\0\0\0\0\0\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\0\0\0\0\x0E\x0E\x0E\x0E\x01\x04\0\0\0\0\0\0\0\0\0\0\0\x02\x02\x02\x02\x02\x02\0\0\0\0\0\0\x05\x05\0\0\t\t\t\t\t\t\t\t\t\t\t\0\x02\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\x06\x05\0\x01\x01\t\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\t\t\t\t\t\t\t\x02\0\t\t\t\t\t\t\x01\x01\t\t\0\t\t\t\t\x01\x01\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x01\x01\x01\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x01\t\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\t\t\t\t\t\t\t\t\t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\t\t\t\t\t\t\t\x01\x01\0\0\x05\0\x01\0\0\t\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\t\t\x01\t\t\t\t\t\t\t\t\t\x01\t\t\t\x01\t\t\t\t\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\t\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\0\x02\x02\0\0\0\0\0\0\t\t\t\t\t\t\t\t\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\x02\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\t\x01\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\x01\t\t\t\t\t\t\t\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\t\0\x01\x01\x01\x01\x01\x01\x01\x01\0\0\x01\x01\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\0\x01\0\0\0\x01\x01\x01\x01\0\0\t\x01\t\t\t\t\t\0\0\t\t\0\0\t\t\t\x01\0\0\0\0\0\0\0\0\t\0\0\0\0\x01\x01\0\x01\x01\x01\t\t\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x01\x01\0\0\0\0\0\0\0\0\0\0\x01\0\t\0\t\t\t\0\x01\x01\x01\x01\x01\x01\0\0\0\0\x01\x01\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\0\x01\x01\0\x01\x01\0\0\t\0\t\t\t\0\0\0\0\t\t\0\0\t\t\t\0\0\0\t\0\0\0\0\0\0\0\x01\x01\x01\x01\0\x01\0\0\0\0\0\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\t\t\x01\x01\x01\t\0\0\0\0\0\0\0\0\0\0\t\t\t\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\0\x01\x01\x01\x01\x01\0\0\t\x01\t\t\t\t\t\t\0\t\t\t\0\t\t\t\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\t\t\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\0\0\0\0\0\0\0\0\x01\t\t\t\t\t\t\0\t\t\t\0\x01\x01\x01\x01\x01\x01\x01\x01\0\0\x01\x01\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\0\x01\x01\x01\x01\x01\0\0\t\x01\t\t\t\t\t\0\0\t\t\0\0\t\t\t\0\0\0\0\0\0\0\t\t\t\0\0\0\0\x01\x01\0\x01\x01\x01\t\t\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\t\x01\0\x01\x01\x01\x01\x01\x01\0\0\0\x01\x01\x01\0\x01\x01\x01\x01\0\0\0\x01\x01\0\x01\0\x01\x01\0\0\0\x01\x01\0\0\0\x01\x01\x01\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\t\t\t\0\0\0\t\t\t\0\t\t\t\t\0\0\x01\0\0\0\0\0\0\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\t\t\t\t\t\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\t\x01\t\t\t\t\t\0\t\t\t\0\t\t\t\t\0\0\0\0\0\0\0\t\t\0\x01\x01\x01\0\0\x01\0\0\x01\x01\t\t\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\t\t\t\0\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\0\0\t\x01\t\t\t\t\t\0\t\t\t\0\t\t\t\t\0\0\0\0\0\0\0\t\t\0\0\0\0\0\0\x01\x01\0\x01\x01\t\t\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\x01\x01\t\0\0\0\0\0\0\0\0\0\0\0\0\t\t\t\t\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\x01\t\t\t\t\t\0\t\t\t\0\t\t\t\t\x01\0\0\0\0\0\x01\x01\x01\t\0\0\0\0\0\0\0\x01\x01\x01\t\t\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\0\t\t\t\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\0\0\x01\x01\x01\x01\x01\x01\x01\0\0\0\t\0\0\0\0\t\t\t\t\t\t\0\t\0\t\t\t\t\t\t\t\t\0\0\0\0\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\0\t\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\t\0\0\t\t\t\t\t\t\t\0\0\0\0\0\0\0\t\t\t\t\t\t\t\t\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\t\0\0\t\t\t\t\t\t\t\t\t\0\0\0\0\0\0\0\0\t\t\t\t\t\t\t\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\t\t\0\0\0\0\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\0\0\0\0\0\0\0\0\0\0\t\0\t\0\t\0\0\0\0\t\t\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\0\t\t\x01\x01\x01\x01\x01\t\t\t\t\t\t\t\t\t\t\t\0\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\0\0\0\0\0\0\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\t\t\t\t\0\0\0\0\t\t\0\t\t\t\0\0\t\t\t\t\t\t\t\0\0\t\t\t\t\0\0\0\0\0\0\0\0\0\0\0\t\t\t\t\t\t\t\t\t\t\t\t\0\t\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\t\t\t\t\0\0\x01\x01\x01\x01\x01\x01\0\x01\0\0\0\0\0\x01\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\0\0\x01\x01\x01\x01\x01\x01\x01\0\x01\0\x01\x01\x01\x01\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\t\t\t\x01\x01\x01\x01\x01\x01\0\0\x01\x01\x01\x01\x01\x01\0\0\x16\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\0\0\0\x01\x01\t\t\t\t\0\0\0\0\0\0\0\0\0\x01\x01\t\t\t\0\0\0\0\0\0\0\0\0\0\0\x01\x01\t\t\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\t\t\0\0\0\0\0\0\0\0\0\0\0\0\t\t\t\t\0\0\0\0\0\0\0\0\0\t\0\0\0\0\0\0\0\0\0\0\0\t\t\t\x02\t\x01\x01\x01\x01\x01\t\t\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\x01\0\0\0\0\0\x01\x01\x01\x01\x01\x01\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\t\t\t\t\t\0\0\0\0\0\t\t\t\t\t\t\t\t\t\t\0\t\t\t\t\t\t\t\t\t\t\t\t\t\0\0\t\t\t\t\t\x01\x01\x01\x01\x01\x01\x01\x01\0\0\0\x01\t\t\t\t\t\t\t\t\t\t\t\t\t\x01\x01\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\0\0\x01\x01\x01\t\t\t\t\t\t\t\t\t\x01\x01\x01\x01\t\x01\x01\t\t\t\x01\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\0\x01\0\x01\0\x01\0\0\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\0\0\0\x01\x01\x01\x01\0\0\x01\x01\x01\x01\x01\x01\0\0\0\0\x16\x16\x16\x16\x16\x16\x16\0\x16\x16\x16\0\t\x15\x02\x02\0\0\0\0\0\0\0\0\x0B\x0B\0\0\0\0\0\0\x0B\0\0\x04\x0C\x0C\x02\x02\x02\x02\x02\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x07\0\0\0\x05\0\0\0\0\0\0\0\0\0\0\0\x07\0\0\0\0\0\0\0\0\0\0\x16\x02\x02\x02\x02\x02\0\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\0\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\0\0\x01\0\0\x01\x01\x01\x01\x01\x01\0\x01\0\0\0\x01\x01\x01\x01\x01\0\0\0\0\x01\0\x01\0\x01\0\x01\x01\x01\x01\0\x01\0\0\0\0\0\x01\x01\x01\x01\x01\0\0\0\0\x01\0\x01\x01\x01\x01\x01\0\0\0\0\0\0\x01\x01\x01\x01\t\t\x01\x01\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\0\0\0\0\0\x16\0\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\x03\x03\x03\x03\x03\0\0\0\0\0\x01\x01\0\0\0\0\0\0\0\0\0\t\t\x03\x03\0\0\0\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\0\x03\x03\x03\x03\x03\x03\x03\x03\0\0\0\0\0\0\0\0\t\t\t\0\t\t\t\t\t\t\t\t\t\t\0\x01\x01\0\x01\0\x01\x01\x01\x01\x01\0\0\0\0\0\0\x01\x01\t\x01\x01\x01\t\x01\x01\x01\x01\t\x01\x01\x01\x01\t\t\t\t\t\0\0\0\0\t\0\0\0\x01\x01\x01\x01\0\0\0\0\0\0\0\0\0\0\0\0\t\t\t\t\t\t\0\0\0\0\0\0\0\0\0\0\t\t\x01\x01\x01\x01\x01\x01\0\0\0\x01\0\x01\x01\t\x01\x01\x01\x01\x01\x01\t\t\t\t\t\t\t\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\t\x01\x01\x01\x01\x01\x01\x01\x01\t\t\0\0\0\0\0\0\0\0\0\0\0\t\t\t\0\0\t\0\t\t\t\0\0\t\t\0\0\0\0\0\t\t\0\0\x01\x01\x01\t\t\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\0\0\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\t\t\t\t\t\t\t\t\0\t\t\0\0\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\x01\x01\x01\x01\x01\0\0\0\0\0\x0E\t\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\0\x0E\x0E\x0E\x0E\x0E\x0E\x0E\0\x0E\x0E\x0E\x0E\x0E\0\x0E\0\x0E\x0E\0\x0E\x0E\0\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x0E\x01\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\0\0\0\0\x05\0\0\x04\x05\0\0\0\0\0\0\0\0\0\0\0\x07\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\x07\x07\x07\x05\0\x0B\0\x05\x04\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\x02\0\0\0\0\0\0\0\x0B\0\0\0\0\x05\0\x0B\0\0\0\0\0\0\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\t\t\0\0\x01\x01\x01\x01\x01\x01\0\0\x01\x01\x01\0\0\0\0\0\0\0\0\0\x02\x02\x02\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\0\x01\x01\x01\x01\x01\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\0\0\0\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\0\0\0\x01\x01\x01\x01\x01\x01\0\0\x01\0\x01\x01\x01\x01\x01\x01\0\x01\x01\0\0\0\x01\0\0\x01\x01\x01\0\x01\x01\0\0\0\0\0\0\0\0\0\0\x01\t\t\t\0\t\t\0\0\0\0\0\t\t\t\t\x01\x01\x01\x01\x01\x01\0\0\t\t\t\0\0\0\0\t\x01\x01\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\t\t\t\t\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\t\t\0\0\0\x01\x01\t\t\t\t\0\0\0\0\0\0\0\0\0\0\t\x01\x01\t\t\x01\0\0\0\0\0\0\0\0\0\t\t\t\t\t\t\t\t\t\t\t\0\0\x02\0\0\t\0\0\0\0\0\0\0\0\0\0\x02\0\0\0\0\x01\t\t\x01\0\0\0\0\0\0\0\0\x01\x01\x01\t\0\0\x01\0\0\0\0\0\0\0\0\0\t\x01\x01\x01\x01\0\0\0\0\t\t\t\t\0\t\t\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x01\0\x01\0\0\0\t\t\t\t\t\t\t\t\0\0\0\0\0\0\t\x01\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\0\x01\0\x01\x01\x01\x01\0\x01\t\t\t\t\0\x01\x01\x01\x01\x01\x01\x01\x01\0\0\x01\0\x01\x01\0\x01\x01\x01\x01\x01\0\t\t\x01\t\t\x01\0\0\0\0\0\0\t\0\0\0\0\0\x01\x01\x01\t\t\0\0\t\t\t\t\t\t\t\0\0\0\t\t\t\t\t\t\t\x01\x01\x01\x01\0\0\0\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\0\0\0\t\x01\t\t\t\t\x01\x01\0\x01\0\0\0\0\0\0\0\0\t\t\t\t\t\t\0\0\t\t\t\t\t\t\t\t\0\0\0\0\0\0\0\0\x01\x01\x01\x01\t\t\0\0\0\x01\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\0\0\x01\0\0\x01\x01\x01\x01\t\t\t\t\t\t\0\t\t\0\0\t\t\t\t\x01\t\t\0\0\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\0\0\x01\x01\x01\x01\x01\x01\t\t\t\t\t\t\t\0\0\t\t\t\t\t\t\x01\0\x01\t\0\0\0\0\0\0\0\0\0\0\0\x01\t\t\t\t\t\t\t\t\t\t\x01\x01\x01\x01\x01\t\t\t\t\t\t\t\x01\t\t\t\t\0\x01\t\t\t\t\t\t\t\t\t\t\t\x01\x01\x01\x01\t\t\t\t\t\t\t\t\t\t\0\0\0\x01\0\0\x01\t\t\t\t\t\t\0\0\0\t\0\t\t\0\t\t\t\t\t\t\x01\t\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\t\t\t\0\t\t\t\t\t\x01\0\0\0\0\0\0\0\x01\x01\x01\t\t\t\t\0\0\0\0\0\0\0\0\0\t\t\x01\t\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\t\t\t\t\t\t\0\0\0\t\t\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\x02\t\x01\x01\x01\x01\x01\x01\t\t\t\t\t\t\t\t\t\0\0\0\0\0\0\0\t\x01\x01\0\x01\t\0\0\0\0\0\0\0\0\0\0\0\x03\x03\x03\x03\0\x03\x03\x03\x03\x03\x03\x03\0\x03\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x03\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\0\0\0\0\0\0\0\0\0\0\x03\x03\x03\x03\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\0\0\t\t\0\x02\x02\x02\x02\0\0\0\0\0\0\0\0\0\0\0\0\t\t\t\t\t\0\0\0\t\t\t\x02\x02\x02\x02\x02\x02\x02\x02\t\t\t\t\t\0\0\t\t\t\t\t\t\t\0\0\0\0\t\t\t\0\0\0\0\0\0\0\0\0\0\0\x01\0\0\x01\x01\0\0\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\0\x01\0\0\0\x01\x01\x01\x01\x01\x01\0\0\x01\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\t\t\t\t\t\t\t\0\0\0\0\t\t\t\t\t\0\t\t\0\t\t\t\t\t\0\0\0\0\0\t\t\t\t\t\t\t\x01\x01\x01\x01\x01\x01\x01\0\0\x06\x06\x06\x06\x06\x06\x06\x06\x06\x06\0\0\0\0\x01\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\x01\t\0\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\0\x01\x01\0\x01\x01\x01\x01\t\t\t\t\t\t\t\x01\0\0\0\0\x01\x01\0\x01\0\0\x01\0\x01\x01\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\0\x01\0\x01\0\0\0\0\x01\0\0\0\0\x01\0\x01\0\x01\0\x01\x01\x01\0\x01\x01\0\x01\0\0\x01\0\x01\0\x01\0\x01\0\x01\x01\0\x01\0\0\x01\x01\x01\x01\0\x01\x01\x01\x01\0\x01\x01\x01\x01\0\x01\0\x01\x01\x01\0\x01\x01\x01\x01\x01\0\x01\x01\x01\x01\x01\0\0\0\0\0\0\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\r\0\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, icu::properties::WordBreak(0u8)));
        }
        #[clippy::msrv = "1.66"]
        impl icu_provider::DataProvider<icu::properties::provider::WordBreakV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::WordBreakV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_WB_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::WordBreakV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
