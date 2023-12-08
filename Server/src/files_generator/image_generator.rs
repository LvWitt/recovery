use nalgebra::DVector;
pub enum ImageSize {
    Small,
    Medium,
}

pub fn create_img(vector: DVector<f64>, size: ImageSize, filename: u32) {
    let imgx: u32;
    let imgy: u32;
    match size {
        ImageSize::Small => {
            imgx = 30;
            imgy = 30;
        }
        ImageSize::Medium => {
            imgx = 60;
            imgy = 60;
        }
    }

    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(imgx, imgy);

    for i in 0..imgx {
        for j in 0..imgy {
            let img_pixel = imgbuf.get_pixel_mut(i, j);
            let index: usize = ((i * imgx) + j) as usize;
            let pixel_value = vector.get(index).unwrap();
            let r = (pixel_value * 255.0) as u8;
            *img_pixel = image::Rgb([r, r, r]);
        }
    }
    let path = format!("image_output/{:?}.png", filename);
    let _ = imgbuf.save(path);
}
