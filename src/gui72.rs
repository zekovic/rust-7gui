/*
  Author: Mihajlo Zekovic
  https://www.linkedin.com/in/mihajlo-zekovic/
  2023-10-02
  
  7 GUIs - [2] Temperature Converter
  cargo run --bin gui72
*/

#![allow(unused)]
#![windows_subsystem = "windows"]

use fltk::{prelude::*, *, enums::{CallbackTrigger, Event, Font, Color}};
use fltk_theme::{
		widget_themes, WidgetScheme, SchemeType,
		WidgetTheme, ThemeType, color_themes, ColorTheme};

const WIDTH: i32 = 250;
const HEIGHT: i32 = 100;

fn main() {
	
	let mut app = app::App::default();
	let widget_theme = WidgetTheme::new(ThemeType::Metro);
	widget_theme.apply();
	app::set_visible_focus(true);
	
	let fnt = Font::load_font("verdana.ttf").or_else(|_| Font::load_font("../../verdana.ttf")).unwrap();
	Font::set_font(Font::Helvetica, &fnt);
	app::set_font_size(11);
	let mut win = window::Window::new(0, 0, WIDTH, HEIGHT, "Temperature Converter").center_screen();
	//win.make_resizable(true);
	
	let mut txt_c = input::FloatInput::new(90, 20, 120, 24, "Celsius: ");
	let mut txt_f = input::FloatInput::new(90, 50, 120, 24, "Fahrenheit: ");
	txt_c.set_value("0");
	txt_f.set_value("0");
	
	win.end();
	win.show();
	
	txt_c.set_trigger(CallbackTrigger::Changed);
	txt_f.set_trigger(CallbackTrigger::Changed);
	
	let mut txt_f_clone = txt_f.clone();
	txt_c.set_callback(move |txt| {
		match txt.value().parse::<f64>() {
			Ok(c) => txt_f_clone.set_value(format!("{:.2}", c * (9.0 / 5.0) + 32.0).as_str()),
			Err(_) => txt_f_clone.set_value("")
		}
	});
	
	txt_f.set_callback(move |txt| {
		match txt.value().parse::<f64>() {
			Ok(f) => txt_c.set_value(format!("{:.2}", (f - 32.0) * (5.0 / 9.0) ).as_str()),
			Err(_) => txt_c.set_value("")
		}
	});
	
	app.run().unwrap();
}
