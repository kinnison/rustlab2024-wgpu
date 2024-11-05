use std::sync::Arc;

use anyhow::Result;
use winit::{dpi::PhysicalSize, window::Window};

pub struct Application {}

impl Application {
    pub async fn new(window: Arc<Window>, size: PhysicalSize<u32>) -> Result<Self> {
        Ok(Self {})
    }

    pub fn resize(&mut self, width: u32, height: u32) {}

    pub fn handle_event(
        &mut self,
        window: &winit::window::Window,
        winit_event: &winit::event::WindowEvent,
    ) -> bool {
        false
    }

    pub fn render(&mut self, window: &winit::window::Window) -> Result<(), wgpu::SurfaceError> {
        Ok(())
    }
}
