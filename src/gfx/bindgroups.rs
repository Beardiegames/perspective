use super::{*, buffers::BufferHandle};
use wgpu::{util::DeviceExt, Surface};


pub struct BindgroupHandle {
    pub bind_group: wgpu::BindGroup,
    pub layout: wgpu::BindGroupLayout,
    set_idx: u32,
    bond_idx: u32,
}

impl BindgroupHandle {
    pub fn set_idx(&self) -> &u32 { &self.set_idx }
    pub fn bond_idx(&self) -> &u32 { &self.bond_idx }
}

impl BindgroupHandle {

    /// A bind group defines how buffers are accessed by shaders.
    /// binding` here refers to the `binding` of a buffer in the shader (`layout(set = 0, binding = 0) buffer`).
    /// 
    pub fn new<P, T>(core: &mut WgpuCore, label: &str, pipe: &P, buffer: &BufferHandle<T>) -> BindgroupHandle 
        where P: PipelineHandle,
    {

        let set_idx = core.bindgroup_count;
        let bond_idx = 0;

        let layout = pipe.get_bind_group_layout(set_idx);
        let bind_group = core.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: bond_idx,
                resource: buffer.storage.as_entire_binding(),
            }],
        });

        core.bindgroup_count += 1;

        BindgroupHandle {
            bind_group,
            layout,
            set_idx,
            bond_idx
        }
    }
}