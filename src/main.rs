use log::{info};
use env_logger::Builder;
use log::LevelFilter;
mod bn64;
use bn64::Bn64;

fn main() {
    Builder::new()
            .filter_level(LevelFilter::Info)
            .init();
/*
    let mut bn0: Bn64 = Bn64::from(String::from("0123456789abcdef0123456789abcdef123"));
    bn0.to_hex();
*/
    left_push();
}

fn left_push(){
    let v:u64 = 0x1;
    for index in 0..70{
        let (v, b) = v.overflowing_sub(index);
        info!("{v}, {b}");
    }
}
