// super simple add kernel on the GPU
#[wax_macro::module]
pub mod kernels {
    use cutile::core::*;

    #[wax_macro::entry()]
    pub fn add(
        out: &mut Tensor<f32, { [1, 256] }>,
        x: &Tensor<f32, { [1, 256] }>,
        y: &Tensor<f32, { [1, 256] }>,
    ) {
        let ts = const_shape![1, 256];
        out.store(x.partition(ts).load([0i32, 0i32]) + y.partition(ts).load([0i32, 0i32]));
    }
}

use wax_example::{Launch, run};

fn main() -> anyhow::Result<()> {
    let x: Vec<f32> = (0..256).map(|i| i as f32).collect();
    let y: Vec<f32> = x.iter().map(|v| 1000.0 - v).collect();
    let args = [("x", &x[..]), ("y", &y[..])];
    // the macro gives us __module_ast_self so wax can read the kernel source
    let out = run(
        kernels::__module_ast_self,
        "add",
        &args,
        [256, 1],
        Launch::new(1, 256),
    )?;
    let ok = out.iter().all(|&v| v == 1000.0);
    println!("{:?} all 1000: {ok}", &out[..4]);
    Ok(())
}
