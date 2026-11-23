use anyhow::{Result, anyhow, bail};
use candle_metal_kernels::metal::{Commands, Device, ResidencySet};
use candle_metal_kernels::{RESOURCE_OPTIONS, kernel::load_pipeline_from_metallib};
use objc2_metal::MTLSize;
use pliron::{op::Op, printable::Printable};
use wax_frontend::{KernelCompiler, ast};

// wax only hands back metallib bytes, so how to dispatch it is on the caller
pub struct Launch {
    pub blocks: usize,
    pub threads: usize,
}

impl Launch {
    pub fn new(blocks: usize, threads: usize) -> Self {
        Self { blocks, threads }
    }
}

pub fn run(
    module: fn() -> ast::Module,
    entry: &str,
    inputs: &[(&str, &[f32])],
    strides: [i32; 2],
    launch: Launch,
) -> Result<Vec<f32>> {
    // output first then inputs, every tensor gets the same strides
    let n = inputs[0].1.len();
    let mut args = vec![("out", &strides[..])];
    args.extend(inputs.iter().map(|(name, _)| (*name, &strides[..])));
    // rust dsl -> cutile ir -> wax ir -> metallib, all in process
    let mut module = KernelCompiler::new(module, "kernels", entry)
        .strides(&args)
        .compile()
        .map_err(|e| anyhow!("{e}"))?
        .to_wax_module(entry)
        .map_err(|e| anyhow!("{e}"))?;
    // WAX_DUMP=1 writes the wax ir, the metallib and its air bitcode to artifacts/
    let dump = std::env::var("WAX_DUMP").is_ok();
    let ir = dump.then(|| {
        module
            .module_op()
            .get_operation()
            .disp(&module.ctx)
            .to_string()
    });
    let metallib = wax_metal::lower_to_metallib(&mut module)?;
    if let Some(ir) = ir {
        let bc = wax_metal::metallib::bitcode_of(&metallib).map_err(|e| anyhow!(e))?;
        std::fs::create_dir_all("artifacts")?;
        std::fs::write(format!("artifacts/{entry}.wax"), ir)?;
        std::fs::write(format!("artifacts/{entry}.metallib"), &metallib)?;
        std::fs::write(format!("artifacts/{entry}.bc"), bc)?;
    }

    // plain metal from here, load the metallib and bind buffers in arg order
    let device = Device::system_default().ok_or(anyhow!("no metal device"))?;
    let pipeline = load_pipeline_from_metallib(&device, &metallib, entry)?;
    // metal skips an oversized threadgroup without any error, so check it here
    let max = pipeline.max_total_threads_per_threadgroup();
    if launch.threads > max {
        bail!(
            "{entry}: {} threads per block, this gpu allows {max}",
            launch.threads
        );
    }
    let out = device.new_buffer(n * 4, RESOURCE_OPTIONS)?;
    let ins = inputs
        .iter()
        .map(|(_, v)| device.new_buffer_with_data(v.as_ptr().cast(), n * 4, RESOURCE_OPTIONS))
        .collect::<Result<Vec<_>, _>>()?;

    let residency = std::sync::Arc::new(ResidencySet::new(&device));
    let commands = Commands::new(device.new_command_queue()?, &residency)?;
    let enc = commands.command_encoder()?;
    let e = enc.as_ref();
    e.set_compute_pipeline_state(&pipeline);
    e.set_output_buffer(0, Some(&out), 0);
    for (i, b) in ins.iter().enumerate() {
        e.set_input_buffer(i + 1, Some(b), 0);
    }
    let size = |w| MTLSize {
        width: w,
        height: 1,
        depth: 1,
    };
    e.dispatch_thread_groups(size(launch.blocks), size(launch.threads));
    drop(enc);
    commands.wait_until_completed()?;
    Ok(unsafe { std::slice::from_raw_parts(out.contents() as *const f32, n) }.to_vec())
}
