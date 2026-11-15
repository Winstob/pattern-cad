use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct Buffer {
    device: wgpu::Device,
    builder: BufferBuilder,

    pub data: Rc<RefCell<BufferInternal>>,
}

pub struct BufferInternal {
    pub buffer: wgpu::Buffer,
}

pub struct BufferBuilder {
    label: Option<String>,
    size: u64,
    usage: wgpu::BufferUsages,
    mapped_at_creation: bool,
}

impl Buffer {
    pub fn builder() -> BufferBuilder {
        BufferBuilder {
            label: None,
            size: 1024,
            usage: wgpu::BufferUsages::empty(),
            mapped_at_creation: false,
        }
    }

    pub fn resize(&mut self, new_size: u64) {
        self.builder.size = new_size;
        *self.data.borrow_mut() = BufferInternal {
            buffer: self.device.create_buffer(&self.builder.descriptor()),
        };
    }

    pub fn size(&self) -> u64 {
        self.builder.size
    }

    pub fn data(&self) -> Ref<'_, wgpu::Buffer> {
        Ref::map(self.data.borrow(), |internal| &internal.buffer)
    }
}

impl BufferBuilder {
    pub fn build(self, device: &wgpu::Device) -> Buffer {
        let descriptor = self.descriptor();

        let data = Rc::new(RefCell::new(BufferInternal {
            buffer: device.create_buffer(&descriptor),
        }));

        Buffer {
            device: device.clone(),
            builder: self,
            data,
        }
    }

    fn descriptor(&self) -> wgpu::BufferDescriptor<'_> {
        wgpu::BufferDescriptor {
            label: self.label.as_deref(),
            size: self.size,
            usage: self.usage,
            mapped_at_creation: self.mapped_at_creation,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn size(mut self, sz: u64) -> Self {
        self.size = sz;
        self
    }

    pub fn usage(mut self, usage: wgpu::BufferUsages) -> Self {
        self.usage |= usage;
        self
    }
}
