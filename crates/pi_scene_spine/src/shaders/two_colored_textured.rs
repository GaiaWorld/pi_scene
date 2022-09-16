
pub struct APositionVertexBuffer;
impl APositionVertexBuffer {
	pub fn id() -> u32 {
		0
	}
}

pub struct AColorVertexBuffer;
impl AColorVertexBuffer {
	pub fn id() -> u32 {
		1
	}
}

pub struct AColor2VertexBuffer;
impl AColor2VertexBuffer {
	pub fn id() -> u32 {
		2
	}
}

pub struct ATexCoordVertexBuffer;
impl ATexCoordVertexBuffer {
	pub fn id() -> u32 {
		3
	}
}

pub struct ParamGroup;
impl pi_render::rhi::dyn_uniform_buffer::Group for ParamGroup {
	fn id() -> u32 {
		0
	}

	fn create_layout(
		device: &pi_render::rhi::device::RenderDevice,
		has_dynamic_offset: bool,
	) -> pi_render::rhi::bind_group_layout::BindGroupLayout {
		device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("param bindgroup layout"),
			entries: &[wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset,
					min_binding_size: wgpu::BufferSize::new(80),
				},
				count: None, // TODO
			}],
		})
	}
}

impl pi_render::rhi::dyn_uniform_buffer::BufferGroup for ParamGroup {
	fn create_bind_group(
		device: &pi_render::rhi::device::RenderDevice,
		layout: &pi_render::rhi::bind_group_layout::BindGroupLayout,
		buffer: &pi_render::rhi::buffer::Buffer,
	) -> pi_render::rhi::bind_group::BindGroup {
		device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
					buffer,
					offset: 0,
					size: Some(std::num::NonZeroU64::new(80).unwrap()),
				}),
			}],
			label: Some("param bindgroup"),
		})
	}
}

pub struct SamplerUTextureUTextureGroup;
impl pi_render::rhi::dyn_uniform_buffer::Group for SamplerUTextureUTextureGroup {
	fn id() -> u32 {
		1
	}

	fn create_layout(
		device: &pi_render::rhi::device::RenderDevice,
		has_dynamic_offset: bool,
	) -> pi_render::rhi::bind_group_layout::BindGroupLayout {
		device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("sampler_u_texture_u_texture bindgroup layout"),
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
					count: None,
				},
				wgpu::BindGroupLayoutEntry {
					binding: 1,
					visibility: wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Texture {
						multisampled: false,
						sample_type: wgpu::TextureSampleType::Float { filterable: true },
						view_dimension: wgpu::TextureViewDimension::D2,
					},
					count: None, // TODO
				},
			],
		})
	}
}

pub struct ParamBind;
impl pi_render::rhi::dyn_uniform_buffer::Bind for ParamBind {
	#[inline]
	fn min_size() -> usize {
		80
	}

	fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
		pi_render::rhi::dyn_uniform_buffer::BindIndex::new(0)
	}
}

pub struct UProjTranUniform<'a>(pub &'a [f32]);
impl<'a> pi_render::rhi::dyn_uniform_buffer::Uniform for UProjTranUniform<'a> {
	fn write_into(&self, index: u32, buffer: &mut [u8]) {
		unsafe {
			std::ptr::copy_nonoverlapping(
				self.0.as_ptr() as usize as *const u8,
				buffer.as_mut_ptr().add(index as usize + 0),
				64,
			)
		};
	}
}

pub struct UMaskflagUniform<'a>(pub &'a [f32]);
impl<'a> pi_render::rhi::dyn_uniform_buffer::Uniform for UMaskflagUniform<'a> {
	fn write_into(&self, index: u32, buffer: &mut [u8]) {
		unsafe {
			std::ptr::copy_nonoverlapping(
				self.0.as_ptr() as usize as *const u8,
				buffer.as_mut_ptr().add(index as usize + 64),
				16,
			)
		};
	}
}

pub struct TwoColoredTexturedShader;

impl TwoColoredTexturedShader {
	pub fn create_bind_group_sampler_u_texture_u_texture(
		device: &pi_render::rhi::device::RenderDevice,
		layout: &pi_render::rhi::bind_group_layout::BindGroupLayout,
		sampler_u_texture: &wgpu::Sampler,
		u_texture: &wgpu::TextureView,
	) -> pi_render::rhi::bind_group::BindGroup {
		device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::Sampler(sampler_u_texture),
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: wgpu::BindingResource::TextureView(u_texture),
				},
			],
			label: Some("sampler_u_texture_u_texture bindgroup"),
		})
	}
}
