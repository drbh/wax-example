# wax-example

a tiny gpu kernel written in rust, compiled to metal by [wax](https://github.com/huggingface/candle/tree/wax) and run on apple silicon. no xcode, no shader source, all in process

## usage

```bash
cargo run   # or nix run
# [1000.0, 1000.0, 1000.0, 1000.0] all 1000: true
```

super simple add kernel on the GPU

```rust
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
```

shout out the compiler wizard [@ivarflakstad](https://github.com/ivarflakstad), creator of wax
