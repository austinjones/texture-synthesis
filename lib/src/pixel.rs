use image::{DynamicImage, ImageBuffer, Pixel};

pub type SynthPixelBuffer<P> = ImageBuffer<P, Vec<u8>>;
pub type Rgb = image::Rgb<u8>;
pub type Rgba = image::Rgba<u8>;
pub type Luma = image::Luma<u8>;
pub type LumaA = image::LumaA<u8>;

// nice color scaling method: http://eastfarthing.com/blog/2015-12-19-color/
// used here because we want symmetric mapping

#[inline(always)]
fn u8_to_f32(x: u8) -> f32 {
    (x as f32 + 0.5f32) / 256.0
}

#[inline(always)]
fn f32_to_u8(x: f32) -> u8 {
    (256.0 * x) as u8
}

pub trait SynthPixel: Pixel<Subpixel = u8> + Copy + Send + Sync + 'static {
    fn into_dynamic_image(buffer: SynthPixelBuffer<Self>) -> DynamicImage;
    fn from_dynamic_image(image: DynamicImage) -> SynthPixelBuffer<Self>;
    fn colors(&self) -> &[u8];
    fn alpha(&self) -> Option<u8>;
    fn color_len() -> usize;
    fn greyscale(amt: u8) -> Self;

    /// Write our *color data* into the provided slice
    /// We have to multiply color data by alphas!
    /// Otherwise we'd consider a transparent bright green to be the same as visible bright green
    fn write(&self, slice: &mut [u8]);
}

impl SynthPixel for Rgba {
    fn into_dynamic_image(buffer: SynthPixelBuffer<Self>) -> DynamicImage {
        DynamicImage::ImageRgba8(buffer)
    }

    fn from_dynamic_image(image: DynamicImage) -> SynthPixelBuffer<Self> {
        image.to_rgba()
    }

    #[inline(always)]
    fn colors(&self) -> &[u8] {
        &self.0[0..3]
    }

    #[inline(always)]
    fn alpha(&self) -> Option<u8> {
        Some(self.0[3])
    }

    #[inline(always)]
    fn color_len() -> usize {
        4
    }

    #[inline(always)]
    fn greyscale(amt: u8) -> Self {
        image::Rgba([amt, amt, amt, 255])
    }

    #[inline(always)]
    fn write(&self, slice: &mut [u8]) {
        let alpha = u8_to_f32(self.0[3]);
        let r = alpha * u8_to_f32(self.0[0]);
        let g = alpha * u8_to_f32(self.0[0]);
        let b = alpha * u8_to_f32(self.0[0]);

        let target = &mut slice[0..3];
        target[0] = f32_to_u8(r);
        target[1] = f32_to_u8(g);
        target[2] = f32_to_u8(b);
    }
}

impl SynthPixel for Rgb {
    fn into_dynamic_image(buffer: SynthPixelBuffer<Self>) -> DynamicImage {
        DynamicImage::ImageRgb8(buffer)
    }

    fn from_dynamic_image(image: DynamicImage) -> SynthPixelBuffer<Self> {
        image.to_rgb()
    }

    #[inline(always)]
    fn colors(&self) -> &[u8] {
        &self.0[0..3]
    }

    #[inline(always)]
    fn alpha(&self) -> Option<u8> {
        None
    }

    #[inline(always)]
    fn color_len() -> usize {
        3
    }

    #[inline(always)]
    fn greyscale(amt: u8) -> Self {
        image::Rgb([amt, amt, amt])
    }

    #[inline(never)]
    fn write(&self, slice: &mut [u8]) {
        let target = &mut slice[0..3];
        target[0] = self.0[0];
        target[1] = self.0[1];
        target[2] = self.0[2];
    }
}

impl SynthPixel for LumaA {
    fn into_dynamic_image(buffer: SynthPixelBuffer<Self>) -> DynamicImage {
        DynamicImage::ImageLumaA8(buffer)
    }

    fn from_dynamic_image(image: DynamicImage) -> SynthPixelBuffer<Self> {
        image.to_luma_alpha()
    }

    #[inline(always)]
    fn colors(&self) -> &[u8] {
        &self.0[0..1]
    }

    #[inline(always)]
    fn alpha(&self) -> Option<u8> {
        Some(self.0[1])
    }

    #[inline(always)]
    fn color_len() -> usize {
        1
    }

    #[inline(always)]
    fn greyscale(amt: u8) -> Self {
        image::LumaA([amt, 255])
    }

    #[inline(always)]
    fn write(&self, slice: &mut [u8]) {
        let alpha = u8_to_f32(self.0[1]);
        let luma = alpha * u8_to_f32(self.0[0]);
        slice[0] = f32_to_u8(luma);
    }
}

impl SynthPixel for Luma {
    fn into_dynamic_image(buffer: SynthPixelBuffer<Self>) -> DynamicImage {
        DynamicImage::ImageLuma8(buffer)
    }

    #[inline(always)]
    fn from_dynamic_image(image: DynamicImage) -> SynthPixelBuffer<Self> {
        image.to_luma()
    }

    #[inline(always)]
    fn colors(&self) -> &[u8] {
        &self.0[0..1]
    }

    #[inline(always)]
    fn alpha(&self) -> Option<u8> {
        None
    }

    #[inline(always)]
    fn color_len() -> usize {
        1
    }

    #[inline(always)]
    fn greyscale(amt: u8) -> Self {
        image::Luma([amt])
    }

    #[inline(always)]
    fn write(&self, slice: &mut [u8]) {
        slice[0] = self.0[0];
    }
}
