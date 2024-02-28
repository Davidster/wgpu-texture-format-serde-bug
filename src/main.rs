use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WgpuTextureFormat(wgpu::TextureFormat);

fn main() {
    let texture_format = wgpu::TextureFormat::Rgba8Unorm;
}
