use crate::pixel::{SynthPixel, SynthPixelBuffer};
use image::{ImageBuffer, Pixel};

#[derive(Clone)]
pub struct ImagePyramid<P: SynthPixel> {
    pub pyramid: Vec<SynthPixelBuffer<P>>,
    pub levels: u32,
}

impl<P: SynthPixel> ImagePyramid<P> {
    pub fn new(in_img: SynthPixelBuffer<P>, levels: Option<u32>) -> Self {
        let lvls = levels.unwrap_or_else(|| {
            //auto-calculate max number of downsampling
            let (dimx, dimy) = in_img.dimensions();
            (f64::from(dimx.max(dimy))).log2() as u32 // pow(2, x) ~ img => x ~ log2(img)
        });

        Self {
            pyramid: ImagePyramid::build_gaussian(lvls, in_img),
            levels: lvls,
        }
    }

    //build gaussian pyramid by downsampling the image by 2
    fn build_gaussian(in_lvls: u32, in_img: SynthPixelBuffer<P>) -> Vec<SynthPixelBuffer<P>> {
        let mut imgs = Vec::new();
        let (dimx, dimy) = in_img.dimensions();

        //going from lowest to largest resolution (to match the texture synthesis generation order)
        for i in (1..in_lvls).rev() {
            let p = u32::pow(2, i);
            imgs.push(image::imageops::resize(
                &image::imageops::resize(&in_img, dimx / p, dimy / p, image::imageops::Gaussian),
                dimx,
                dimy,
                image::imageops::Gaussian,
            ));
        }

        imgs.push(in_img);
        imgs
    }

    pub fn bottom(&self) -> &SynthPixelBuffer<P> {
        &self.pyramid[self.levels as usize - 1]
    }
}
