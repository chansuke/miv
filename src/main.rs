use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;

use miv::editor::Editor;

fn main() {
    Editor::default().run();
}
