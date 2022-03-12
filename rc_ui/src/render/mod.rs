use crate::component::UIComponent;
use crate::render::pipeline::UIRenderPipeline;
use std::lazy::SyncOnceCell;
use std::sync::Arc;
use std::sync::Mutex;
use wgpu::{Device, TextureFormat};

mod combine;
mod component;
mod default;
pub mod pipeline;
pub mod projection;

pub(crate) static DEVICE: SyncOnceCell<&'static Device> = SyncOnceCell::new();
pub(crate) static SWAPCHAIN_FORMAT: SyncOnceCell<&'static TextureFormat> = SyncOnceCell::new();

pub(crate) fn get_device() -> &'static Device {
    DEVICE.get().unwrap()
}

pub(crate) fn get_swapchain_format() -> &'static TextureFormat {
    SWAPCHAIN_FORMAT.get().unwrap()
}
