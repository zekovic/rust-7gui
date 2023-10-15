/*
  Author: Mihajlo Zekovic
  https://www.linkedin.com/in/mihajlo-zekovic/
  2023-10-07
  
  7 GUIs - [6] Circle Drawer
  cargo run --bin gui76
*/

#![allow(unused)]
#![windows_subsystem = "windows"]

use fltk::{prelude::*, *, enums::{CallbackTrigger, Event, Font, Color, Align}, draw::Offscreen, frame::Frame};
use fltk_theme::{
		widget_themes, WidgetScheme, SchemeType,
		WidgetTheme, ThemeType, color_themes, ColorTheme};

use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;

#[derive(Debug)]
struct Circle {
	x: i32,
	y: i32,
	r: i32, // R = r * 2
	c: u8,
}

const WIDTH: i32 = 550;
const HEIGHT: i32 = 350;

fn main() {
	let mut app_main = app::App::default();
	let widget_theme = WidgetTheme::new(ThemeType::Blue);
	widget_theme.apply();
	app::set_visible_focus(true);
	
	let fnt = Font::load_font("verdana.ttf").or_else(|_| Font::load_font("../../verdana.ttf")).unwrap();
	Font::set_font(Font::Helvetica, &fnt);
	app::set_font_size(11);
	
	let mut win = window::Window::new(0, 0, WIDTH, HEIGHT, "Circle Drawing").center_screen();
	
	let mut btn_undo = button::Button::new(10, 15, 80, 24, "&Undo");
	btn_undo.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
	btn_undo.deactivate();
	let mut btn_redo = button::Button::new(100, 15, 80, 24, "&Redo");
	btn_redo.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
	btn_redo.deactivate();
	
	let mut status_bar = frame::Frame::new(200, 35, 0, 0, "").with_align(Align::TopLeft);
	
	let mut plot = frame::Frame::new(10, 50, WIDTH-20, HEIGHT-60, "");
	plot.set_frame(enums::FrameType::BorderBox);
	
	win.end();
	win.show();
	app::set_visible_focus(true);
	
	let mut win_radius = window::Window::new(0, 0, 350, 80, "Edit radius").center_screen();
		
		win_radius.make_modal(true);
		let mut slider_radius = valuator::HorNiceSlider::new(100, 10, 200, 24, "Radius: ").with_id("slider_radius");
		slider_radius.set_align(Align::Left);
		slider_radius.set_minimum(15.0);
		slider_radius.set_maximum(128.0);
		slider_radius.set_value(25.0);
		
		let mut lbl_radius = frame::Frame::new(20, 12, 20, 20, "");
		lbl_radius.set_frame(enums::FrameType::BorderBox);
		
		let mut btn_apply_radius = button::Button::new(200, 40, 100, 24, "&Apply");
		btn_apply_radius.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
		
		let mut win_radius_tmp = win_radius.clone();
		btn_apply_radius.set_callback(move |btn| {
			win_radius_tmp.hide();
		});
		
	win_radius.end();
	
	let mut circle_arr: Vec<Circle> = Vec::new();
	let mut circle_arr_ptr = Rc::from(RefCell::from(circle_arr));
	
	let mut current_index: usize = 0;
	let mut top_index: usize = 0;
	let current_index_ptr = Rc::from(RefCell::from(current_index));
	let top_index_ptr = Rc::from(RefCell::from(top_index));
	
	let offs = draw::Offscreen::new(plot.width(), plot.height()).unwrap();
	#[cfg(not(target_os = "macos"))]
	{
		offs.begin();
		draw::draw_rect_fill(0, 0, plot.w(), plot.h(), Color::White);
		offs.end();
	}
	
	let offs = Rc::from(RefCell::from(offs));
	
	plot.draw({
		let offs = offs.clone();
		move |plot| {
			let mut offs = offs.borrow_mut();
			if offs.is_valid() {
				offs.rescale();
				offs.copy(plot.x(), plot.y(), plot.w(), plot.h(), 0, 0);
			} else {
				offs.begin();
				draw::draw_rect_fill(plot.x(), plot.y(), plot.w(), plot.h(), Color::White);
				offs.copy(0, 0, plot.w(), plot.h(), 0, 0);
				offs.end();
			}
		}
	});
	
	plot.handle({
		let mut x = 0;
		let mut y = 0;
		let mut r = 32;
		let mut rng = rand::thread_rng();
		let mut c: u8 = 0;
		let circle_arr_ptr = circle_arr_ptr.clone();
		let current_index_ptr = current_index_ptr.clone();
		let top_index_ptr = top_index_ptr.clone();
		let mut btn_undo = btn_undo.clone();
		let mut btn_redo = btn_redo.clone();
		let mut status_bar = status_bar.clone();
		let mut plot = plot.clone();
		let offs = offs.clone();
		move |f, ev| {
			match ev {
				Event::Push => {
					
					let offs = offs.borrow_mut();
					let coords = app::event_coords();
					let mut top_index = top_index_ptr.borrow_mut();
					let mut circle_arr = circle_arr_ptr.borrow_mut();
					let mut current_index = current_index_ptr.borrow_mut();
					
					if app::event_button() == 1 {
						c = rng.gen_range(56..160);
						offs.begin();
						draw::set_draw_color(Color::by_index(c));
						draw::set_line_style(draw::LineStyle::Solid, 3);
						x = coords.0 - plot.x();
						y = coords.1 - plot.y();
						draw::draw_arc(x - r, y - r, r*2, r*2, 0.0, 360.0);
						offs.end();
						plot.redraw();
						draw::set_line_style(draw::LineStyle::Solid, 0);
						
						*current_index += 1;
						*top_index = *current_index;
						if circle_arr.len() < *current_index {
							circle_arr.push(Circle { x, y, r, c });
						} else {
							circle_arr[*current_index - 1] = Circle { x, y, r, c };
						}
						btn_undo.activate();
						btn_redo.deactivate();
						//status_bar.set_label(format!("0 ... {} ... {} ... {} \t\t\t", *current_index, *top_index, circle_arr.len()).as_str());
						
					} else {
						let plot_x = coords.0 - plot.x();
						let plot_y = coords.1 - plot.y();
						
						for i in 0..*current_index {
							let item = &circle_arr[*current_index - i - 1];
							if plot_x.clamp(item.x - item.r, item.x + item.r) == plot_x && plot_y.clamp(item.y - item.r, item.y + item.r) == plot_y {
								if (item.x - plot_x).pow(2) + (item.y - plot_y).pow(2) <= item.r.pow(2) {
									slider_radius.set_value(item.r as f64);
									lbl_radius.set_color(Color::by_index(item.c));
									win_radius.set_pos(win.x() + 20, win.y() + 60);
									win_radius.show();
									while win_radius.shown() {
										app::wait();
									}
									let new_radius = slider_radius.value() as i32;
									if item.r != new_radius {
										circle_arr[*current_index - i - 1].r = new_radius;
										redraw_circles(&circle_arr, &current_index, &offs, &mut plot);
									}
									break;
								}
							}
						}
					}
					true
				},
				_ => false,
			}
		}
	});
	
	btn_undo.set_callback({
		let circle_arr_ptr = circle_arr_ptr.clone();
		let current_index_ptr = current_index_ptr.clone();
		let top_index_ptr = top_index_ptr.clone();
		let mut btn_undo = btn_undo.clone();
		let mut btn_redo = btn_redo.clone();
		let mut status_bar = status_bar.clone();
		let mut plot = plot.clone();
		let offs = offs.clone();
		
		move |btn| {
			let mut circle_arr = circle_arr_ptr.borrow_mut();
			let mut current_index = current_index_ptr.borrow_mut();
			let mut top_index = top_index_ptr.borrow_mut();
			
			*current_index -= 1;
			if *current_index == 0 {
				btn_undo.deactivate();
			}
			btn_redo.activate();
			redraw_circles(&circle_arr, &current_index, &offs.borrow_mut(), &mut plot);
			//status_bar.set_label(format!("0 ... {} ... {} ... {} \t\t\t", *current_index, *top_index, circle_arr.len()).as_str());
		}
	});
	
	btn_redo.set_callback({
		let circle_arr_ptr = circle_arr_ptr.clone();
		let current_index_ptr = current_index_ptr.clone();
		let top_index_ptr = top_index_ptr.clone();
		let mut btn_undo = btn_undo.clone();
		let mut btn_redo = btn_redo.clone();
		let mut status_bar = status_bar.clone();
		let mut plot = plot.clone();
		let offs = offs.clone();
		
		move |btn| {
			let mut circle_arr = circle_arr_ptr.borrow_mut();
			let mut current_index = current_index_ptr.borrow_mut();
			let mut top_index = top_index_ptr.borrow_mut();
			
			*current_index += 1;
			if *current_index == *top_index {
				btn_redo.deactivate();
			}
			btn_undo.activate();
			redraw_circles(&circle_arr, &current_index, &offs.borrow_mut(), &mut plot);
			//status_bar.set_label(format!("0 ... {} ... {} ... {} \t\t\t", *current_index, *top_index, circle_arr.len()).as_str());
		}
	});
	
	app_main.run().unwrap();
}

fn redraw_circles(circle_arr: &Vec<Circle>, current_index: &usize, offs: &Offscreen, plot: &mut frame::Frame) {
	
	offs.begin();
	draw::draw_rect_fill(0, 0, plot.w(), plot.h(), Color::White);
	for i in 0..*current_index {
		
		let item = &circle_arr[i];
		draw::set_draw_color(Color::by_index(item.c));
		draw::set_line_style(draw::LineStyle::Solid, 3);
		draw::draw_arc(item.x - item.r, item.y - item.r, item.r*2, item.r*2, 0.0, 360.0);
	}
	offs.copy(0, 0, plot.w(), plot.h(), 0, 0);
	offs.end();
	plot.redraw();
}