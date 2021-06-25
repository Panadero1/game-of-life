use winit::{
    event::{Event, WindowEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::PhysicalSize,
};
use pixels::{Pixels, Error, SurfaceTexture};
use winit_input_helper::WinitInputHelper;
use random::Source;

const SCALE: u32 = 7;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    window.set_title("Game of life");

    let mut width = 600;
    let mut height = 400;

    window.set_inner_size(PhysicalSize::new(width, height));

    let surface_texture = SurfaceTexture::new(width, height, &window);

    let mut input = WinitInputHelper::new();

    let mut pixels = Pixels::new(width, height, surface_texture)?;

    let mut alive = false;

    let mut play = false;

    let mut rand = random::default().seed([112, 6432]);
    

    let mut grid: Vec<bool> = vec![false; (width * height) as usize];

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::default();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {

                let mut i = 0;


                let a = pixels.get_frame();
                for pix in a.chunks_exact_mut(4) {
                    pix.copy_from_slice(&[if grid[i] {255} else {0}; 4]);
                    i += 1;
                }
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            },
            _ => (),
        }
        if input.update(&event) {
            if play {
                update_grid(&mut grid, &width, &height);
            }
            let mouse_coords = input.mouse();
            if input.key_pressed(VirtualKeyCode::Space) {
                play = false;
                update_grid(&mut grid, &width, &height);
            }
            if input.key_pressed(VirtualKeyCode::P) {
                play = !play;
            }
            if input.key_pressed(VirtualKeyCode::R) {
                play = false;
                for cell in &mut grid {
                    *cell = rand.read_f64().round() == 1_f64;
                }
            }
            if input.key_pressed(VirtualKeyCode::C) {
                play = false;
                for cell in &mut grid {
                    *cell = false;
                }
            }
            if input.mouse_pressed(0) {if let Some(coords) = mouse_coords {
                play = false;
                let pos = (coords.0 as u32 / SCALE) as usize + ((coords.1 as u32 / SCALE) * width) as usize;
                alive = match grid.get(pos) {
                    Some(living) => !living,
                    None => false,
                }
            }}
            if input.mouse_held(0) {
                play = false;
                if let Some(coords) = mouse_coords {
                    let pos = ((coords.0).clamp(0_f32, (width * SCALE - 1) as f32) as u32 / SCALE) as usize + ((coords.1 as u32 / SCALE) * width) as usize;
                    match grid.get_mut(pos) {
                        Some(_) => grid[pos] = alive,
                        None => (),
                    };
                }
            }
            if let Some(size) = input.window_resized() {
                play = false;
                width = size.width / SCALE;
                height = size.height / SCALE;
                pixels.resize_surface(width * SCALE, height * SCALE);
                pixels.resize_buffer(width, height);
                grid.resize((width * height) as usize, false);
            }
            window.request_redraw();
        }
    });

}

fn update_grid(grid: &mut Vec<bool>, width: &u32, height: &u32) {
    
    let mut new_grid = vec![false; (width * height) as usize];

    for i in 0..grid.len() {
        let mut alive_neighbors = 0;
        
        let width_signed = *width as i32;
        
        let neighbor_positions = [-1, -1 - width_signed, 0 - width_signed, 1 - width_signed, 1, 1 + width_signed, width_signed, -1 + width_signed];

        for neighbor_pos in &neighbor_positions {
            let pos = (i as i32 + neighbor_pos) as usize;
            match grid.get(pos) {
                Some(_) => if grid[pos] {alive_neighbors += 1},
                None => (),
            };
        }

        new_grid[i as usize] = if grid[i as usize] {
            alive_neighbors == 2 || alive_neighbors == 3
        } else {
            alive_neighbors == 3
        };
    }


    *grid = new_grid;
}