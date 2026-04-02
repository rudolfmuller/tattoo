use tattoo::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init
    let screen = terminal::Terminal::new()?;

    let mut master = surface::Surface::new('.', Scale { w: 30, h: 10 });
    master.set_terminal(screen);

    let mut window = surface::Surface::new(' ', Scale { w: 23, h: 5 });
    draw::border_layout(
        &mut window,
        Border {
            tl: '╭',
            tr: '╮',
            bl: '└',
            br: '┘',
            h: '─',
            v: '│',
        },
    );
    draw::label(&mut window, Position { x: 1, y: 0 }, "Question");
    draw::label(
        &mut window,
        Position { x: 1, y: 1 },
        "Do you want a tattoo?",
    );

    master.blit(&window, Position { x: 5, y: 2 });
    for _ in 0..100 {
        master.flip()?;
        std::thread::sleep(std::time::Duration::from_millis(42));
    }

    Ok(())
}
