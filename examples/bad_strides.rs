// strides are passed next to the kernel, not checked against its shape.
// a [2, 128] tensor with strides [1, 1] compiles, runs, and is quietly wrong
#[wax_macro::module]
pub mod kernels {
    use cutile::core::*;

    #[wax_macro::entry()]
    pub fn add(
        out: &mut Tensor<f32, { [2, 128] }>,
        x: &Tensor<f32, { [2, 128] }>,
        y: &Tensor<f32, { [2, 128] }>,
    ) {
        let ts = const_shape![2, 128];
        out.store(x.partition(ts).load([0i32, 0i32]) + y.partition(ts).load([0i32, 0i32]));
    }
}

use wax_example::{Launch, run};

fn main() -> anyhow::Result<()> {
    let x: Vec<f32> = (0..256).map(|i| i as f32).collect();
    let y = vec![1f32; 256];
    let args = [("x", &x[..]), ("y", &y[..])];
    for strides in [[128, 1], [1, 1]] {
        let out = run(
            kernels::__module_ast_self,
            "add",
            &args,
            strides,
            Launch::new(1, 256),
        )?;
        let wrong = (0..256).filter(|&i| out[i] != x[i] + y[i]).count();
        println!("strides {strides:?}: {wrong}/256 wrong");
    }
    Ok(())
}
