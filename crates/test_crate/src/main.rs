use quick_xml::de::from_str;
use std::fs;
use shared_lib::parsers::mchd::poa::PoaMchd;



fn main() {
    let path = "/home/khabipovrinat/dev/fin_app_full/crates/test_crate/org_person_text.xml";

    let xml_data = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("MissFile!"));

    match from_str::<PoaMchd>(&xml_data) {
        Ok(mchd) => {
            let json_mchd = serde_json
                ::to_string_pretty(&mchd)
                .unwrap();
            std::println!("{json_mchd}");
        }
        Err(err) => {std::println!("{err:?}");}
    }
}
