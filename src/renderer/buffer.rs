pub struct Buffer {
    device: wgpu::Device,
    buffer: wgpu::Buffer,

    builder: BufferBuilder,
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
        self.buffer = self.device.create_buffer(&self.builder.descriptor());
    }
}

impl BufferBuilder {
    pub fn build(self, device: &wgpu::Device) -> Buffer {
        let descriptor = self.descriptor();

        Buffer {
            device: device.clone(),
            buffer: device.create_buffer(&descriptor),
            builder: self,
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

    pub fn label(mut self, label: &impl std::fmt::Display) -> Self {
        self.label = Some(format!("{label}").to_string());
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
