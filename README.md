# tattoo

<b>Low-level Rust TUI/CUI library
<br>
POSIX - Only (linux)
</b>
<br>

<b>WARNING:</b> This library is primarily intended for our personal project.
I just want to share it with you so you can test it and, if you like, give me some feedback (or even use it)
<br>

install: ```$ cargo add tattoo```.
<br>
documentation on: https://docs.rs/tattoo/
<br>
### Example
```rust
use tattoo::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    
    master.flip()?;

    Ok(())
}
```
