use nalgebra::Vector3;
use path_tracer::RenderBuffer;
use rand::{thread_rng, Rng};

const SIZE: u32 = 100;

/*
Given a function f that samples v

We want a function p(v) that approximates the probability that f produces v

suppose we have a pdf(v) that gives the probability v is chosen by cdf(n)
with n a random normalized vector

pdf = cdf-1(x,y,z) dx dy dz 

*/

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
