use tcod::colors::*;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("rogue-ts")
        .init();

    let mut tcod = Tcod { root };

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();
        tcod.root
            .put_char(player_x, player_y, '@', BackgroundFlag::None);
        tcod.root.flush();

        let exit = handle_keys(&mut tcod, &mut player_x, &mut player_y);
        if exit {
            break;
        }
    }
}

/**
 * true: exit game
 * false: continue game
 */
fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            tcod.root.set_fullscreen(!tcod.root.is_fullscreen());
        }
        Key { code: Escape, .. } => return true,

        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,

        _ => {}
    }

    false
}