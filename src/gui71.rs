/*
  Author: Mihajlo Zekovic
  https://www.linkedin.com/in/mihajlo-zekovic/
  2023-09-27
  
  7 GUIs - [1] Counter
  cargo run --bin gui71
*/

#![allow(unused)]
#![windows_subsystem = "windows"]

use fltk::{prelude::*, *, enums::{CallbackTrigger, Event, Font, Color}};
use fltk_theme::{
		widget_themes, WidgetScheme, SchemeType,
		WidgetTheme, ThemeType, color_themes, ColorTheme};

const WIDTH: i32 = 250;
const HEIGHT: i32 = 70;

fn main() {
	
	let mut app = app::App::default()/*.with_scheme(app::Scheme::Gtk)*/;
	let widget_theme = WidgetTheme::new(ThemeType::Metro);
	widget_theme.apply();
	//app.set_scheme(app::Scheme::Gtk);
	//let color_theme = ColorTheme::new(color_themes::GRAY_THEME);
	//color_theme.apply();
	app::set_visible_focus(true);
	
	let fnt = Font::load_font("verdana.ttf").or_else(|_| Font::load_font("../../verdana.ttf")).unwrap();
	Font::set_font(Font::Helvetica, &fnt);
	app::set_font_size(11);
	let mut win = window::Window::new(0, 0, WIDTH, HEIGHT, "Counter").center_screen();
	//win.make_resizable(true);
	
	let mut txt_counter = input::IntInput::new(35, 20, 120, 24, "");
	//txt_counter.set_value("0");
	let mut btn_increase = button::Button::new(165, 20, 50, 24, "&Count");
	btn_increase.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
	
	btn_increase.set_callback(move |btn| {
		let mut number: i128 = txt_counter.value().parse().unwrap_or(0);
		number += 1;
		txt_counter.set_value(number.to_string().as_str());
	});
	
	win.end();
	win.show();
	
	app.run().unwrap();
}
