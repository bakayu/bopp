mod animation;
mod renderer;
mod terminal;

use animation::RatAnimation;
use renderer::Renderer;
use std::io;
use std::time::Duration;
use terminal::Terminal;

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new()?;
    let mut renderer = Renderer::new();
    let animation = RatAnimation::new();

    terminal.enter()?;

    let result = run_animation(&mut terminal, &mut renderer, &animation);

    terminal.exit()?;

    result
}

fn run_animation(
    terminal: &mut Terminal,
    renderer: &mut Renderer,
    animation: &RatAnimation,
) -> io::Result<()> {
    let frame_duration = Duration::from_millis(100);

    loop {
        if terminal.should_quit()? {
            break;
        }

        let frame = animation.get_current_frame();
        renderer.render(terminal, frame)?;
        animation.next_frame();

        std::thread::sleep(frame_duration);
    }

    Ok(())
}
