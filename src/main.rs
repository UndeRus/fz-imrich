#![no_std]
#![no_main]
extern crate alloc;
use alloc::boxed::Box;
use core::ffi::{c_char, c_void, CStr};
use core::mem::{self, MaybeUninit};

use flipperzero_rt as rt;
use flipperzero_sys as sys;

// Required for allocator
extern crate flipperzero_alloc;

rt::manifest!(name = "Example: Images");
rt::entry!(main);

const RECORD_GUI: *const c_char = sys::c_string!("gui");

static mut TARGET_ICON: Icon = Icon {
    width: 128,
    height: 64,
    frame_count: 55,
    frame_rate: 4,
    frames: unsafe { TARGET_FRAMES.as_ptr() },
};
static mut TARGET_FRAMES: [*const u8; 55] = [
include_bytes!("frames/0000.icon").as_ptr(),
include_bytes!("frames/0001.icon").as_ptr(),
include_bytes!("frames/0002.icon").as_ptr(),
include_bytes!("frames/0003.icon").as_ptr(),
include_bytes!("frames/0004.icon").as_ptr(),
include_bytes!("frames/0005.icon").as_ptr(),
include_bytes!("frames/0006.icon").as_ptr(),
include_bytes!("frames/0007.icon").as_ptr(),
include_bytes!("frames/0008.icon").as_ptr(),
include_bytes!("frames/0009.icon").as_ptr(),
include_bytes!("frames/0010.icon").as_ptr(),
include_bytes!("frames/0011.icon").as_ptr(),
include_bytes!("frames/0012.icon").as_ptr(),
include_bytes!("frames/0013.icon").as_ptr(),
include_bytes!("frames/0014.icon").as_ptr(),
include_bytes!("frames/0015.icon").as_ptr(),
include_bytes!("frames/0016.icon").as_ptr(),
include_bytes!("frames/0017.icon").as_ptr(),
include_bytes!("frames/0018.icon").as_ptr(),
include_bytes!("frames/0019.icon").as_ptr(),
include_bytes!("frames/0020.icon").as_ptr(),
include_bytes!("frames/0021.icon").as_ptr(),
include_bytes!("frames/0022.icon").as_ptr(),
include_bytes!("frames/0023.icon").as_ptr(),
include_bytes!("frames/0024.icon").as_ptr(),
include_bytes!("frames/0025.icon").as_ptr(),
include_bytes!("frames/0026.icon").as_ptr(),
include_bytes!("frames/0027.icon").as_ptr(),
include_bytes!("frames/0028.icon").as_ptr(),
include_bytes!("frames/0029.icon").as_ptr(),
include_bytes!("frames/0030.icon").as_ptr(),
include_bytes!("frames/0031.icon").as_ptr(),
include_bytes!("frames/0032.icon").as_ptr(),
include_bytes!("frames/0033.icon").as_ptr(),
include_bytes!("frames/0034.icon").as_ptr(),
include_bytes!("frames/0035.icon").as_ptr(),
include_bytes!("frames/0036.icon").as_ptr(),
include_bytes!("frames/0037.icon").as_ptr(),
include_bytes!("frames/0038.icon").as_ptr(),
include_bytes!("frames/0039.icon").as_ptr(),
include_bytes!("frames/0040.icon").as_ptr(),
include_bytes!("frames/0041.icon").as_ptr(),
include_bytes!("frames/0042.icon").as_ptr(),
include_bytes!("frames/0043.icon").as_ptr(),
include_bytes!("frames/0044.icon").as_ptr(),
include_bytes!("frames/0045.icon").as_ptr(),
include_bytes!("frames/0046.icon").as_ptr(),
include_bytes!("frames/0047.icon").as_ptr(),
include_bytes!("frames/0048.icon").as_ptr(),
include_bytes!("frames/0049.icon").as_ptr(),
include_bytes!("frames/0050.icon").as_ptr(),
include_bytes!("frames/0051.icon").as_ptr(),
include_bytes!("frames/0052.icon").as_ptr(),
include_bytes!("frames/0053.icon").as_ptr(),
include_bytes!("frames/0054.icon").as_ptr(),
];


/// Internal icon representation.
#[repr(C)]
struct Icon {
    width: u8,
    height: u8,
    frame_count: u8,
    frame_rate: u8,
    frames: *const *const u8,
}


struct App {
    animation: *mut sys::IconAnimation
}



impl Drop for App {
  fn drop(&mut self) {
    unsafe {
      sys::icon_animation_free(self.animation);
    }
  }
}

// Screen is 128x64 px
extern "C" fn app_draw_callback(canvas: *mut sys::Canvas, context: *mut c_void) {
    let app = context as *mut App;
    unsafe {
        sys::canvas_clear(canvas);
	let animation = (*app).animation;
	sys::canvas_draw_icon_animation(canvas, 0, 0, animation);
    }
}

extern "C" fn app_input_callback(input_event: *mut sys::InputEvent, ctx: *mut c_void) {
    unsafe {
        let event_queue = ctx as *mut sys::FuriMessageQueue;
        sys::furi_message_queue_put(event_queue, input_event as *mut c_void, 0);
    }
}

fn main(_args: Option<&CStr>) -> i32 {
    let app = App {
      animation:  unsafe {sys::icon_animation_alloc(&TARGET_ICON as *const Icon as *const c_void as *const sys::Icon)}
    };

    unsafe { sys::icon_animation_start(app.animation);};
    let app = Box::new(app);
    unsafe {
        let event_queue = sys::furi_message_queue_alloc(8, mem::size_of::<sys::InputEvent>() as u32)
            as *mut sys::FuriMessageQueue;

        // Configure view port
        let view_port = sys::view_port_alloc();
        sys::view_port_draw_callback_set(
            view_port,
            Some(app_draw_callback),
	    &*app as *const App as *mut c_void,
            //view_port as *mut c_void,
        );
        sys::view_port_input_callback_set(
            view_port,
            Some(app_input_callback),
            event_queue as *mut c_void,
        );

        // Register view port in GUI
        let gui = sys::furi_record_open(RECORD_GUI) as *mut sys::Gui;
        sys::gui_add_view_port(gui, view_port, sys::GuiLayer_GuiLayerFullscreen);

        let mut event: MaybeUninit<sys::InputEvent> = MaybeUninit::uninit();

        let mut running = true;
        while running {
            if sys::furi_message_queue_get(
                event_queue,
                event.as_mut_ptr() as *mut sys::InputEvent as *mut c_void,
                100,
            ) == sys::FuriStatus_FuriStatusOk
            {
                let event = event.assume_init();
                if event.type_ == sys::InputType_InputTypePress
                    || event.type_ == sys::InputType_InputTypeRepeat
                {
                    match event.key {
                        sys::InputKey_InputKeyLeft => {}, // IMAGE_POSITION.x -= 2,
                        sys::InputKey_InputKeyRight => {}, //IMAGE_POSITION.x += 2,
                        sys::InputKey_InputKeyUp => {}, //IMAGE_POSITION.y -= 2,
                        sys::InputKey_InputKeyDown => {}, //IMAGE_POSITION.y += 2,
                        _ => running = false,
                    }
                }
            }
            sys::view_port_update(view_port);
        }

        sys::view_port_enabled_set(view_port, false);
        sys::gui_remove_view_port(gui, view_port);
        sys::view_port_free(view_port);
        sys::furi_message_queue_free(event_queue);

        sys::furi_record_close(RECORD_GUI);
    }

    0
}
