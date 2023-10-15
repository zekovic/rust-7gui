/*
  Author: Mihajlo Zekovic
  https://www.linkedin.com/in/mihajlo-zekovic/
  2023-10-03
  
  7 GUIs - [3] Flight Booker
  cargo run --bin gui73
*/

#![allow(unused)]
#![windows_subsystem = "windows"]

use fltk::{prelude::*, *, enums::{CallbackTrigger, Event, Font, Color}};
use fltk_calendar::calendar;
use fltk_theme::{
		widget_themes, WidgetScheme, SchemeType,
		WidgetTheme, ThemeType, color_themes, ColorTheme};
use chrono::prelude::*;

const WIDTH: i32 = 200;
const HEIGHT: i32 = 150;

fn main() {
	
	let mut app = app::App::default();
	let widget_theme = WidgetTheme::new(ThemeType::Metro);
	widget_theme.apply();
	app::set_visible_focus(true);
	
	let fnt = Font::load_font("verdana.ttf").or_else(|_| Font::load_font("../../verdana.ttf")).unwrap();
	Font::set_font(Font::Helvetica, &fnt);
	app::set_font_size(11);
	let mut win = window::Window::new(0, 0, WIDTH, HEIGHT, "Book Flight").center_screen();
	//win.make_resizable(true);
	
	let mut cmb_flight_type = menu::Choice::new(20, 10, 120, 24, "").with_id("cmb_flight_type");
	cmb_flight_type.add_choice("One-way flight");
	cmb_flight_type.add_choice("Return flight");
	
	let mut txt_date_go = input::Input::new(20, 40, 120, 24, "").with_id("txt_date_go");
	let mut btn_date_go = button::Button::new(150, 40, 24, 24, "...").with_id("btn_date_go");
	let mut txt_date_return = input::Input::new(20, 70, 120, 24, "").with_id("txt_date_return");
	let mut btn_date_return = button::Button::new(150, 70, 24, 24, "...").with_id("btn_date_return");
	
	txt_date_go.deactivate();
	btn_date_go.deactivate();
	txt_date_return.deactivate();
	btn_date_return.deactivate();
	
	let mut btn_book = button::Button::new(20, 100, 50, 24, "&Book").with_id("btn_book");
	btn_book.deactivate();
	
	btn_date_go.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
	btn_date_return.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
	btn_book.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
	
	win.end();
	win.show();
	
	let win2 = win.clone();
	let mut txt_date_go2 = txt_date_go.clone();
	btn_date_go.set_callback(move |btn| {
		let dlg_cal = calendar::Calendar::new(win2.x() + btn.x() - 70 , win2.y() + btn.y() + btn.h() + 32);
		let date = dlg_cal.get_date();
		if let Some(date) = date {
			txt_date_go2.set_value(format!("{:?}", date).as_str());
		}
		check_all();
	});
	
	let win2 = win.clone();
	let mut txt_date_return2 = txt_date_return.clone();
	btn_date_return.set_callback(move |btn| {
		let dlg_cal = calendar::Calendar::new(win2.x() + btn.x() - 70 , win2.y() + btn.y() + btn.h() + 32);
		let date = dlg_cal.get_date();
		if let Some(date) = date {
			txt_date_return2.set_value(format!("{:?}", date).as_str());
		}
		check_all();
	});
	
	let mut txt_date_go2 = txt_date_go.clone();
	let mut btn_date_go2 = btn_date_go.clone();
	let mut txt_date_return2 = txt_date_return.clone();
	let mut btn_date_return2 = btn_date_return.clone();
	cmb_flight_type.set_callback(move |cmb| {
		txt_date_go2.activate();
		btn_date_go2.activate();
		check_all();
	});
	
	
	txt_date_go.set_trigger(CallbackTrigger::Changed);
	txt_date_return.set_trigger(CallbackTrigger::Changed);
	
	txt_date_go.set_callback(|txt| {
		check_all();
	});
	txt_date_return.set_callback(|txt| {
		check_all();
	});
	
	btn_book.set_callback(|btn| {
		book_flight();
	});
	
	app.run().unwrap();
}

fn check_all() {
	let mut cmb_flight_type: menu::Choice = app::widget_from_id("cmb_flight_type").unwrap();
	let mut txt_date_go: input::Input = app::widget_from_id("txt_date_go").unwrap();
	let mut btn_date_go: button::Button = app::widget_from_id("btn_date_go").unwrap();
	let mut txt_date_return: input::Input = app::widget_from_id("txt_date_return").unwrap();
	let mut btn_date_return: button::Button = app::widget_from_id("btn_date_return").unwrap();
	let mut btn_book: button::Button = app::widget_from_id("btn_book").unwrap();
	
	let go_unix: i64 = get_unix(&txt_date_go.value());
	let return_unix: i64 = get_unix(&txt_date_return.value());
	let mut book_valid = true;
	
	if cmb_flight_type.value() == 0 {
		txt_date_return.deactivate();
		btn_date_return.deactivate();
	} else {
		txt_date_return.activate();
		btn_date_return.activate();
	}
	
	if go_unix < 0 {
		txt_date_go.set_text_color(Color::from_u32(0xDD5555));
	} else {
		txt_date_go.set_text_color(Color::from_u32(0));
	}
	if return_unix < 0 {
		txt_date_return.set_text_color(Color::from_u32(0xDD5555));
	} else {
		txt_date_return.set_text_color(Color::from_u32(0));
	}
	txt_date_go.redraw();
	txt_date_return.redraw();
	
	if go_unix < 0 || (cmb_flight_type.value() == 1 && (return_unix < 0 || go_unix > return_unix) ) {
		book_valid = false;
	} else {
		book_valid = true;
	}
	
	if book_valid {
		btn_book.activate()
	} else {
		btn_book.deactivate();
	}
}

fn get_unix(date: &String) -> i64 {
	match NaiveDateTime::parse_from_str(format!("{} 00:00:00", &date).as_str() , "%F %H:%M:%S") { // "%Y-%m-%d %H:%M:%S"
		Ok(result) => result.timestamp(),
		Err(_) => -1
	}
}

fn book_flight() {
	let mut cmb_flight_type: menu::Choice = app::widget_from_id("cmb_flight_type").unwrap();
	let mut txt_date_go: input::Input = app::widget_from_id("txt_date_go").unwrap();
	let mut txt_date_return: input::Input = app::widget_from_id("txt_date_return").unwrap();
	
	if cmb_flight_type.value() == 0 {
		dialog::message_default(
			format!("You have booked a one-way flight on {}.",
				txt_date_go.value()).as_str());
	}
	
	if cmb_flight_type.value() == 1 {
		dialog::message_default(
			format!("You have booked a return flight on {} to {}.",
				txt_date_go.value(), txt_date_return.value()).as_str());
	}
	
}