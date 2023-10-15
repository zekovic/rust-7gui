/*
  Author: Mihajlo Zekovic
  https://www.linkedin.com/in/mihajlo-zekovic/
  2023-10-03
  
  7 GUIs - [4] Timer
  cargo run --bin gui74
*/

#![allow(unused)]
#![windows_subsystem = "windows"]

use fltk::{prelude::*, *, enums::{CallbackTrigger, Event, Font, Color, Align}};
use fltk::misc::*;
use fltk_theme::{
		widget_themes, WidgetScheme, SchemeType,
		WidgetTheme, ThemeType, color_themes, ColorTheme};

const WIDTH: i32 = 330;
const HEIGHT: i32 = 150;

fn main() {
	let mut app = app::App::default().with_scheme(app::Scheme::Gtk);
	let widget_theme = WidgetTheme::new(ThemeType::Blue);
	widget_theme.apply();
	app::set_visible_focus(true);
	
	let fnt = Font::load_font("verdana.ttf").or_else(|_| Font::load_font("../../verdana.ttf")).unwrap();
	Font::set_font(Font::Helvetica, &fnt);
	app::set_font_size(11);
	let mut win = window::Window::new(0, 0, WIDTH, HEIGHT, "Timer").center_screen();
	win.make_resizable(true);
	
	let mut progress_bar = Progress::new(100, 15, 200, 20, "");
	progress_bar.set_align(Align::Inside);
	progress_bar.visible_focus(false);
	progress_bar.set_minimum(0.0);
	progress_bar.set_maximum(500.0);
	progress_bar.set_color(Color::from_u32(0xDDDDDD));
	progress_bar.set_selection_color(Color::from_u32(0x55AAEE));
	progress_bar.set_frame(enums::FrameType::EmbossedBox);
	
	let mut lbl_duration = frame::Frame::new(30, 40, 80, 24, "");
	//lbl_duration.visible_focus(false);
	
	let mut lbl_progress = frame::Frame::new(15, 15, 80, 20, "Elapsed Time: ");
	
	let mut slider = valuator::HorNiceSlider::new(100, 70, 200, 24, "Duration: ");
	slider.set_align(Align::Left);
	slider.set_minimum(0.005);
	slider.set_maximum(1.0);
	slider.set_value(0.5);
	lbl_duration.set_label(format!("{:.3}s", slider.value()).as_str());
	
	let mut btn_reset = button::Button::new(200, 110, 100, 24, "&Reset");
	btn_reset.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
	
	win.end();
	win.show();
	
	
	let mut progress_bar_clone = progress_bar.clone();
	btn_reset.set_callback(move |btn| {
		progress_bar_clone.set_value(0.0);
		progress_bar_clone.set_label(format!("0/{:.0}", progress_bar_clone.maximum()).as_str());
	});
	
	let slider_clone = slider.clone();
	let progress_timer = app::add_timeout3(0.5, move |hndl| {
		
		progress_bar.set_value(progress_bar.value() + 1.0);
		if progress_bar.value() > progress_bar.maximum() {
			progress_bar.set_value(progress_bar.maximum());
		}
		progress_bar.set_label(format!("{:.0}/{:.0}", progress_bar.value(), progress_bar.maximum()).as_str());
		
		app::repeat_timeout3(slider_clone.value(), hndl);
	});
	//app::remove_timeout3(progress_timer);
	
	slider.set_callback(move |sld| {
		//app::remove_timeout3(progress_timer);
		//app::repeat_timeout3(sld.value(), progress_timer);
		lbl_duration.set_label(format!("{:.3}s", sld.value()).as_str());
	});
	
	app.run().unwrap();
}
