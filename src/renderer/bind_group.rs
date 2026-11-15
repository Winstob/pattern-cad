use super::buffer::{Buffer, BufferInternal};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct BindGroup {
    device: wgpu::Device,
    label: Option<String>,
    bindings: Vec<Binding>,
    layout: wgpu::BindGroupLayout,
    data: wgpu::BindGroup,
}

struct BindGroupBuilder<'a> {
    label: Option<String>,
    entries: Vec<BindGroupEntry<'a>>,
}

struct BindGroupEntry<'a> {
    pub visibility: wgpu::ShaderStages,
    pub binding: BindingDescriptor<'a>,
}

pub enum BindingDescriptor<'a> {
    Buffer {
        ty: wgpu::BufferBindingType,
        resource: &'a Buffer,
    },
}

pub enum Binding {
    Buffer(Rc<RefCell<BufferInternal>>),
}

enum BindGuard<'a> {
    Buffer(Ref<'a, BufferInternal>),
}

impl BindGroup {
    pub fn builder<'a>() -> BindGroupBuilder<'a> {
        BindGroupBuilder {
            label: None,
            entries: Vec::new(),
        }
    }

    pub fn recreate(&mut self) {
        let bind_guards: Vec<_> = self
            .bindings
            .iter()
            .map(|binding| match binding {
                Binding::Buffer(resource) => BindGuard::Buffer(resource.borrow()),
            })
            .collect();
        let entries: Vec<_> = bind_guards
            .iter()
            .enumerate()
            .map(|(i, bind_guard)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: match bind_guard {
                    BindGuard::Buffer(buf_internal) => buf_internal.buffer.as_entire_binding(),
                },
            })
            .collect();
        let data = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: self.label.as_deref(),
            layout: &self.layout,
            entries: &entries,
        });

        self.data = data;
    }
}

impl<'a> BindGroupBuilder<'a> {
    pub fn build(self, device: wgpu::Device) -> BindGroup {
        let mut bindings = Vec::new();
        for entry in self.entries.iter() {
            bindings.push(match entry.binding {
                BindingDescriptor::Buffer { ty: _, resource } => {
                    Binding::Buffer(resource.data.clone())
                }
            })
        }

        let mut layout_entries = Vec::new();
        for (i, entry) in self.entries.iter().enumerate() {
            layout_entries.push(wgpu::BindGroupLayoutEntry {
                binding: i as u32,
                visibility: entry.visibility,
                ty: match entry.binding {
                    BindingDescriptor::Buffer { ty, resource: _ } => wgpu::BindingType::Buffer {
                        ty,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                },
                count: None,
            });
        }
        let layout_label = match self.label {
            Some(ref label) => Some(label.clone() + " Layout"),
            None => None,
        };
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: layout_label.as_deref(),
            entries: &layout_entries,
        });

        let bind_guards: Vec<_> = self
            .entries
            .iter()
            .map(|entry| match entry.binding {
                BindingDescriptor::Buffer { ty: _, resource } => {
                    BindGuard::Buffer(resource.data.borrow())
                }
            })
            .collect();
        let entries: Vec<_> = bind_guards
            .iter()
            .enumerate()
            .map(|(i, bind_guard)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: match bind_guard {
                    BindGuard::Buffer(buf_internal) => buf_internal.buffer.as_entire_binding(),
                },
            })
            .collect();
        let data = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: self.label.as_deref(),
            layout: &layout,
            entries: &entries,
        });

        BindGroup {
            device: device.clone(),
            label: self.label,
            bindings: bindings,
            layout: layout,
            data: data,
        }
    }

    pub fn entry(mut self, entry: BindGroupEntry<'a>) -> Self {
        self.entries.push(entry);
        self
    }
}
