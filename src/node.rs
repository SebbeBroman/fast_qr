use crate::{smol_png, ECL, QRCode, Version};
use napi::bindgen_prelude::{Buffer, Result};
use napi_derive::napi;

#[napi(object)]
pub struct SmolImageOptions {
    pub scale: Option<u32>,
    #[napi(js_name = "quietZone")]
    pub quiet_zone: Option<u32>,
    pub version: Option<i32>,
    pub ecl: Option<i32>,
}

fn parse_ecl(value: i32) -> Option<ECL> {
    match value {
        0 => Some(ECL::L),
        1 => Some(ECL::M),
        2 => Some(ECL::Q),
        3 => Some(ECL::H),
        _ => None,
    }
}

fn parse_version(value: i32) -> Option<Version> {
    match value {
        0 => Some(Version::V01),
        1 => Some(Version::V02),
        2 => Some(Version::V03),
        3 => Some(Version::V04),
        4 => Some(Version::V05),
        5 => Some(Version::V06),
        6 => Some(Version::V07),
        7 => Some(Version::V08),
        8 => Some(Version::V09),
        9 => Some(Version::V10),
        10 => Some(Version::V11),
        11 => Some(Version::V12),
        12 => Some(Version::V13),
        13 => Some(Version::V14),
        14 => Some(Version::V15),
        15 => Some(Version::V16),
        16 => Some(Version::V17),
        17 => Some(Version::V18),
        18 => Some(Version::V19),
        19 => Some(Version::V20),
        20 => Some(Version::V21),
        21 => Some(Version::V22),
        22 => Some(Version::V23),
        23 => Some(Version::V24),
        24 => Some(Version::V25),
        25 => Some(Version::V26),
        26 => Some(Version::V27),
        27 => Some(Version::V28),
        28 => Some(Version::V29),
        29 => Some(Version::V30),
        30 => Some(Version::V31),
        31 => Some(Version::V32),
        32 => Some(Version::V33),
        33 => Some(Version::V34),
        34 => Some(Version::V35),
        35 => Some(Version::V36),
        36 => Some(Version::V37),
        37 => Some(Version::V38),
        38 => Some(Version::V39),
        39 => Some(Version::V40),
        _ => None,
    }
}

fn normalize_options(options: Option<SmolImageOptions>) -> SmolImageOptions {
    options.unwrap_or(SmolImageOptions {
        scale: Some(1),
        quiet_zone: Some(4),
        version: None,
        ecl: None,
    })
}

fn encode_smol_image(content: &str, options: &SmolImageOptions) -> Vec<u8> {
    QRCode::new(
        content.as_bytes(),
        options.ecl.and_then(parse_ecl),
        options.version.and_then(parse_version),
        None,
        None,
    )
    .map(|qrcode| smol_png::encode(&qrcode, options.scale.unwrap_or(1), options.quiet_zone.unwrap_or(4)))
    .unwrap_or_default()
}

#[napi(js_name = "qrSmolImage")]
pub fn qr_smol_image(content: String, options: Option<SmolImageOptions>) -> Result<Buffer> {
    let options = normalize_options(options);
    Ok(encode_smol_image(&content, &options).into())
}
