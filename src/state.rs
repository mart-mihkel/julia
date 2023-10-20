use std::default::Default;
use wgpu::{AddressMode, Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages, Color, CommandEncoderDescriptor, ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor, Device, DeviceDescriptor, Extent3d, Features, FilterMode, FragmentState, include_wgsl, IndexFormat, Instance, InstanceDescriptor, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor, PowerPreference, PrimitiveState, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, SamplerBindingType, SamplerDescriptor, ShaderStages, StorageTextureAccess, Surface, SurfaceConfiguration, SurfaceError, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureViewDescriptor, TextureViewDimension, VertexState};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::Window;
use crate::vertex::Vertex;
use crate::{Args, vertex};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct JuliaUniforms {
    constant: [f32; 2],
    offset: [f32; 2],
    zoom: f32,
}

pub struct State {
    args: Args,
    surface: Surface,
    surface_config: SurfaceConfiguration,
    device: Device,
    queue: Queue,

    window: Window,
    size: PhysicalSize<u32>,
    mouse_position: PhysicalPosition<f64>,

    render_bind_group: BindGroup,
    compute_bind_group: BindGroup,
    render_pipeline: RenderPipeline,
    compute_pipeline: ComputePipeline,

    uniforms: JuliaUniforms,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_vertices: u32,
    num_indices: u32,
}

impl State {
    pub async fn new(args: Args, window: Window) -> Self {
        // the instance is a handle to our GPU
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::VULKAN,
            dx12_shader_compiler: Default::default(),
        });

        // the surface needs to live as long as the window that created it,
        // state owns the window so this should be safe
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&DeviceDescriptor {
            features: Features::empty(),
            label: Some("Device"),
            limits: Default::default(),
        }, None).await.unwrap();

        // surface
        let mouse_position = PhysicalPosition::new(0f64, 0f64);
        let size = window.inner_size();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        // shaders
        let compute_shader = device.create_shader_module(include_wgsl!("compute.wgsl"));
        let render_shader = device.create_shader_module(include_wgsl!("render.wgsl"));

        // render bind layout
        let render_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Render bind group layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        });

        // render pipeline
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&render_bind_group_layout],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &render_shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(FragmentState {
                module: &render_shader,
                entry_point: "fs_main",
                targets: &[Some(surface_format.into())],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });

        // image
        let img = device.create_texture(&TextureDescriptor {
            label: Some("Image"),
            size: Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let img_view = img.create_view(&Default::default());

        // compute bind group layout
        let compute_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Compute bind group layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::WriteOnly,
                        format: TextureFormat::R8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });

        // compute pipeline
        let compute_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Compute pipeline layout"),
            bind_group_layouts: &[&compute_bind_group_layout],
            push_constant_ranges: &[],
        });
        let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("Compute pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "cs_main",
        });

        // compute buffers
        let compute_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Compute buffer"),
            size: 1, // todo
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE | BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });
        let compute_buffer_binding = compute_buffer.as_entire_binding();

        // compute bind group
        let compute_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Compute bind group"),
            layout: &compute_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: compute_buffer_binding,
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&img_view),
                },
            ],
        });

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("Sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let render_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Render bind group"),
            layout: &render_bind_group_layout,
            entries: &[
                BindGroupEntry { binding: 0, resource: BindingResource::TextureView(&img_view) },
                BindGroupEntry { binding: 0, resource: BindingResource::Sampler(&sampler) },
            ],
        });

        // uniform buffer
        let uniforms = JuliaUniforms { // todo meetodisse
            constant: args.constant,
            offset: [0f32; 2],
            zoom: 0f32,
        };
        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // vertex and index buffers
        let vertices = vertex::VERTICES;
        let num_vertices = vertices.len() as u32;
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        });

        let indices = vertex::INDICES;
        let num_indices = indices.len() as u32;
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });

        Self {
            args,
            device,
            queue,
            surface,
            surface_config,

            window,
            size,
            mouse_position,

            render_bind_group,
            compute_bind_group,
            render_pipeline,
            compute_pipeline,

            uniforms,
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
        }
    }

    pub fn update(&mut self) {
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Compute encoder"),
        });

        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("Compute pass"),
        });
        compute_pass.set_pipeline(&self.compute_pipeline);
        compute_pass.set_bind_group(0, &self.compute_bind_group, &[]);
        compute_pass.dispatch_workgroups(0, 0, 1); // todo
        drop(compute_pass);

        self.queue.submit(Some(encoder.finish()));
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.render_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        drop(render_pass);

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn zoom_in(&mut self) {
        self.uniforms.zoom *= 0.95;
    }

    pub fn zoom_out(&mut self) {
        self.uniforms.zoom *= 1.05;
    }

    pub fn set_mouse_position(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_position = position;
    }

    pub fn offset_to_mouse(&mut self) {
        let half_width = self.window.inner_size().width as f64 / 2.0;
        let half_height = self.window.inner_size().height as f64 / 2.0;
        self.uniforms.offset = [
            self.uniforms.offset[0] + (self.mouse_position.x / half_width - 1.0) * self.uniforms.zoom,
            self.uniforms.offset[1] + (self.mouse_position.y / -half_height + 1.0) * self.uniforms.zoom,
        ];
    }
}