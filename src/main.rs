use std::collections::BTreeMap;

use combine::parser::char::newline;
use combine::stream::position;
use combine::EasyParser as _;
use combine::{many, many1, satisfy, sep_end_by, skip_many, token, Parser, Stream};

fn main() {
    let res = DriverInfo::parser()
        .easy_parse(position::Stream::new(ETHTOOL_DRIVER_OUTPUT))
        .unwrap();
    println!("{:#?}", res.0);
}

#[derive(Debug)]
pub struct DriverInfo {
    pub firmware_version: String,
    pub pci_bus_info: String,
}

impl DriverInfo {
    fn parser<I>() -> impl Parser<I, Output = Option<Self>>
    where
        I: Stream<Token = char>,
    {
        let line = || {
            (
                many1(satisfy(|c| c != ':')),
                token(':'),
                skip_many(token(' ')),
                many(satisfy(|c| c != '\n')),
            )
                .map(|(key, _, _, value)| (key, value))
        };
        sep_end_by(line(), newline())
            .map(|info: BTreeMap<String, String>| Self::from_ethtool_output(info))
    }

    fn from_ethtool_output(output: BTreeMap<String, String>) -> Option<Self> {
        Some(DriverInfo {
            firmware_version: output.get("firmware-version")?.to_owned(),
            pci_bus_info: output.get("bus-info")?.to_owned(),
        })
    }
}

const ETHTOOL_DRIVER_OUTPUT: &'static str = "driver: r8169
version: 6.1.69
firmware-version: rtl8168g-2_0.0.1 02/06/13
expansion-rom-version:
bus-info: 0000:06:00.0
supports-statistics: yes
supports-test: no
supports-eeprom-access: no
supports-register-dump: yes
supports-priv-flags: no
";
