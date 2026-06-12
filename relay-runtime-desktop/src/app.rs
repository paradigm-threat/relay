//! winit application shell: hook → layout → tiny-skia → softbuffer present.

use crate::hook::{run_hook_to_vnode_stable, reset_hook_session};
use crate::layout::{build_layout, hit_test, LayoutTree};
use crate::paint::Painter;
use relay_runtime::host::HostHandle;
use relay_runtime::run_state::{consume_render_requested, invoke_handler};
use relay_runtime::vdom::VNode;
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::rc::Rc;
use taffy::prelude::*;
use tiny_skia::Pixmap;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

pub struct DesktopApp {
    title: String,
    hook_source: String,
    theme_yaml: String,
    modules: HashMap<String, String>,
    host: HostHandle,
    window: Option<Rc<Window>>,
    pixmap: Option<Pixmap>,
    softbuffer: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
    layout: Option<LayoutTree>,
    vnode: Option<VNode>,
    painter: Painter,
    width: u32,
    height: u32,
    cursor_x: f32,
    cursor_y: f32,
}

impl DesktopApp {
    pub fn new(
        title: impl Into<String>,
        hook_source: impl Into<String>,
        theme_yaml: impl Into<String>,
        modules: HashMap<String, String>,
        host: HostHandle,
    ) -> Self {
        Self {
            title: title.into(),
            hook_source: hook_source.into(),
            theme_yaml: theme_yaml.into(),
            modules,
            host,
            window: None,
            pixmap: None,
            softbuffer: None,
            layout: None,
            vnode: None,
            painter: Painter::default(),
            width: 800,
            height: 600,
            cursor_x: 0.0,
            cursor_y: 0.0,
        }
    }

    pub fn run(self) -> Result<(), String> {
        let event_loop = EventLoop::new().map_err(|e| format!("event loop: {e}"))?;
        let mut app = self;
        event_loop
            .run_app(&mut app)
            .map_err(|e| format!("run_app: {e}"))
    }

    fn rerender(&mut self, reset: bool) {
        match run_hook_to_vnode_stable(
            &self.hook_source,
            self.host.clone(),
            &self.theme_yaml,
            self.modules.clone(),
            reset,
        ) {
            Ok(r) => {
                self.vnode = Some(r.vnode.clone());
                let viewport = Size {
                    width: self.width as f32,
                    height: self.height as f32,
                };
                self.layout = Some(build_layout(r.vnode, viewport));
            }
            Err(e) => {
                tracing::error!("hook run failed: {e}");
            }
        }
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.pixmap = Pixmap::new(self.width, self.height);
        if let Some(sb) = self.softbuffer.as_mut() {
            let _ = sb.resize(
                NonZeroU32::new(self.width).unwrap(),
                NonZeroU32::new(self.height).unwrap(),
            );
        }
        if let Some(v) = self.vnode.clone() {
            let viewport = Size {
                width: self.width as f32,
                height: self.height as f32,
            };
            self.layout = Some(build_layout(v, viewport));
        }
    }

    fn present(&mut self) {
        let pixmap = match self.pixmap.as_mut() {
            Some(p) => p,
            None => return,
        };
        if let Some(layout) = &self.layout {
            self.painter.paint_layout(pixmap, layout);
        } else {
            pixmap.fill(tiny_skia::Color::from_rgba8(30, 30, 30, 255));
        }

        let Some(sb) = self.softbuffer.as_mut() else {
            return;
        };
        let Ok(mut buffer) = sb.buffer_mut() else {
            return;
        };
        let data = pixmap.data();
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let o = i * 4;
            if o + 3 >= data.len() {
                break;
            }
            let r = data[o] as u32;
            let g = data[o + 1] as u32;
            let b = data[o + 2] as u32;
            let a = data[o + 3] as u32;
            *pixel = (a << 24) | (r << 16) | (g << 8) | b;
        }
        let _ = buffer.present();
    }
}

impl ApplicationHandler for DesktopApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = WindowAttributes::default()
            .with_title(self.title.clone())
            .with_inner_size(LogicalSize::new(1024.0, 768.0));
        let window = Rc::new(
            event_loop
                .create_window(attrs)
                .expect("create_window"),
        );
        let ctx = softbuffer::Context::new(window.clone()).expect("softbuffer context");
        let mut sb = softbuffer::Surface::new(&ctx, window.clone()).expect("softbuffer surface");
        let size = window.inner_size();
        self.width = size.width.max(1);
        self.height = size.height.max(1);
        let _ = sb.resize(
            NonZeroU32::new(self.width).unwrap(),
            NonZeroU32::new(self.height).unwrap(),
        );
        self.pixmap = Pixmap::new(self.width, self.height);
        self.softbuffer = Some(sb);
        self.window = Some(window);
        reset_hook_session();
        self.rerender(true);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => self.present(),
            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_x = position.x as f32;
                self.cursor_y = position.y as f32;
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                if let Some(layout) = &self.layout {
                    if let Some(id) = hit_test(layout, self.cursor_x, self.cursor_y) {
                        let _ = invoke_handler(id);
                        self.rerender(false);
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if consume_render_requested() {
            self.rerender(false);
        }
    }
}

pub fn run_desktop_window(
    title: &str,
    hook_source: &str,
    theme_yaml: &str,
    modules: HashMap<String, String>,
    host: HostHandle,
) -> Result<(), String> {
    DesktopApp::new(title, hook_source, theme_yaml, modules, host).run()
}
