use logger::Voyager;

fn main() {
    // önce loglayıcıyı oluşturalım
    env_logger::init();

    let mut gemini = Voyager::new(String::from("Gemini"));
    println!("{}\n", gemini.nickname);
    gemini.connect(String::from("Andromeda"));

    for _ in 0..3 {
        println!("{:?}", gemini);
        gemini.hited();
    }

    gemini.life = 1;
    gemini.connect(String::from("Orion"));
}

/* # önerileri alıp kodu toparlamak için
cargo clippy
# library'nin başarılı şekilde build olup olmadığını görmek için
cargo build --lib

# varsayılan çalıştırmada sadece ERROR Logları görünür
cargo run

# log çıktılarını okumak için farklı yollar kullanabiliriz.
# warn ve error mesajlarını gösterir
RUST_LOG=warn cargo run

# trace ile birlikte debug,warn,info,error mesajlarını alırız
RUST_LOG=trace cargo run */
