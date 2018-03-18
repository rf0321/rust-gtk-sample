extern crate gtk;
extern crate gio;

use gtk::{ WidgetExt, WindowExt };
use gio::{ ApplicationExt };


fn make_hello_window() {
    match gtk::Application::new("com.github.ItinoseSan.rs_gui_app", gio::APPLICATION_HANDLES_OPEN) {
        Ok(app) => {
            //アプリケーションへ、activateシグナルに対するハンドラを設定する
            app.connect_activate(|app| {
                //ウィンドウを生成する
                let win = gtk::ApplicationWindow::new(&app);
                win.set_default_size(800,600);
                //ウィンドウのタイトルに表示する文字列を設定する
                win.set_title("Hello Gtk-rs");
                //ウィンドウとその中身全てを可視状態にする
                win.show_all();
            });

            //アプリケーションを開始、アプリケーションへactivateシグナルがエミットされる
            app.run(&[""]);
        },
        Err(_) => {
            println!("Application start up error");
        }
    };
}

fn main() {
   make_hello_window(); 
}