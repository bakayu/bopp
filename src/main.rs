use bopp::animation::{Animation, CustomAnimation};
use bopp::animation_loader::AnimationLoader;
use bopp::renderer::Renderer;
use bopp::terminal::{Terminal, TerminalEvent};
use std::io;
use std::path::Path;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new()?;
    let mut renderer = Renderer::new();

    // Load animations from ./animations directory
    let animations_dir = Path::new("./animations");
    let mut animations: Vec<Box<dyn Animation>> = Vec::new();

    match AnimationLoader::load_from_directory(animations_dir) {
        Ok(configs) => {
            for config in configs {
                animations.push(Box::new(CustomAnimation::new(config)));
            }
        }
        Err(e) => {
            eprintln!("Warning: Could not load animations from directory: {}", e);
            eprintln!("Creating example animations...");
        }
    }

    if animations.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No animations found. Please add .json files to ./animations directory",
        ));
    }

    terminal.enter()?;
    let result = run_animation(&mut terminal, &mut renderer, animations);
    terminal.exit()?;

    result
}

fn run_animation(
    terminal: &mut Terminal,
    renderer: &mut Renderer,
    animations: Vec<Box<dyn Animation>>,
) -> io::Result<()> {
    let frame_duration = Duration::from_millis(100);
    let mut current_animation_index = 0;

    loop {
        match terminal.poll_event()? {
            TerminalEvent::Quit => break,
            TerminalEvent::SwitchAnimation(index) => {
                if index < animations.len() {
                    current_animation_index = index;
                }
            }
            TerminalEvent::None => {}
        }

        let animation = &animations[current_animation_index];
        let frame = animation.get_current_frame();

        let info = format!(
            "Press 'q' to quit | Press 1-{} to switch | Current: {} ({})",
            animations.len(),
            current_animation_index + 1,
            animation.name()
        );

        renderer.render(terminal, frame, &info)?;
        animation.next_frame();

        std::thread::sleep(frame_duration);
    }

    Ok(())
}
