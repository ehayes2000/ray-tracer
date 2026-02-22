use anyhow::Result;
#[pollster::main]
async fn main() -> Result<()> {
    ray_tracer::gpu::app::App::run();
    Ok(())
}
