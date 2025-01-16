use image::DynamicImage;
use image::GenericImageView;
use image::{ImageReader, imageops::FilterType};
use ravif::RGBA8 as Rgba;
use ravif::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use webp::Encoder as WebpEncoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the input JPG image
    let input_path = "js-fw-meme.png";
    let img = ImageReader::open(input_path)?.decode()?;

    // just_resize();

    // Convert and save as WebP
    convert_to_webp(&img, "output.webp")?;

    // Convert and save as AVIF
    convert_to_avif(&img, "output.avif")?;

    println!("Conversion completed!");
    Ok(())
}

fn convert_to_webp(
    img: &DynamicImage,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let encoder = WebpEncoder::from_image(img).unwrap();
    let webp_data = encoder.encode(75.0); // Quality 75
    let mut file = File::create(output_path)?;
    file.write_all(&webp_data)?;
    println!("Saved WebP to {}", output_path);
    Ok(())
}

fn convert_to_avif(
    img: &DynamicImage,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = img.dimensions();
    let max_width = 650;
    let max_height = (width as f32 * (max_width as f32 / height as f32)).round() as u32;
    let resized = img.resize(max_width, max_height, FilterType::Lanczos3);
    let (width, height) = resized.dimensions();
    let rgba = resized.to_rgba8();
    println!("Resized to {}x{}", width, height);
    let encoded_avif = Encoder::new()
        .with_quality(50.0)
        .with_alpha_quality(50.0)
        .with_speed(10)
        .with_alpha_color_mode(AlphaColorMode::UnassociatedClean)
        .with_num_threads(Some(4));
    let avif_pixels = rgba
        .pixels()
        .map(|p| Rgba {
            r: p[0],
            g: p[1],
            b: p[2],
            a: p[3],
        })
        .collect::<Vec<Rgba>>();

    let EncodedImage {
        avif_file,
        color_byte_size,
        alpha_byte_size,
        ..
    } = encoded_avif
        .encode_rgba(Img::new(
            &avif_pixels,
            width.try_into().unwrap(),
            height.try_into().unwrap(),
        ))
        .unwrap();
    let mut file = File::create(output_path)?;
    file.write_all(&avif_file)?;
    println!("Saved AVIF to {}", output_path);
    Ok(())
}

fn just_resize() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = "logo.jpg";
    let output_path = "output_jpg.avif";

    // Target width
    let target_width = 650;

    // Load the image
    let img = ImageReader::open(input_path)?.decode()?;

    // Calculate the target height while maintaining the aspect ratio
    let (orig_width, orig_height) = img.dimensions();
    let target_height =
        (orig_height as f32 * (target_width as f32 / orig_width as f32)).round() as u32;

    // Resize the image using a high-quality filter
    let resized_img = img.resize(target_width, target_height, FilterType::Lanczos3);

    // Save the resized image
    // resized_img.save(output_path)?;

    let rgba = img.to_rgba8();
    println!(
        "Image resized to {}x{} and saved as {}",
        target_width, target_height, output_path
    );
    let encoded_avif = Encoder::new()
        .with_quality(50.0)
        .with_alpha_quality(50.0)
        .with_speed(10)
        .with_alpha_color_mode(AlphaColorMode::UnassociatedClean)
        .with_num_threads(Some(4));
    let avif_pixels = rgba
        .pixels()
        .map(|p| Rgba {
            r: p[0],
            g: p[1],
            b: p[2],
            a: p[3],
        })
        .collect::<Vec<Rgba>>();

    let EncodedImage {
        avif_file,
        color_byte_size,
        alpha_byte_size,
        ..
    } = encoded_avif
        .encode_rgba(Img::new(
            &avif_pixels,
            target_width.try_into().unwrap(),
            target_height.try_into().unwrap(),
        ))
        .unwrap();
    let mut file = File::create(output_path)?;
    file.write_all(&avif_file)?;
    println!("Saved AVIF to {}", output_path);
    Ok(())
}
