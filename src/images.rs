use crate::config::GeneralConfig;
use std::path::Path;


use rocket::State;
use rocket::response::status::NotFound;
use rocket::response::NamedFile;

/*
 * Images and Image manilulation
 */
extern crate image_convert;

use image_convert::{ImageResource, JPGConfig, to_jpg};


// e.g. http://localhost:9000/images/ART-10001/IMG_1967_1000.JPG
#[get("/images/<product>/<image>/<width>")]
pub fn get_product_image(config: State<GeneralConfig>, product: String, image: String, width: u16) -> Result<NamedFile, NotFound<String>> {
   let source = Path::new(&config.data_path).join(&product).join("Images").join(&image);
   let destination_file = format!("/tmp/{}_{}_{}", &product, width, &image);
   let destination = Path::new(&destination_file);

   if !destination.exists() {
       if !source.exists() {
           return Err(NotFound(product));
       }
       image_resize_to_width(&source, &destination, width);
   }
   NamedFile::open(&destination).map_err(|e| NotFound(e.to_string()))
}


fn image_resize_to_width(source: &Path, destination: &Path, width: u16) {
   let mut config = JPGConfig::new();
   config.width = width;

   let input = ImageResource::from_path(source);
   let mut output = ImageResource::from_path(destination);
   
   let status = to_jpg(&mut output, &input, &config);

   if status.is_err() {
       println!("{:?}", status.unwrap_err())
   }
}