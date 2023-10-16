use wgpu::{Backends, BindGroup, BindGroupDescriptor, BindGroupLayoutDescriptor, BindingType, BlendState, Buffer, BufferBindingType, BufferUsages, Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, FragmentState, FrontFace, IndexFormat, Instance, InstanceDescriptor, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode, PowerPreference, PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, ShaderStages, Surface, SurfaceConfiguration, SurfaceError, TextureUsages, TextureViewDescriptor, VertexState};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::Window;
use crate::vertex::Vertex;
use crate::{Args, util, ComplexNumber, palette};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct JuliaUniforms {
    c: [f32; 2],
}

pub struct State {
    args: Args,
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    mouse_position: PhysicalPosition<f64>,
    size: PhysicalSize<u32>,
    window: Window,
    julia_bind_group: BindGroup,
    render_pipeline: RenderPipeline,
    vertices: Vec<Vertex>,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_vertices: u32,
    num_indices: u32,
    offset: ComplexNumber,
    zoom: f64,
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
            label: None,
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
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // shader
        let shader_module_descriptor = if args.use_gpu {
            wgpu::include_wgsl!("julia_shader.wgsl")
        } else {
            wgpu::include_wgsl!("default_shader.wgsl")
        };
        let shader = device.create_shader_module(shader_module_descriptor);

        // uniforms
        let julia_uniforms = JuliaUniforms { c: [args.constant[0] as f32, args.constant[1] as f32] };
        let julia_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Julia uniform buffer"),
            contents: bytemuck::cast_slice(&[julia_uniforms]),
            usage: BufferUsages::UNIFORM,
        });
        let julia_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Julia bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });
        let julia_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Julia bind group"),
            layout: &julia_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: julia_buffer.as_entire_binding(),
                }
            ],
        });

        // render pipeline
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &julia_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });
        let vertex_state = VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::desc()],
        };
        let fragment_state_target = [Some(ColorTargetState {
            format: config.format,
            blend: Some(BlendState::REPLACE),
            write_mask: ColorWrites::ALL,
        })];
        let fragment_state = Some(FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &fragment_state_target,
        });
        let primitive_state = wgpu::PrimitiveState {
            topology: if args.use_gpu { PrimitiveTopology::TriangleList } else { PrimitiveTopology::PointList },
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        };
        let multisample_state = MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: vertex_state,
            fragment: fragment_state,
            primitive: primitive_state,
            depth_stencil: None,
            multisample: multisample_state,
            multiview: None,
        });

        // vertex and index buffers
        let vertices = Vertex::init_vertices(args.use_gpu, window.inner_size());
        let num_vertices = vertices.len() as u32;
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let indices = Vertex::init_indices();
        let num_indices = indices.len() as u32;
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices.as_slice()),
            usage: BufferUsages::INDEX,
        });

        let offset = [0.0, 0.0];
        let zoom = 1.0;

        Self {
            args,
            surface,
            device,
            queue,
            config,
            mouse_position,
            size,
            window,
            julia_bind_group,
            render_pipeline,
            vertices,
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
            offset,
            zoom,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn update(&mut self) {
        if self.args.use_gpu { return; }

        let iter_results: Vec<(u32, f64)> = self.vertices.iter()
            .map(|v| v.translate_position(self.offset, self.zoom))
            .map(|z| util::julia_iter(z, self.args.constant, self.args.maximum_iterations))
            .collect();

        iter_results.into_iter().enumerate().for_each(|(i, (it, exp_smoothing))| {
            let c = if it == self.args.maximum_iterations {
                [0f32; 3]
            } else {
                palette::linear_interpolate(
                    palette::pick(self.args.palette, exp_smoothing.floor() as usize),
                    palette::pick(self.args.palette, exp_smoothing.floor() as usize + 1),
                    exp_smoothing as f32 % 1.0,
                )
            };

            self.vertices[i].set_color(c);
        });

        self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices[..]));
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
                    load: LoadOp::Clear(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.julia_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        if self.args.use_gpu {
            render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        } else {
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn use_gpu(&self) -> bool {
        self.args.use_gpu
    }

    pub fn zoom_in(&mut self) {
        self.zoom *= 0.75;
    }

    pub fn zoom_out(&mut self) {
        self.zoom *= 1.25;
    }

    pub fn set_mouse_position(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_position = position;
    }

    pub fn offset_to_mouse(&mut self) {
        let half_width = self.window.inner_size().width as f64 / 2.0;
        let half_height = self.window.inner_size().height as f64 / 2.0;
        self.offset = [
            self.offset[0] + (self.mouse_position.x / half_width - 1.0) * self.zoom,
            self.offset[1] + (self.mouse_position.y / -half_height + 1.0) * self.zoom,
        ];
    }
}