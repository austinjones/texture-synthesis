use texture_synthesis as ts;
use texture_synthesis::pixel::Luma;
use texture_synthesis::Session;

fn main() -> Result<(), ts::Error> {
    // Let's start layering some of the "verbs" of texture synthesis
    // if we just run tiling_mode(true) we will generate a completely new image from scratch (try it!)
    // but what if we want to tile an existing image?
    // we can use inpaint!

    let texsynth: Session<Luma> = ts::SessionBuilder::default()
        // load a mask that specifies borders of the image we can modify to make it tiling
        .add_example(&"imgs/1.jpg")
        .build()?;

    let generated = texsynth.run(None);

    generated.save("out/08.jpg")
}
