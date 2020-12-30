extern crate ncurses;

use ncurses::*;

/*
    UIComponent
*/
trait UIComponent {
    fn get_handle(&self) -> WINDOW;
    fn draw(&self);

    fn render(&self) {
        self.draw();
        wrefresh(self.get_handle());
    }

    fn print(&self, s: &str, x: i32, y: i32) {
        // TODO: make sure text doesn't collide with border
        mvwprintw(self.get_handle(), y, x, s);
    }
}

/*
    Box
*/
struct Box {
    handle: WINDOW,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl UIComponent for Box {
    fn get_handle(&self) -> WINDOW {
        self.handle
    }

    fn draw(&self) {
        box_(self.handle, 0, 0);
        self.print("Hello world!", 1, 1);
    }
}

impl Drop for Box {
    fn drop(&mut self) {
        let sp = ' ' as chtype;
        wborder(self.handle, sp, sp, sp, sp, sp, sp, sp, sp);
        wrefresh(self.handle);
        delwin(self.handle);
    }
}

/*
    Screen
*/
pub struct Screen {
    raw_screen: SCREEN,
    width: i32,
    height: i32,
}

impl Screen {
    pub fn initialize() -> Screen {
        let raw_screen = initscr();
        cbreak();
        noecho();
        keypad(raw_screen, true);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        Screen {
            raw_screen: raw_screen,
            width: COLS(),
            height: LINES(),
        }
    }

    pub fn run(&self) {
        refresh();

        let simple_box = Box {
            handle: newwin(10, 10, 10, 10),
            x: 10,
            y: 10,
            width: 10,
            height: 10,
        };
        simple_box.render();

        let mut ch = getch();
        while ch != KEY_F(1) {
            simple_box.render();
            ch = getch();
        }
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        endwin();
    }
}
