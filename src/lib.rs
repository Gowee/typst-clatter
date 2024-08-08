use std::{cell::LazyCell, collections::HashMap, str};

use rxing::{
    pdf417::PDF417Writer, BarcodeFormat, EncodeHintType, EncodeHintValue, EncodingHintDictionary,
    Writer,
};
use wasm_minimal_protocol::*;

mod utils;

use utils::parse_as_str;

initiate_protocol!();

#[wasm_func]
/// Generate a PDF417 in SVG format from a code text.
///
/// `width` and `height` are expected to be integers.
/// They are passed down to rxing which performs complicated scaling internaly.
/// Setting both of them to `0` or some small integer results in a SVG of the default scale.
/// The ratio of them determines whether the generated image is vertical or not.
pub fn pdf417(text: &[u8], width: &[u8], height: &[u8]) -> Vec<u8> {
    let width: i32 = parse_as_str!(width);
    let height: i32 = parse_as_str!(height);
    let hints: LazyCell<EncodingHintDictionary> = LazyCell::new(|| {
        HashMap::from([(
            EncodeHintType::MARGIN,
            EncodeHintValue::Margin(String::from("0")),
        )])
    });
    let encoded = PDF417Writer::new()
        .encode_with_hints(
            str::from_utf8(text).expect("text is valid utf-8"),
            &BarcodeFormat::PDF_417,
            width as i32,
            height as i32,
            &hints,
        )
        .expect("encoded");
    let svg: svg::Document = (&encoded).into();
    svg.to_string().as_bytes().to_vec()
}
