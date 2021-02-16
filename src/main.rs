use amethyst::prelude::*;
use amethyst::winit::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use amethyst::utils::application_root_dir;
use amethyst::window::WindowBundle;

use amethyst::{
    animation::AnimationBundle,
    assets::{PrefabLoaderSystemDesc, Processor},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::SpriteRender,
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    Application, GameDataBuilder,
};

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, _: StateData<'_, GameData<'_, '_>>) {
        println!("Starting game!");
    }

    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            match event {
                 Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. }, ..
                    } |
                    WindowEvent::CloseRequested => Trans::Quit,
                    _ => Trans::None,
                },
                _ => Trans::None,
            }
        } else {
            Trans::None
        }
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("Computing some more whoop-ass...");
        Trans::Quit
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");

    let assets_dir = app_root.join("assets/");
    let builder = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.008, 0.043, 0.067, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::build(assets_dir, GameState)?.build(builder)?;
    game.run();

    Ok(())
}