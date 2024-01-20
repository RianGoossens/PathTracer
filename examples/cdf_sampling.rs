use nalgebra::Vector3;
use path_tracer::RenderBuffer;
use rand::{thread_rng, Rng};

const SIZE: u32 = 100;

fn main() {
    let mut rng = thread_rng();

    let mut render = RenderBuffer::new(SIZE, SIZE);

    for i in 0..SIZE {
        for j in 0..SIZE {
            render[(i, j)] = Vector3::new(rng.gen(), rng.gen(), rng.gen());
        }
    }

    let image = render.to_image_u8();

    image.save("image.png").expect("Could not save image");
}
