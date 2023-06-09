use camera::Camera;
use profile::scope;
use renderer::Renderer;

use scene::{create_test_scene, create_test_scene_2, create_test_scene_3};

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

use crate::context::GraphicsContext;

mod camera;
mod context;
mod image;
mod pipeline;
mod profile;
mod ray;
mod renderer;
mod scene;
mod utils;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2], // NEW!
}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let size = window.inner_size();

    let context = GraphicsContext::new(&window).await;
    let pipeline = pipeline::Pipeline::new(&context, size.width, size.height);

    let mut camera = Camera::new(45.0, 0.1, 100.0);
    let mut renderer = Renderer::new();

    let scene_selector = 3;
    let scene = match scene_selector {
        2 => create_test_scene_2(),
        3 => create_test_scene_3(),
        _ => create_test_scene(),
    };

    let mut should_close = false;
    while !should_close {
        let _profile = scope("Run loop");

        // update
        camera.on_resize(size.width as usize, size.height as usize);
        camera.on_update(0.0);
        renderer.on_resize(size.width as usize, size.height as usize);
        renderer.render(&scene, &camera);

        let final_image = renderer.final_image();
        let final_image_buffer = final_image.buffer();

        context.queue().write_texture(
            wgpu::ImageCopyTexture {
                texture: &pipeline.texture(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(final_image_buffer),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.width),
                rows_per_image: Some(size.height),
            },
            pipeline.texture_size(),
        );

        // render
        let output = context.surface().get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            context
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            // 1.
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.1,
                                b: 0.1,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }),
                ],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(pipeline.pipeline());
            render_pass.set_bind_group(0, pipeline.bind_group(), &[]); // NEW!
            render_pass.set_vertex_buffer(0, pipeline.vertex_buffer().slice(..)); // 1.
            render_pass
                .set_index_buffer(pipeline.index_buffer().slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..pipeline.num_indices(), 0, 0..1);
            // 2.
        }

        // submit will accept anything that implements IntoIter
        context.queue().submit(std::iter::once(encoder.finish()));
        output.present();

        // handle events
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        should_close = true;
                        *control_flow = ControlFlow::Exit
                    }
                    _ => {}
                },
                Event::MainEventsCleared => *control_flow = ControlFlow::Exit,
                _ => {}
            }
        });
    }
}
