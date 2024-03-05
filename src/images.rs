use crate::config::GeneralConfig;
use std::path::Path;

use rocket::State;
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;


/*
 * Images and Image manilulation
 */
extern crate image;

use image::io::Reader as ImageReader;
use image::imageops::FilterType;

// e.g. http://localhost:9000/images/ART-10001/IMG_1967_1000.JPG
#[get("/images/<product>/<image>/<width>")]
pub async fn get_product_image(config: &State<GeneralConfig>, product: &str, image: &str, width: u32) -> Result<NamedFile, NotFound<String>> {
    let source = Path::new(&config.data_path).join(&product).join("Images").join(&image);
    let destination_file = format!("/tmp/{}_{}_{}", &product, width, &image);
    let destination = Path::new(&destination_file);

    if !destination.exists() {
        if !source.exists() {
            return Err(NotFound(product.to_string()));
        }
        image_resize_to_width(&source, destination, width);
    }
    NamedFile::open(&destination).await.map_err(|e| NotFound(e.to_string()))
}


fn image_resize_to_width(source: &Path, destination: &Path, width: u32) {
    let input = ImageReader::open(source).unwrap().decode().unwrap();
    let factor: f32 = width as f32 / input.width() as f32;
    let height: u32 = (input.height() as f32 * factor).ceil() as u32;
   
    let scaled =  input.resize(width, height, FilterType::Gaussian);
    let status = scaled.save(destination);

   if status.is_err() {
       println!("{:?}", status.unwrap_err())
   }
}



#[cfg(test)]
mod images_service_tests {
    use super::*;

    #[test]
    fn image_resize_to_width_test() {
        let source = Path::new("./test_data/Ilovetrash.png");
        let destination = Path::new("./test_data/Ilovetrash_scaled.png");
        let width = 100;

        super::image_resize_to_width(source, destination, width);

        let input = ImageReader::open(destination).unwrap().decode().unwrap();
        assert_eq!(width, input.width());
        assert!(std::fs::remove_file(destination).is_ok());      
    }
}
