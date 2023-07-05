use pngme::png::Png;

fn main() {
    let bytes = include_bytes!("../image.png");

    let png = Png::try_from(&bytes[..]).unwrap();

    println!("{png}");
}
