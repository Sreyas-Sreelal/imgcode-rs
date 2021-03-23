use image::io::Reader;
use image::DynamicImage;

pub struct RasterImage {
    image_handle: DynamicImage,
}

impl RasterImage {
    pub fn new(dir: &str) -> Result<RasterImage, Box<dyn std::error::Error>> {
        let image_handle = Reader::open(dir)?.decode()?;
        Ok(Self { image_handle })
    }
    pub fn gray_scale(self) -> DynamicImage {
        self.image_handle.grayscale()
    }
}
