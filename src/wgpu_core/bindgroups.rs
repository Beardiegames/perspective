use super::{*, buffers::BufferHandle};


pub struct BindGroupSettings<'a, P> 
where P: PipelineHandle 
{
    pub label: &'a str, 
    pub group_index: u32,// represented within shader as @binding
    pub binding_index: u32,// represented within shader as @binding
    pub resource_buffer: &'a BufferHandle, // Buffer to hand the shader data to work with
    pub pipeline: &'a P,
}

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

    pub fn new<P>(core: &mut WgpuCore, settings: &BindGroupSettings<P>) -> BindgroupHandle
        where P: PipelineHandle,
    {

        let set_idx = settings.group_index;
        let bond_idx = settings.binding_index;

        let layout = settings.pipeline.get_bind_group_layout(set_idx);
        let bind_group = core.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(settings.label),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: bond_idx,
                resource: settings.resource_buffer.storage.as_entire_binding(),
            }],
        });

        BindgroupHandle {
            bind_group,
            layout,
            set_idx,
            bond_idx
        }
    }
}