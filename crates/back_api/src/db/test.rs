use shared_lib::primitives::frozen::implements::{CompInn, Kpp};
use crate::db::parsers::dadata::inn_kpp_query::parse_company_by_inn_kpp;

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use shared_lib::primitives::frozen::implements::{Kpp, CompInn};
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::time::{SystemTime, UNIX_EPOCH};
    use uuid::Uuid;
    use chrono::{Utc, Local, DateTime}; // Обязательно добавь chrono в Cargo.toml, если его там нет

    #[derive(Serialize)]
    struct JwtHeader {
        alg: String,
        typ: String,
    }

    #[derive(Serialize)]
    struct JwtPayload {
        iss: String,
        sub: String,
        aud: String,
        iat: u64,
        exp: u64,
        state: String,
    }

    async fn test_dadata() {
        let inn = CompInn::unchecked("161101510882");
        let kpp = Kpp::unchecked("");
        

    }

    fn base64url_encode<T: Serialize>(data: &T) -> String {
        let json_bytes = serde_json::to_vec(data).unwrap();
        let config = base64::engine::general_purpose::URL_SAFE_NO_PAD;
        base64::Engine::encode(&config, json_bytes)
    }

    #[test]
    fn generate_esia_jwt_test() -> Result<(), Box<dyn std::error::Error>> {
        let mnemonic = "GOSKEY_XPINAT_1".to_string();
        let esia_aud = "https://esia-portal1.test.gosuslugi.ru/aas/oauth2/te".to_string();
        
        let base_dir = "/home/khabipovrinat/dev/fin_app_full";
        let unsigned_path = format!("{}/unsigned_jwt.txt", base_dir);
        let signature_path = format!("{}/unsigned_jwt.txt.p7s", base_dir);
        let final_path = format!("{}/client_assertion.txt", base_dir);

        // 1. Получаем текущее время
        let start = SystemTime::now();
        let iat = start.duration_since(UNIX_EPOCH)?.as_secs();
        let exp = iat + 1800; // 30 минут

        // Для параметра timestamp в curl нам нужен точный формат "yyyy.MM.dd HH:mm:ss Z"
        // Используем локальное время машины (или UTC, ЕСИА принимает оба, если таймзона указана верно)
        let now: DateTime<Local> = Local::now();
        let timestamp_str = now.format("%Y.%m.%d %H:%M:%S %z").to_string();
        let state_str = Uuid::new_v4().to_string();

        // 2. Формируем структуры (state зашиваем фиксированный для этого запуска)
        let header = JwtHeader {
            alg: "GOST3410_2012_256".to_string(), // Вот он, наш золотой ключ!
            typ: "JWT".to_string(),
        };

        let payload = JwtPayload {
            iss: mnemonic.clone(),
            sub: mnemonic.clone(),
            aud: esia_aud.clone(),
            iat,
            exp,
            state: state_str.clone(),
        };

        // 3. Кодируем в Base64URL
        let header_b64 = base64url_encode(&header);
        let payload_b64 = base64url_encode(&payload);
        let unsigned_jwt = format!("{}.{}", header_b64, payload_b64);

        // 4. Записываем файл для подписи
        let mut file = File::create(&unsigned_path)?;
        file.write_all(unsigned_jwt.as_bytes())?;
        
        println!("\n==================================================");
        println!(" [OK] Файл для подписи создан: {}", unsigned_path);
        println!("==================================================");
        println!("Подпиши файл вручную в соседнем терминале.");
        println!("После появления файла подписи нажмите ENTER...");
        println!("==================================================");

        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // 5. Читаем и кодируем подпись
        let sig_bytes = fs::read(&signature_path)?;
        let config = base64::engine::general_purpose::URL_SAFE_NO_PAD;
        let sig_b64url = base64::Engine::encode(&config, sig_bytes);

        // 6. Склеиваем и сохраняем финальный результат
        let final_jwt = format!("{}.{}", unsigned_jwt, sig_b64url);
        let mut final_file = File::create(&final_path)?;
        final_file.write_all(final_jwt.as_bytes())?;

        // 7. Генерируем готовую команду curl!
        println!("\n==================================================");
        println!(" [SUCCESS] Токен собран!");
        println!("==================================================");
        println!("Скопируй и запусти эту команду в терминале:\n");
        
        println!(
            "curl -X POST \"{}\" \\\n\
             -H \"Content-Type: application/x-www-form-urlencoded\" \\\n\
             --data-urlencode \"grant_type=client_credentials\" \\\n\
             --data-urlencode \"client_id={}\" \\\n\
             --data-urlencode \"client_assertion_type=urn:ietf:params:oauth:client-assertion-type:jwt-bearer\" \\\n\
             --data-urlencode \"client_assertion={}\" \\\n\
             --data-urlencode \"scope=http://esia.gosuslugi.ru/usr_inf\" \\\n\
             --data-urlencode \"state={}\" \\\n\
             --data-urlencode \"timestamp={}\"",
            esia_aud, mnemonic, final_jwt, state_str, timestamp_str
        );
        println!("\n==================================================");

        Ok(())
    }
}