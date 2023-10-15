/*
  Author: Mihajlo Zekovic
  https://www.linkedin.com/in/mihajlo-zekovic/
  2023-10-06
  
  7 GUIs - [5] CRUD
  cargo run --bin gui75
*/

#![allow(unused)]
#![windows_subsystem = "windows"]

use movies::*;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use fltk::text::{TextBuffer};
use fltk::{prelude::*, *, enums::{CallbackTrigger, Event, Font, Color, Align}};
use fltk::frame::Frame;
use fltk::browser::{self, HoldBrowser};
use fltk_theme::{
		widget_themes, WidgetScheme, SchemeType,
		WidgetTheme, ThemeType, color_themes, ColorTheme};

use rusqlite::{Connection, Result, Error};
use chrono::prelude::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 550;

#[derive(Copy, Clone)]
enum GuiMsg {
    BtnSaveClick,
    BtnNewClick,
    BtnDelClick,
    TxtFindChange,
    ListSelection,
}

mod movies;

fn main() {
	//env::set_var("RUST_BACKTRACE", "full");
	
	let mut app = app::App::default();
	let widget_theme = WidgetTheme::new(ThemeType::Metro);
	widget_theme.apply();
	app::set_visible_focus(true);
	
	let fnt = Font::load_font("verdana.ttf").or_else(|_| Font::load_font("../../verdana.ttf")).unwrap();
	Font::set_font(Font::Helvetica, &fnt);
	app::set_font_size(11);
	let mut win = window::Window::new(0, 0, WIDTH, HEIGHT, "CRUD").center_screen().with_id("wnd1");
	win.make_resizable(true);
	
	
	let mut txt_title = input::Input::new(20, 30, 425, 24, "Title: ");
	txt_title.set_align(Align::TopLeft);
	
	let mut txt_date = input::Input::new(20, 73, 80, 24, "Date: ");
	txt_date.set_align(Align::TopLeft);
	let mut txt_genre = input::Input::new(110, 73, 335, 24, "Genre: ");
	txt_genre.set_align(Align::TopLeft);
	
	let mut txt_desc_editor = text::TextEditor::new(465, 30, 320, 74, "Description: ");
	let mut txt_desc = text::TextBuffer::default();
	txt_desc_editor.set_buffer(txt_desc.clone());
	txt_desc_editor.wrap_mode(text::WrapMode::AtBounds, 2);
	txt_desc_editor.set_align(Align::TopLeft);
	txt_desc_editor.set_tab_nav(true);
	txt_desc_editor.set_cursor_style(text::Cursor::Simple);
	
	let mut txt_find = input::Input::new(20, 115, 425, 24, "&Search: ");
	txt_find.set_align(Align::TopLeft);
	txt_find.set_trigger(CallbackTrigger::Changed);
	
	let mut btn_new = button::Button::new(525, 115, 80, 24, "&Create");
	let mut btn_save = button::Button::new(615, 115, 80, 24, "&Update");
	let mut btn_del = button::Button::new(705, 115, 80, 24, "&Delete");
	btn_new.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
	btn_save.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
	btn_del.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
	
	btn_new.activate();
	btn_save.deactivate();
	btn_del.deactivate();
	let mut list = browser::HoldBrowser::new(15, 150, WIDTH-30, HEIGHT-170, "").with_id("list01");
	
	let widths = &[60, 250, 50, 100];
	list.set_column_widths(widths);
	list.set_column_char('\t');
	
	let mut status_bar = frame::Frame::new(16, HEIGHT - 5, WIDTH - 30, 0, "").with_id("status_bar");
	status_bar.set_frame(enums::FrameType::BorderBox);
	status_bar.set_align(Align::TopLeft);
	
	let (snd, rcv) = app::channel();
	btn_new.emit(snd, GuiMsg::BtnNewClick);
	btn_save.emit(snd, GuiMsg::BtnSaveClick);
	btn_del.emit(snd, GuiMsg::BtnDelClick);
	txt_find.emit(snd, GuiMsg::TxtFindChange);
	list.emit(snd, GuiMsg::ListSelection);
	
	win.end();
	win.show();
	
	let movies_ptr = Rc::from(RefCell::new(movies::MoviesDB::new()));
	let mut movies_db = movies_ptr.borrow_mut();
	//let mut movies_db = movies::MoviesDB::new();
	let movies_arr: Vec<DataItem> = movies_db.load().unwrap_or(Vec::new());
	let movies_arr_rc_rcell = Rc::from(RefCell::from(movies_arr));
	
	let mut movies_arr = movies_arr_rc_rcell.borrow_mut();
	//load_to_list(&movies_arr_rc_rcell.borrow_mut());
	load_to_list(&movies_arr);
	
	while app::wait() {
		if let Some(msg) = rcv.recv() {
			match msg {
				GuiMsg::ListSelection => {
					let row_id = get_row_id();
					
					if list.selected_items().len() == 0 || row_id == 0 {
						txt_title.set_value("");
						txt_desc.set_text("");
						txt_date.set_value("");
						txt_genre.set_value("");
						btn_new.activate();
						btn_save.deactivate();
						btn_del.deactivate();
						continue;
					}
					let sel_index = list.selected_items()[0] - 1;
					
					if let Some(found_row) = movies_arr.iter().find(|&x| x.id == row_id) {
						txt_title.set_value(found_row.title.as_str());
						txt_desc.set_text(found_row.description.as_str());
						txt_date.set_value(format!("{}", found_row.date).as_str());
						txt_genre.set_value(found_row.genre.as_str());
						btn_new.deactivate();
						btn_save.activate();
						btn_del.activate();
					}
				},
				GuiMsg::BtnSaveClick => {
					if !validate_inputs(&txt_title.value(), &txt_date.value()) {
						continue;
					}
					let mut item = DataItem {
						id: get_row_id(),
						title: txt_title.value(),
						date: txt_date.value(),
						genre: txt_genre.value(),
						description: txt_desc.text()
					};
					let result_updated = movies_db.update(&mut item);
					if result_updated == Ok(true) {
						list.set_text(list.value(), format!("{:07}\t{}\t{}\t{}", item.id, item.title, item.date[0..4].to_string(), item.genre).as_str());
						if let Some(found_row) = movies_arr.iter().position(|x| x.id == item.id) {
							movies_arr[found_row] = item;
						}
					} else {
						dialog::message_default(format!("Error while updating row... \n{:?}", result_updated).as_str());
					}
				},
				GuiMsg::BtnNewClick => {
					if !validate_inputs(&txt_title.value(), &txt_date.value()) {
						continue;
					}
					
					let mut item = DataItem {
						id: 0,
						title: txt_title.value(),
						date: txt_date.value(),
						genre: txt_genre.value(),
						description: txt_desc.text()
					};
					let result_new_id = match movies_db.add(&mut item) {
						Ok(new_id) => {
							item.id = new_id;
							list.set_text(list.size(), format!("{:07}\t{}\t{}\t{}", item.id, item.title, item.date[0..4].to_string(), item.genre).as_str());
							list.add(" \t \t \t ");
							movies_arr.push(item);
							list.select(list.size() - 1);
							btn_new.deactivate();
							btn_save.activate();
							btn_del.activate();
							status_bar.set_label(get_status_text(movies_arr.len(), Some(list.size()) ).as_str() );
						},
						Err(error) => {
							dialog::message_default(format!("Error while adding row...\n{:?}", error).as_str());
						},
					};
				},
				GuiMsg::BtnDelClick => {
					let choice = dialog::choice2_default("Are you sure?", "No", "Yes", "");
					//println!("Result:{:?}", choice); // Yes -> Some(1) , No -> Some(0)
					if choice == Some(1) {
						let row_id = get_row_id();
						let position = list.value();
						let result_deleted = movies_db.delete(row_id);
						if result_deleted == Ok(true) {
							if let Some(found_row) = movies_arr.iter().position(|x| x.id == row_id) {
								movies_arr.remove(found_row);
							}
							list.remove(position);
							
							txt_title.set_value("");
							txt_desc.set_text("");
							txt_date.set_value("");
							txt_genre.set_value("");
							btn_new.deactivate();
							btn_save.deactivate();
							btn_del.deactivate();
							/*list.select(position);
							if position == list.size() {
								btn_del.deactivate();
							}*/
							status_bar.set_label(get_status_text(movies_arr.len(), Some(list.size()) ).as_str() );
						} else {
							dialog::message_default(format!("Error while deleting row... \n{:?}", result_deleted).as_str() );
						}
					}
				},
				GuiMsg::TxtFindChange => {
					let txt_find_val = txt_find.value();
					if txt_find_val == "" {
						load_to_list(&movies_arr);
					} else {
						list.clear();
						movies_arr.iter()
							.filter(move |&x| 
								x.title.to_lowercase().contains(&txt_find_val.to_lowercase())
								|| x.date.contains(&txt_find_val) 
								|| x.genre.to_lowercase().contains(&txt_find_val.to_lowercase()) 
							)
							.for_each(|item| {
								list.add(format!("{:07}\t{}\t{}\t{}", item.id, item.title, item.date[0..4].to_string(), item.genre).as_str());
						});
						list.add(" \t \t \t "); // for creating new rows on click
						list.select(1);
						status_bar.set_label(get_status_text(movies_arr.len(), Some(list.size()) ).as_str() );
					}
					//btn_new.deactivate();
					btn_save.deactivate();
					btn_del.deactivate();
				},
				_ => {},
			}
		}
	}
	
	app.run().unwrap();
	
}

fn get_row_id() -> i64 {
	let list_tmp: browser::HoldBrowser = app::widget_from_id("list01").unwrap();
	let row_text = list_tmp.selected_text().unwrap_or("".to_string());
	get_id_from_row_text(&row_text, list_tmp.column_char())
}

fn get_id_from_row_text(row_text: &String, split_char: char) -> i64 {
	let mut row_text_arr = row_text.split(split_char);
	
	match row_text_arr.next() {
		Some(x) => {
			match x.parse::<i64>() {
				Ok(x_int) => { x_int },
				Err(_) => { 0 },
			}
		},
		None => {0}
	}
}

fn load_to_list(data_arr: &RefMut<Vec<DataItem>>) {
	let mut list: browser::HoldBrowser = app::widget_from_id("list01").unwrap();
	let mut status_bar: frame::Frame = app::widget_from_id("status_bar").unwrap();
	
	list.clear();
	for i in 0..data_arr.len() {
		let item = &data_arr[i];
		list.add(format!("{:07}\t{}\t{}\t{}", item.id, item.title, item.date[0..4].to_string(), item.genre).as_str());
	}
	list.add(" \t \t \t "); // for creating new rows on click
	
	status_bar.set_label(get_status_text(data_arr.len(), None).as_str());
	//status_bar.redraw_label();
}

fn get_status_text(total_items: usize, found_items: Option<i32>) -> String {
	match found_items {
		Some(x) => format!("Total: {} Shown: {}\t\t\t", total_items, x - 1).to_string(),
		None => format!("Total: {}\t\t\t\t\t\t", total_items).to_string()
	}
}

fn validate_inputs(title: &String, date: &String) -> bool {
	
	if title.trim().to_string() == "" {
		dialog::message_default("Title is required");
		return false;
	}
	
	let date_valid = match NaiveDateTime::parse_from_str(format!("{} 00:00:00", &date).as_str() , "%F %H:%M:%S") {
		Ok(result) => true,
		Err(_) => false
	};
	if date_valid == false {
		dialog::message_default("Wrong date format - YYYY-MM-DD required");
		return false;
	}
	
	true
}