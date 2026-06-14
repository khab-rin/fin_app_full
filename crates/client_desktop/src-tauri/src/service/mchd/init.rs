use std::fs;
use std::path::Path;

use shared_lib::Status;
use shared_lib::parsers::mchd::implements::*;
use shared_lib::parsers::mchd::poa::PoaMchd;


fn get_mchd_files<P: AsRef<Path>>(path: P) -> Result<PoaMchd, Status> {
    let xml_content_vec = match fs::read(path) {
        Ok(c) => c,
        Err(err) => {
            println!(
                "FUN get_mchd_files FAILED BY READ FILE, tech_err = {}, local_err = {}",
                err, Status::FileReadError
            );
            return Err(Status::FileReadError);
        }
    };

    let xml_content = match String::from_utf8(xml_content_vec) {
        Ok(x) => x, 
        Err(err) => {
            println!(
                "FUN get_mchd_files FAILED BY READING MCHD FILE, tech_err = {}, local_err = {}",
                err, Status::FileInvalideData
            );
            return Err(Status::FileInvalideData);
        }
    };

    let mchd: PoaMchd = match quick_xml::de::from_str(&xml_content) {
        Ok(m) => m,
        Err(err) =>  {
            log::error!(
                "FUN get_mchd_files FAILED BY READ FILE, tech_err = {}, local_err = {}",
                err, Status::MappingError
            );
            return Err(Status::MappingError);
        }
    };

    log::info!("page = {:?}", mchd);


    Ok(mchd)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mchd_example() {

        
        let poa: PoaMchd = get_mchd_files("/home/khabipovrinat/dev/fin_app_full/crates/client_desktop/src-tauri/src/service/mchd/ON_EMCHD_20260613_884f145c-2914-450c-a0d8-2a90e759b836.xml").expect("err");
    
    }
}

