use chrono::prelude::*;
use chrono::DateTime;
use urlencoding::encode;
use urlencoding::decode;

fn main() {
    // system time
    let sys_utc: DateTime<Utc> = Utc::now();
    let sys_utc_str = sys_utc.to_rfc3339_opts(SecondsFormat::Millis, true);
    println!("sys_utc_str = {}", sys_utc_str);

    let encoded = encode(&sys_utc_str);
    println!("sys_utc_str urlencoded = {}", encoded);

    let decoded = decode(&encoded).unwrap();
    println!("sys_utc_str decoded = {}", decoded);


    // sample timestamp
    let sample_str = "2018-01-26T18:30:09.453Z";
    let dt = DateTime::parse_from_str(sample_str, "%+").unwrap();
    let dt_str = dt.to_rfc3339_opts(SecondsFormat::Millis, true);
    println!("sample_str = {} parsed and formated as => dt_str = {}", sample_str, dt_str);

    let encoded = encode(&dt_str);
    println!("sample_str urlencoded = {}", encoded);

    let decoded = decode(&encoded).unwrap();
    println!("sample_str decoded = {}", decoded);
}
