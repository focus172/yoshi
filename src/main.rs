use image::{io::Reader as ImageReader, GenericImageView, Pixel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_path = "/home/focus/code/yoshi/test.png";
    let image = ImageReader::open(image_path)?.decode()?;

    let mut buffer = String::new();
    let mut ascii: Vec<Line> = Vec::new();
    let image_width = image.width();
    image.pixels().for_each(|pixel| {
        let index = &pixel.0;
        let pix = pixel.2.to_rgba();
        let rgba = pix.channels();
        let brightness = (rgba[0] as u16 + rgba[1] as u16 + rgba[2] as u16) / 3;
        // scale used @B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. 
        let character = match brightness as u8 {
            0..=10 => "@",
            11..=20 => "%",
            21..=30 => "&",
            31..=40 => "M",
            41..=50 => "*",
            51..=60 => "a",
            61..=70 => "k",
            71..=80 => "d",
            81..=90 => "q",
            91..=100 => "m",
            101..=110 => "0",
            111..=120 => "L",
            121..=130 => "J",
            131..=140 => "Y",
            141..=160 => "z",
            161..=180 => "f",
            181..=200 => "1",
            201..=210 => "i",
            211..=220 => "`",
            221..=255 => " ",
        };

        buffer.push_str(character);
        if index+1 == image_width {
            ascii.push(Line { text: buffer.clone() });
            buffer.clear();
        }
    });

    for line in ascii {
        println!("{}", line.text);
    }
    Ok(())
}

struct Line {
    text: String,
}
