use crate::pixel::{SynthPixel, SynthPixelBuffer};
use crate::Error;
use std::path::Path;

/// Helper type used to pass image data to the Session
#[derive(Clone)]
pub enum ImageSource<'a> {
    /// A raw buffer of image data, see `image::load_from_memory` for details
    /// on what is supported
    Memory(&'a [u8]),
    /// The path to an image to load from disk. The image format is inferred
    /// from the file extension, see `image::open` for details
    Path(&'a Path),
    /// An already loaded image that is passed directly to the generator
    Image(image::DynamicImage),
}

impl<'a> From<image::DynamicImage> for ImageSource<'a> {
    fn from(img: image::DynamicImage) -> Self {
        ImageSource::Image(img)
    }
}

impl<'a, S> From<&'a S> for ImageSource<'a>
where
    S: AsRef<Path> + 'a,
{
    fn from(path: &'a S) -> Self {
        Self::Path(path.as_ref())
    }
}

pub(crate) fn load_image<P: SynthPixel>(
    src: ImageSource<'_>,
    resize: Option<(u32, u32)>,
) -> Result<SynthPixelBuffer<P>, Error> {
    let img = match src {
        ImageSource::Memory(data) => image::load_from_memory(data),
        ImageSource::Path(path) => image::open(path),
        ImageSource::Image(img) => Ok(img),
    }?;

    Ok(match resize {
        None => P::from_dynamic_image(img),
        Some(ref size) => {
            use image::GenericImageView;

            if img.width() != size.0 || img.height() != size.1 {
                image::imageops::resize::<SynthPixelBuffer<P>>(
                    &P::from_dynamic_image(img),
                    size.0,
                    size.1,
                    image::imageops::CatmullRom,
                )
            } else {
                P::from_dynamic_image(img)
            }
        }
    })
}

pub(crate) fn transform_to_guide_map<P: SynthPixel>(
    image: SynthPixelBuffer<P>,
    size: Option<(u32, u32)>,
    blur_sigma: f32,
) -> SynthPixelBuffer<P> {
    use image::GenericImageView;
    let dyn_img = P::into_dynamic_image(image);

    if let Some(s) = size {
        if dyn_img.width() != s.0 || dyn_img.height() != s.1 {
            dyn_img.resize(s.0, s.1, image::imageops::Triangle);
        }
    }

    let greyscale = dyn_img.blur(blur_sigma).grayscale();
    P::from_dynamic_image(greyscale)
}

pub(crate) fn get_histogram<P: SynthPixel>(img: &SynthPixelBuffer<P>) -> Vec<u32> {
    let mut hist = vec![0; 256]; //0-255 incl

    let pixels = &img;

    //populate the hist
    for (i, pixel_value) in pixels.iter().enumerate() {
        //since RGBA image, we only care for 1st channel
        if i % 4 == 0 {
            hist[*pixel_value as usize] += 1; //increment histogram by 1
        }
    }

    hist
}

//source will be modified to fit the target
pub(crate) fn match_histograms<P: SynthPixel>(
    source: &mut SynthPixelBuffer<P>,
    target: &SynthPixelBuffer<P>,
) {
    let target_hist = get_histogram::<P>(target);
    let source_hist = get_histogram::<P>(source);

    //get commutative distrib
    let target_cdf = get_cdf(&target_hist);
    let source_cdf = get_cdf(&source_hist);

    //clone the source image, modify and return
    let (dx, dy) = source.dimensions();

    for x in 0..dx {
        for y in 0..dy {
            let pixel_value = source.get_pixel(x, y).colors()[0]; //we only care about the first channel
            let pixel_source_cdf = source_cdf[pixel_value as usize];

            //now need to find by value similar cdf in the target
            let new_pixel_val = target_cdf
                .iter()
                .position(|cdf| *cdf > pixel_source_cdf)
                .unwrap_or((pixel_value + 1) as usize) as u8
                - 1;

            source.put_pixel(x, y, P::greyscale(new_pixel_val));
        }
    }
}

pub(crate) fn get_cdf(a: &[u32]) -> Vec<f32> {
    let mut cumm = vec![0.0; 256];

    for i in 0..a.len() {
        if i != 0 {
            cumm[i] = cumm[i - 1] + (a[i] as f32);
        } else {
            cumm[i] = a[i] as f32;
        }
    }

    //normalize
    let max = cumm[255];
    for i in cumm.iter_mut() {
        *i /= max;
    }

    cumm
}
