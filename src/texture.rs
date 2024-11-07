pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub format: wgpu::TextureFormat,
    pub dimensions: (u32, u32),
}

impl Texture {
    pub fn new(
        device: &wgpu::Device,
        dimensions: (u32, u32),
        label: Option<&str>,
        format: wgpu::TextureFormat,
        storage: bool,
    ) -> Self {
        let (width, height) = dimensions;
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            view_formats: &[format],
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | if storage {
                    wgpu::TextureUsages::STORAGE_BINDING
                } else {
                    wgpu::TextureUsages::RENDER_ATTACHMENT
                },
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture,
            view,
            format,
            dimensions,
        }
    }
}
