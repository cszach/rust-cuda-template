use cust::prelude::*;
use std::error::Error;

const NUMBERS_LEN: usize = 1 << 20;

static PTX: &str = include_str!("../../../resources/add.ptx");

fn main() -> Result<(), Box<dyn Error>> {
    let x = vec![1.0f32; NUMBERS_LEN];
    let y = vec![2.0f32; NUMBERS_LEN];

    let _ctx = cust::quick_init()?;

    let module = Module::from_ptx(PTX, &[])?;

    let stream = Stream::new(StreamFlags::NON_BLOCKING, None)?;

    let x_gpu = x.as_slice().as_dbuf()?;
    let y_gpu = y.as_slice().as_dbuf()?;

    let mut out = vec![0.0f32; NUMBERS_LEN];
    let out_buf = out.as_slice().as_dbuf().unwrap();

    let func = module.get_function("add")?;

    let (_, block_size) = func.suggested_launch_configuration(0, 0.into())?;

    let grid_size = (NUMBERS_LEN as u32 + block_size - 1) / block_size;

    println!(
        "using {} blocks and {} threads per block",
        grid_size, block_size
    );

    unsafe {
        launch!(
            func<<<grid_size, block_size, 0, stream>>>(
                x_gpu.as_device_ptr(),
                x_gpu.len(),
                y_gpu.as_device_ptr(),
                y_gpu.len(),
                out_buf.as_device_ptr(),
            )
        )?;
    }

    stream.synchronize()?;

    out_buf.copy_to(&mut out)?;

    println!("{} + {} = {}", x[0], y[0], out[0]);

    Ok(())
}