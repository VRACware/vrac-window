use vrac_window::{Context, ControlFlow, WindowDescriptor, Windows};
use vulkano_util::window::WindowMode;
use winit::event::{Event, WindowEvent};

fn main() {
    println!("Creating window context");
    let mut c = Context::init_default();
    c.control_flow(ControlFlow::Poll);

    println!("Adding 4 different windows");
    let mut w = Windows::init();
    // let _w_simple = w.add_window(&c, WindowDescriptor::default());
    let _w_primary = w.add_window( &c,
        WindowDescriptor {
            title: "Primary window - borderless fullscreen".to_owned(),
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        }
    );
    let _w_1 = w.add_window( &c,
        WindowDescriptor {
            title: "Window 1 _ unresizable and with invisible cursor".to_owned(),
            resizable: false,
            cursor_visible: false,
            ..Default::default()
        }
    );
    let _w_2 = w.add_window( &c,
        WindowDescriptor {
            title: "Window 2 - undecorated".to_owned(),
            decorations: false,
            ..Default::default()
        }
    );
    let _w_3 = w.add_window( &c,
        WindowDescriptor {
            title: "Window 3 - Transparent".to_owned(),
            transparent: true,
            ..Default::default()
        }
    );


    println!("Running an event loop");
    let mut events_count: u32 = 0;
    c.event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } => {
                #[cfg(any(target_os = "windows", target_os = "macos"))]
                println!("The close button of a window (id: {}, title: \"{}\") was pressed. Closing this window.", u64::from(window_id), w.get_window(window_id).title());

                #[cfg(not(any(target_os = "windows", target_os = "macos")))]
                println!("The close button of a window (id: {}) was pressed. Closing this window.", u64::from(window_id));


                // Several windows
                if window_id==w.w.primary_window_id().unwrap() { elwt.exit(); }
                w.remove_window(window_id);

                // // Single window
                // elwt.exit();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                .. // window_id,
            } => {
                // w.get_window(window_id).request_redraw();
                events_count+=1;
                if events_count>3 {
                    println!("Exiting the event loop");
                    elwt.exit();
                }
            },
            _ => ()
        }
    }).unwrap();
}