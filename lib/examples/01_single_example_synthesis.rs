use std::fs::File;
use texture_synthesis as ts;
use texture_synthesis::pixel::Rgb;
use texture_synthesis::Session;

fn main() -> Result<(), ts::Error> {
    //create a new session
    let texsynth: Session<Rgb> = ts::SessionBuilder::default()
        //load a single example image
        .add_example(&"imgs/1.jpg")
        .output_size(800, 800)
        .nearest_neighbors(16)
        .random_sample_locations(8)
        .build()?;

    //generate an image
    let generated = texsynth.run(None);

    //save the image to the disk
    generated.save("out/01.jpg")
}
