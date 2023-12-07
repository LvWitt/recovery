use nalgebra::DVector;

pub fn create_img(vector: DVector<f64>, size:u32) {
    let imgx = size;
    let imgy = size;

    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(imgx, imgy);

    for i in 0..imgx {
        for j in 0..imgy {
            let img_pixel = imgbuf.get_pixel_mut(i, j);
            let index: usize = ((i * imgx) + j) as usize;
            let pixel_value = vector.get(index).unwrap();
            let r = (pixel_value * 255.0) as u8;
            *img_pixel = image::Rgb([r, 0, 0]);
        }
    }

    let _ = imgbuf.save("MEUFILHO.png");
}