pub fn from_sjis(src: &[u8]) -> String {
    let (result, _, _) = encoding_rs::SHIFT_JIS.decode(src);
    result.to_string()
}

pub fn to_sjis(src: &str) -> Vec<u8> {
    let (result, _, _) = encoding_rs::SHIFT_JIS.encode(src);
    Vec::from(result)
}