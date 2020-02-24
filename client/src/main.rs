use std::time::Duration;
use std::net::UdpSocket;
use std::str;
use std::thread;
use std::time;

extern crate sdl2; 
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::Canvas;
use sdl2::video::Window;

mod global_state;
use global_state::GlobalState;


fn main() {

    let (sdl_context, mut canvas) = init_window();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut global_state = GlobalState::default();

    let socket = UdpSocket::bind("127.0.0.1:11111").unwrap();
    register_reciever(socket);

    while global_state.is_running() {
        canvas.clear();
        for event in event_pump.poll_iter() {
            handle_events(&mut global_state, event);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn register_reciever(socket: UdpSocket) {
    thread::spawn(move || {
        let mut buf = [0; 50];
        loop {
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            let rec = str::from_utf8(&buf[0 .. amt]).unwrap();
            println!("Recieved: {:?} from {}", rec, src);

            let wait_time = time::Duration::from_millis(10000);
            thread::sleep(wait_time);

            let return_str = format!("{}{}", rec, "-recieved");
            let bytes = return_str.as_bytes();
            socket.send_to(bytes, src).unwrap();
        } 
    });  
}

fn init_window() -> (Sdl, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_system = sdl_context.video().unwrap();

    let window = video_system.window("client", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    return (sdl_context, canvas);
}

fn handle_events(global_state: &mut GlobalState, event: Event) {
    match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            global_state.shutdown();
        },
        Event::MouseButtonDown { mouse_btn: MouseButton::Left, ..  } => {
            println!("{:?}", event);
        }
        _ => {}
    }
}
