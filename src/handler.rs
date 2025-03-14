use crate::{
    app::{App, AppResult},
    constants::Pages,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `q`
        KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        KeyCode::Right | KeyCode::Char('l') => app.board.cursor_right(),
        KeyCode::Left | KeyCode::Char('h') => app.board.cursor_left(),
        KeyCode::Up | KeyCode::Char('k') => {
            if app.current_page == Pages::Home {
                app.menu_cursor_up()
            } else {
                app.board.cursor_up()
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.current_page == Pages::Home {
                app.menu_cursor_down()
            } else {
                app.board.cursor_down()
            }
        }
        KeyCode::Char(' ') | KeyCode::Enter => {
            if app.current_page != Pages::Home {
                app.board.select_cell()
            } else {
                app.menu_select()
            }
        }
        KeyCode::Char('?') => {
            if app.current_page != Pages::Home {
                app.show_help_popup = true
            }
        }
        KeyCode::Char('x') => {
            if app.current_page == Pages::Solo || app.current_page == Pages::Bot {
                app.show_help_popup = false;
            } else {
                app.current_page = Pages::Home
            }
        }
        KeyCode::Char('r') => app.restart(),
        KeyCode::Esc => {
            app.board.unselect_cell();
            app.show_help_popup = false;
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
