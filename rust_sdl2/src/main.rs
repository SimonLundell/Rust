use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod test; // "include" mod short for module

fn generic_function<Type1>(value: Type1) -> Type1 {
    return value;
}

fn generic_function2<'t, Type2>(value: &'t Type2) -> &'t Type2 { // 't is referencing the lifetime of the "borrowed object" we pass into the function
    return value;
}

fn main() -> Result<(), String> { // -> return type from main with templated arguments () means "nothing"
    let sdl = sdl2::init(); //.unwrap() <- unwrap shows the error message if any, we create our own handling below
    println!("{}", test::other_function()); // function from test.rs
    let testval: i32 = 32;
    println!("{}", generic_function(testval));

    let r:&i32 = &testval;
    println!("{}", generic_function2(r));
    
    let vec = vec![1 as i8,2,3,4]; // as i8 makes the compiler deduce full vector should be i8 () has to be same type)
    
    // Custom response handler. Takes result stored in sdl and matches it with Ok/Err (default returns in rust). Then stores this in sdl_context
    let sdl_context = match sdl {
        Ok(result) => result,
        Err(message) => { 
            println!("{}", message); 
            return Ok(()); // Has to have "Ok(()) if main return-type is not void. Means return empty Ok... "
        }
    };

    let video_subsystem = sdl_context.video()?; // ? means it sees inside the "ok". if not OK it will return the error type

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(()) // No ; needed because ; "eats" the value and nothing is returned. We need to have a value due to main needing a return type.
}