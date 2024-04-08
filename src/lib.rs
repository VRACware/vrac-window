use winit::{event_loop::EventLoop, window::{Window, WindowId}};
use vulkano_util::{context::{VulkanoConfig, VulkanoContext}, window::VulkanoWindows};

pub use winit::event_loop::ControlFlow;
pub use vulkano_util::window::WindowDescriptor;

/// Fundamental contextual elements.
/// i.e. event loop and actual Vulkan context.
pub struct Context {
    pub event_loop: EventLoop<()>,
    pub context: VulkanoContext,
}

impl Context {
    /// Initializes an event loop and a Vulkan context with default configuration
    pub fn init_default() -> Self {
        let event_loop = EventLoop::new().unwrap();
        let context = VulkanoContext::new(VulkanoConfig::default());
        Self {
            event_loop,
            context,
        }
    }

    /// Initializes an event loop and a Vulkan context with the given configuration.
    pub fn init(config: VulkanoConfig) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let context = VulkanoContext::new(config);
        Self {
            event_loop,
            context,
        }
    }

    /// Modifies the event loop control flow (default is `Wait`).
    pub fn control_flow(&mut self, cf: ControlFlow) {
        self.event_loop.set_control_flow(cf);
    }
}

/// The windows.
pub struct Windows {
    pub w: VulkanoWindows,
}

impl Windows {
    /// Initializes the windows
    pub fn init() -> Self {
        let w = VulkanoWindows::default();
        Self {
            w,
        }
    }

    /// Creates a new window, returns a pointer to it.
    /// The first window created is the primary window. The primary window is usually responsible for terminating the full render target (all windows) when it is closed.
    pub fn add_window(&mut self, c: &Context, wd: WindowDescriptor) -> WindowId {
        self.w.create_window(
            &c.event_loop,
            &c.context,
            &wd,
            |_| {},
        )
    }

    /// Gets a window from its id.
    pub fn get_window(&self, id: WindowId) -> &Window {
        self.w.get_window(id).unwrap()
    }

    /// Closes an existing window.
    pub fn remove_window(&mut self, id: WindowId) {
        self.w.remove_renderer(id);
    }
}