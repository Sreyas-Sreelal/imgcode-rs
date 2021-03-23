use mask::convolution;

mod filter;
mod mask;
mod raster;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // just something like a driver code because i'm lazy to write a proper testing code
    // will subject to change soon smh!
    let img = raster::RasterImage::new("mylogo.png")?.gray_scale();
    let mask = filter::sobel_mask();
    let output = convolution(mask, img);
    output.save("output.png")?;
    Ok(())
}
