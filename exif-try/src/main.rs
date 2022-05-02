use exif::Tag;

fn main() {
    let file = std::fs::File::open("test.jpg").unwrap();
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();
    let original = &Tag::DateTimeOriginal;
    let digitizied = &Tag::DateTimeDigitized;
    let modified = &Tag::DateTime;
    for f in exif.fields() {
        let tag = &f.tag;
        if *tag == *original || *tag == *digitizied || *tag == *modified {
            println!(
                "{} {} {}",
                f.tag,
                f.ifd_num,
                f.display_value().with_unit(&exif)
            );
        }
    }
}
