pub mod corebuilder;
//pub mod encoders;
mod pipeline;
mod bindgroups;
mod buffers;
mod compute;

use pollster::FutureExt;
use wgpu::{InstanceDescriptor, BindGroupLayout};
use raw_window_handle::*;
use pipeline::*;
use bindgroups::*;
use buffers::*;

pub use compute::*;


pub struct WindowSettings<'a, W>
    where W: HasRawWindowHandle + HasRawDisplayHandle,
{
    pub window: &'a W,
    pub width: u32,
    pub height: u32
}

pub struct WgpuCore {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: Option<wgpu::Surface>,

    bindgroup_count: u32,
}

pub trait FromBytes {
	fn from_bytes(b: &[u8]) -> Self;
}
