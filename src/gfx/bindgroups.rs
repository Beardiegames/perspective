use super::*;
use wgpu::{util::DeviceExt, Surface};


impl WgpuCore {

    /// A bind group defines how buffers are accessed by shaders.
    /// binding` here refers to the `binding` of a buffer in the shader (`layout(set = 0, binding = 0) buffer`).
    /// 
    pub fn setup_bind_group<'a, P, T>(&mut self, label: &str, pipe: &P, buffer: &BufferHandle<'a, T>) -> BindgroupHandle 
        where P: PipelineHandle,
    {

        let set_idx = self.bindgroup_count;
        let bond_idx = 0;

        let layout = pipe.get_bind_group_layout(set_idx);
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: bond_idx,
                resource: buffer.storage.as_entire_binding(),
            }],
        });

        self.bindgroup_count += 1;

        BindgroupHandle {
            bind_group,
            layout,
            set_idx,
            bond_idx
        }
    }
}