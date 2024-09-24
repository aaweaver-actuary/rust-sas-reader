use crate::types::sas::sas_enums::SasStringEncoding;
use std::collections::HashMap;

// Define the charset table as a constant array
pub fn build_charset_lookup() -> HashMap<u32, String> {
    let mut lookup = HashMap::new();

    lookup.insert(0, SasStringEncoding::default().to_string());
    lookup.insert(20, "UTF-8".to_string());
    lookup.insert(28, "US-ASCII".to_string());
    lookup.insert(29, "ISO-8859-1".to_string());
    lookup.insert(30, "ISO-8859-2".to_string());
    lookup.insert(31, "ISO-8859-3".to_string());
    lookup.insert(32, "ISO-8859-4".to_string());
    lookup.insert(33, "ISO-8859-5".to_string());
    lookup.insert(34, "ISO-8859-6".to_string());
    lookup.insert(35, "ISO-8859-7".to_string());
    lookup.insert(36, "ISO-8859-8".to_string());
    lookup.insert(37, "ISO-8859-9".to_string());
    lookup.insert(39, "ISO-8859-11".to_string());
    lookup.insert(40, "ISO-8859-15".to_string());
    lookup.insert(41, "CP437".to_string());
    lookup.insert(42, "CP850".to_string());
    lookup.insert(43, "CP852".to_string());
    lookup.insert(44, "CP857".to_string());
    lookup.insert(45, "CP858".to_string());
    lookup.insert(46, "CP862".to_string());
    lookup.insert(47, "CP864".to_string());
    lookup.insert(48, "CP865".to_string());
    lookup.insert(49, "CP866".to_string());
    lookup.insert(50, "CP869".to_string());
    lookup.insert(51, "CP874".to_string());
    lookup.insert(52, "CP921".to_string());
    lookup.insert(53, "CP922".to_string());
    lookup.insert(54, "CP1129".to_string());
    lookup.insert(55, "CP720".to_string());
    lookup.insert(56, "CP737".to_string());
    lookup.insert(57, "CP775".to_string());
    lookup.insert(58, "CP860".to_string());
    lookup.insert(59, "CP863".to_string());
    lookup.insert(60, "WINDOWS-1250".to_string());
    lookup.insert(61, "WINDOWS-1251".to_string());
    lookup.insert(62, "WINDOWS-1252".to_string());
    lookup.insert(63, "WINDOWS-1253".to_string());
    lookup.insert(64, "WINDOWS-1254".to_string());
    lookup.insert(65, "WINDOWS-1255".to_string());
    lookup.insert(66, "WINDOWS-1256".to_string());
    lookup.insert(67, "WINDOWS-1257".to_string());
    lookup.insert(68, "WINDOWS-1258".to_string());
    lookup.insert(69, "MACROMAN".to_string());
    lookup.insert(70, "MACARABIC".to_string());
    lookup.insert(71, "MACHEBREW".to_string());
    lookup.insert(72, "MACGREEK".to_string());
    lookup.insert(73, "MACTHAI".to_string());
    lookup.insert(75, "MACTURKISH".to_string());
    lookup.insert(76, "MACUKRAINE".to_string());
    lookup.insert(118, "CP950".to_string());
    lookup.insert(119, "EUC-TW".to_string());
    lookup.insert(123, "BIG-5".to_string());
    lookup.insert(125, "GB18030".to_string());
    lookup.insert(126, "WINDOWS-936".to_string());
    lookup.insert(128, "CP1381".to_string());
    lookup.insert(134, "EUC-JP".to_string());
    lookup.insert(136, "CP949".to_string());
    lookup.insert(137, "CP942".to_string());
    lookup.insert(138, "CP932".to_string());
    lookup.insert(140, "EUC-KR".to_string());
    lookup.insert(141, "CP949".to_string());
    lookup.insert(142, "CP949".to_string());
    lookup.insert(163, "MACICELAND".to_string());
    lookup.insert(167, "ISO-2022-JP".to_string());
    lookup.insert(168, "ISO-2022-KR".to_string());
    lookup.insert(169, "ISO-2022-CN".to_string());
    lookup.insert(172, "ISO-2022-CN-EXT".to_string());
    lookup.insert(204, SasStringEncoding::default().to_string());
    lookup.insert(205, "GB18030".to_string());
    lookup.insert(227, "ISO-8859-14".to_string());
    lookup.insert(242, "ISO-8859-13".to_string());
    lookup.insert(245, "MACCROATIAN".to_string());
    lookup.insert(246, "MACCYRILLIC".to_string());
    lookup.insert(247, "MACROMANIA".to_string());
    lookup.insert(248, "SHIFT_JISX0213".to_string());

    lookup
}
