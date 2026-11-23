// wax compiles a 2048 element tile fine, but that is 2048 threads in one block
// and metal allows 1024. without the check in run you just get zeros back
#[wax_macro::module]
pub mod kernels {
    use cutile::core::*;

    #[wax_macro::entry()]
    pub fn big(
        out: &mut Tensor<f32, { [1, 2048] }>,
        x: &Tensor<f32, { [1, 2048] }>,
        y: &Tensor<f32, { [1, 2048] }>,
    ) {
        let ts = const_shape![1, 2048];
        out.store(x.partition(ts).load([0i32, 0i32]) + y.partition(ts).load([0i32, 0i32]));
    }
}

use wax_example::{Launch, run};

fn main() {
    let x = vec![1f32; 2048];
    let args = [("x", &x[..]), ("y", &x[..])];
    match run(
        kernels::__module_ast_self,
        "big",
        &args,
        [2048, 1],
        Launch::new(1, 2048),
    ) {
        Ok(out) => println!("ran, out[..4] = {:?}", &out[..4]),
        Err(e) => println!("error: {e}"),
    }
}
