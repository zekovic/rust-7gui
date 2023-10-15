/*
  Author: Mihajlo Zekovic
  https://www.linkedin.com/in/mihajlo-zekovic/
  2023-10-12
  
  7 GUIs - [7] Cells
  cargo run --bin gui77
*/

#![allow(unused)]
#![windows_subsystem = "windows"]

use fltk::{prelude::*, *, enums::{CallbackTrigger, Event, Font, Color, Align} };
use fltk_theme::{
		widget_themes, WidgetScheme, SchemeType,
		WidgetTheme, ThemeType, color_themes, ColorTheme};
//use fltk_table::*;
use fltk_table_031::*;
use std::{cell::RefCell, cell::RefMut, vec, mem::swap, ops::Deref};
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug)]
struct TableCell {
	row: i32,
	col: i32,
	val: String,
	result: Option<String>,
	func: Option<String>,
	range: Option<(i32, i32, i32, i32)>,
}

const WIDTH: i32 = 700;
const HEIGHT: i32 = 500;

mod fltk_table_031;

fn main() {
	let mut app_main = app::App::default();
	let widget_theme = WidgetTheme::new(ThemeType::Metro);
	widget_theme.apply();
	app::set_visible_focus(true);
	
	let fnt = Font::load_font("verdana.ttf").or_else(|_| Font::load_font("../../verdana.ttf")).unwrap();
	Font::set_font(Font::Helvetica, &fnt);
	app::set_font_size(11);
	
	let mut win = window::Window::new(0, 0, WIDTH, HEIGHT, "Cells").center_screen();
	win.make_resizable(true);
	
	let mut options = TableOpts {
		rows: 200, cols: 200, 
		editable: true,
		cell_border_color: Color::from_u32(0xAAAAAA),
		cell_align: Align::Left,
		cell_font: Font::by_name("verdana.ttf"), cell_font_size: 11, header_font_size: 11,
		cell_padding: 3, //  0.3.1
		header_frame: enums::FrameType::ThinUpBox,
		..Default::default()
		/*cell_color: (), cell_font_color: (), cell_font_size: (),
		cell_selection_color: (),
		header_font: (), header_frame: (), header_color: (),
		header_font_color: (), header_font_size: (), header_align: ()*/
	};
	let mut tbl = fltk_table_031::SmartTable::new(10, 20, WIDTH-20, HEIGHT-60, "");
	tbl.set_opts(options);
	tbl.set_col_width_all(120);
	//tbl.set_tab_cell_nav(true);
	
	
	win.end();
	win.show();
	app::set_visible_focus(true);
	
	win.set_callback(move |win| {
		if app::event() == enums::Event::Close {
			app_main.quit();
		}
	});
	
	let mut eval_cells: HashMap<(i32, i32), TableCell> = HashMap::new();
	let eval_cells_ptr = Rc::from(RefCell::from(eval_cells)).clone();
	
	
	let mut win_tmp = win.clone();
	let mut selected_editing: (i32, i32, String) = (0, 0, String::from(""));
	let mut tbl_tmp = tbl.clone();
	let selected_editing_ptr = Rc::from(RefCell::from(selected_editing));
	
	let mut tbl_input = tbl.input().clone().unwrap().clone(); // 0.31 - Error on tbl.input().handle(...)
	tbl_input.set_frame(enums::FrameType::BorderBox);
	//tbl_input.set_color(Color::from_u32(0xaaaadd));
	
	tbl_input.handle({
		let eval_cells_ptr = eval_cells_ptr.clone();
		let selected_editing_ptr = selected_editing_ptr.clone();
		move |txt, ev| {
			
			if ev == Event::KeyUp {
				//dbg!(app::event_key());
				let selected = tbl_tmp.get_selection();
				let mut selected_editing = selected_editing_ptr.borrow_mut();
				*selected_editing = (selected.0, selected.1, txt.value());
			}
			
			if ev == Event::KeyDown {
				//let updating_key = app::event_key_down(enums::Key::Tab) || app::event_key_down(enums::Key::Enter);
				
				if app::event_key_down(enums::Key::Tab) || app::event_key_down(enums::Key::Enter) {
					win_tmp.set_cursor(enums::Cursor::Default);
					let selected = tbl_tmp.get_selection();
					//println!("TAB || ENTER");
					//dbg!("Handle... {:?}", ev);
					//tbl_tmp.set_cell_value(selected.0, selected.1, txt.value().as_str());
					txt.hide();
					let mut eval_cells = eval_cells_ptr.borrow_mut();
					
					let mut opened_value = tbl_tmp.cell_value(selected.0, selected.1);
					if eval_cells.contains_key(&(selected.0, selected.1)) {
						opened_value = eval_cells.get(&(selected.0, selected.1)).unwrap().val.clone();
					}
					
					//if txt.value() != tbl_tmp.cell_value(selected.0, selected.1) {
					if txt.value() != opened_value {
						let updated_cell = get_updated_cell(&txt.value(), (selected.0, selected.1), &mut eval_cells);
						
						//println!("INPUT CALLBACK...");
						tbl_tmp.set_cell_value(selected.0, selected.1, updated_cell.clone().as_str());
						
						//let found_changes = evaluate_table(&tbl_tmp);
						evaluate_table(&tbl_tmp.data(), &mut eval_cells);
						update_table(&mut tbl_tmp, &eval_cells);
					}
					//println!("WHAT: {}", tbl_tmp.col_header_value(selected.1));
					
					//if app::event_key_down(enums::Key::Tab) {
					// for some reason "=Something" -> "VALUE" on Enter works only when if(Key::Tab) part is down ...
						let mut next_step = (selected.0 + 1, selected.1);
						txt.set_pos(txt.x(), txt.y() + tbl_tmp.row_height(next_step.0));
						if app::event_key_down(enums::Key::ShiftL) || app::event_key_down(enums::Key::ShiftR) {
							next_step = (selected.0 - 1, selected.1);
							txt.set_pos(txt.x(), txt.y()+1 - txt.height() - tbl_tmp.row_height(next_step.0));
							//txt.set_size(tbl_tmp.col_width(next_step.1), tbl_tmp.row_height(next_step.0));
						}
						//txt.set_size(tbl_tmp.col_width(next_step.1), tbl_tmp.row_height(next_step.0));
						txt.set_value(&tbl_tmp.cell_value(next_step.0, next_step.1));
						
					if app::event_key_down(enums::Key::Tab) {
						tbl_tmp.set_selection(next_step.0, next_step.1, next_step.0, next_step.1);
						txt.show();
					}
					return false;
				}
				if app::event_key_down(enums::Key::Escape) {
					//println!("ESC");
					txt.hide();
					win_tmp.set_cursor(enums::Cursor::Default);
					let selected = tbl_tmp.get_selection();
					let mut selected_editing = selected_editing_ptr.borrow_mut();
					//selected_editing.2 = tbl_tmp.cell_value(selected.0, selected.1);
					//txt.set_value(&tbl_tmp.cell_value(selected.0, selected.1));
					
					txt.set_value("");
					selected_editing.0 = -1;
					selected_editing.1 = -1;
					return true;
				}
				
				//return true;
			}
			
			// avoid ::Show to avoid triggering twice in a row
			if /*ev == Event::Show ||*/ ev == Event::Focus {
				let mut eval_cells = eval_cells_ptr.borrow_mut();
				let selected = tbl_tmp.get_selection();
				
				if eval_cells.contains_key(&(selected.0, selected.1)) {
					let found: &TableCell = &eval_cells[&(selected.0, selected.1)];
					//println!("Found [=...] {:?}", found);
					
					txt.set_value(&found.val);
				}
				return true;
			}
			
			false
			
	}});
	
	tbl.set_callback({
		let mut tbl_tmp = tbl.clone();
		let eval_cells_ptr2 = eval_cells_ptr.clone();
		let selected_editing_ptr = selected_editing_ptr.clone();
		move |tbl|{
			//println!("Callback... {:?}, {:?}, Visible: {}", tbl.input().value(), app::event(), tbl.input().visible());
			if app::event() == Event::Push {
				let mut selected_editing = selected_editing_ptr.borrow_mut();
				if (selected_editing.0 != -1 && selected_editing.1 != -1) {
					
					let mut eval_cells = eval_cells_ptr2.borrow_mut();
					
					//println!("tbl.set_callback");
					let input_val = selected_editing.2.clone();
					let input_row = selected_editing.0;
					let input_col = selected_editing.1;
					
					let mut opened_value = tbl_tmp.cell_value(input_row, input_col);
					if eval_cells.contains_key(&(input_row, input_col)) {
						opened_value = eval_cells.get(&(input_row, input_col)).unwrap().val.clone();
					}
					//if input_val != tbl_tmp.cell_value(input_row, input_col) {
					if input_val != opened_value {
						let updated_cell = get_updated_cell(&input_val, (input_row, input_col), &mut eval_cells);
						
						//println!("TBL CALLBACK...");
						tbl_tmp.set_cell_value(input_row, input_col, &updated_cell);
						
						evaluate_table(&tbl_tmp.data(), &mut eval_cells);
						update_table(&mut tbl_tmp, &eval_cells);
					}
				}
			}
	}});
	
	//let mut tbl_tmp = tbl.clone();
	let mut tbl_input_tmp = tbl_input.clone();
	win.handle(move |wnd, ev| {
		
		if ev == Event::Resize {
			tbl_input_tmp.hide();
		}
		false
	});
	
	
	app_main.run().unwrap();
}



fn get_updated_cell(input_val: &str, selected: (i32, i32), eval_cells: &mut HashMap<(i32, i32), TableCell>) -> String {
	
	if input_val.len() == 0 || &input_val[0..1] != "=" {
		//println!("nothing...");
		eval_cells.remove(&(selected.0, selected.1));
		return input_val.to_string();
	}
	//if input_val.len() > 0 && &input_val[0..1] == "=" {
	let mut cell_fn: Option<String> = None;
	let mut cell_range: Option<(i32, i32, i32, i32)> = None;
	let arg_start = input_val.find('(');
	let arg_end = input_val.find(')');
	let mut row_start = String::from("");
	let mut col_start = String::from("");
	let mut row_end = String::from("");
	let mut col_end = String::from("");
	let mut row_start_int = 0;
	let mut col_start_int = 0;
	let mut row_end_int = 0;
	let mut col_end_int = 0;
	let mut found_separator = false;
	let mut fn_str: &str = "";
	if arg_start != None && arg_end != None {
		let arg_start_int = arg_start.unwrap();
		let arg_end_int = arg_end.unwrap();
		
		if (arg_start_int > 1 && arg_start_int < arg_end_int) {
			fn_str = &input_val[1..arg_start_int];
			cell_fn = Some(fn_str.to_string().to_uppercase());
			let arg_str = &input_val[arg_start_int+1..arg_end_int /*-1*/];
			let mut i: usize = 0;
			let arg_chars = arg_str.as_bytes();
			for i in 0..arg_chars.len() {
				let letter_code = arg_chars[i];
				if letter_code == 58 {
					found_separator = true;
				}
				if letter_code >= 48 && letter_code <= 57 {
					if !found_separator {
						row_start = format!("{}{}", row_start, letter_code as char);
					} else {
						row_end = format!("{}{}", row_end, letter_code as char);
					}
				}
				
				if (letter_code >= 65 && letter_code <= 90) || (letter_code >= 97 && letter_code <= 122) {
					if !found_separator {
						col_start = format!("{}{}", col_start, letter_code as char);
						
					} else {
						col_end = format!("{}{}", col_end, letter_code as char);
					}
				}
				
			}
		}
		
	}
	col_start = col_start.to_uppercase();
	col_end = col_end.to_uppercase();
	let cell_fn_clone = cell_fn.clone().unwrap().clone();
	//println!("TEST: {} ( {}_{} : {}_{} )", cell_fn_clone, col_start, row_start, col_end, row_end);
	col_start_int = get_column_from_letters(&col_start);
	col_end_int = get_column_from_letters(&col_end);
	row_start_int = row_start.parse().unwrap_or(0);
	row_end_int = row_end.parse().unwrap_or(0);
	// 0.31 - first cell = 1
	row_start_int -= 1;
	row_end_int -= 1;
	
	if row_start_int > row_end_int {
		swap(&mut row_start_int, &mut row_end_int);
	}
	if col_start_int > col_end_int {
		swap(&mut col_start_int, &mut col_end_int);
	}
	
	cell_range = Some((row_start_int, col_start_int, row_end_int, col_end_int));
	
	eval_cells.insert((selected.0, selected.1), 
		TableCell { row: selected.0, col: selected.1, val: input_val.to_string(), result: None, func: cell_fn, range: cell_range });
	return String::from("[ VALUE ]");
	
}

fn get_column_from_letters(letters: &str) -> i32 {
	let ints = letters.as_bytes();
	let mut column: i32 = 0;
	let base26: i32 = 26;
	for i in 0..ints.len() {
		column += (ints[i] as i32 - 64) * base26.pow((ints.len() - i - 1) as u32);
	}
	column -= 1; // index starts at 0
	column
}

fn get_array_of_values(data: &Vec<Vec<String>>, range: &(i32, i32, i32, i32), eval_cells: &HashMap<(i32, i32), TableCell>) -> Option<Vec<f64>> {
	let mut values_arr: Vec<f64> = vec![];
	
	for i in range.0..range.2 + 1 {
		for j in range.1..range.3 + 1 {
			
			// if all are numbers it returns array of numbers
			// if some celle are formulas: 
			//	- if result is calculated - return it together with other numbers, 
			//	- if any of result is not calculated - return None
			let mut data_value: String = data[i as usize][j as usize].clone();
			if eval_cells.contains_key(&(i, j)) {
				let cell: &TableCell = eval_cells.get(&(i, j)).unwrap();
				if cell.result != None {
					data_value = cell.result.clone().unwrap().clone();
					//println!("Found calculated: {}", data_value);
				} else {
					//println!("Found empty");
					return None;
				}
			}
			match data_value.parse::<f64>() {
				Ok(parsed_val) => values_arr.push(parsed_val),
				Err(_) => {},
			}
		}
	}
	
	Some(values_arr)
}

fn evaluate_table(data: &Vec<Vec<String>>, eval_cells: &mut HashMap<(i32, i32), TableCell>) {
	
	let functions = vec!["SUM", "AVG", "PROD" /*, "DIV"*/];
	
	let mut eval_tmp: Vec<TableCell> = vec![];
	let mut found_formulas = true;
	let max_tries = 200;
	let mut tries_count = 0;
	while found_formulas && tries_count < max_tries {
		tries_count += 1;
		
		for ((row, col), cell) in &*eval_cells {
		//for ((row, col), cell) in eval_cells.iter_mut() {
			
			//println!("{:?}", cell);
			if cell.range != None  && cell.func != None {
				let function = cell.func.clone().unwrap().clone();
				if functions.contains(&function.as_str()) {
					let cell_range = &cell.range.unwrap();
					let range_values = get_array_of_values(&data, &cell_range, eval_cells);
					//println!("RANGE ARR: {:?}", range_values);
					if range_values != None {
						//println!("No func cells found... Can be calculated");
						let mut range_arr = range_values.clone().unwrap();
						let mut new_cell = TableCell {
							row: row.clone(), col: col.clone(), val: cell.val.clone(), 
							result: None, func: Some(function.clone()), range: Some(cell_range.clone()) };
						
						//let mut new_cell = cell.clone();
						
						if function == "SUM" {
							let result_val: f64 = range_arr.iter().sum();
							//cell.result = Some(format!("[ {} ]", result_val));
							//cell.deref().result = Some(format!("[ {} ]", result_val));
							new_cell.result = Some(format!("{}", result_val));
							
						}
						if function == "AVG" {
							let result_val: f64 = range_arr.iter().sum();
							let range_len = range_arr.len();
							new_cell.result = Some(format!("{}", result_val / (range_len as f64) ));
						}
						if function == "PROD" {
							let result_val: f64 = range_arr.iter().product();
							new_cell.result = Some(format!("{}", result_val));
						}
						/*if function == "DIV" {
							let result_val: f64 = 0;
							new_cell.result = Some(format!("[ {} ]", result_val));
						}*/
						eval_tmp.push(new_cell);
						//eval_cells.insert((new_cell.row, new_cell.col), new_cell);
						found_formulas = false;
					} else {
						found_formulas = true;
						//println!("Keep calculating... tries: {}", tries_count);
						
					}
				}
			}
		}
		
		for i in 0..eval_tmp.len() {
		
			let mut new_cell = TableCell {
				row: eval_tmp[i].row, col: eval_tmp[i].col, val: eval_tmp[i].val.clone(), 
				result: eval_tmp[i].result.clone(), func: eval_tmp[i].func.clone(), range: eval_tmp[i].range
			};
			eval_cells.insert((eval_tmp[i].row, eval_tmp[i].col), new_cell);
		}
	}
	
}

fn is_cell_in_range(row: i32, col: i32, range: &(i32, i32, i32, i32)) -> bool {
	row >= range.0 && row <= range.2 && col >= range.1 && col <= range.3
}

fn update_table(table: &mut fltk_table_031::SmartTable, eval_cells: &HashMap<(i32, i32), TableCell>) {
	for ((row, col), cell) in eval_cells {
		let val_str = &cell.result.clone().unwrap().clone();
		//let mut val_number: f64 = 0.0;
		match val_str.parse::<f64>() {
			Ok(parsed_val) => {
				//val_number = parsed_val;
				table.set_cell_value(cell.row, cell.col, format!("={:.5}", parsed_val).as_str() );
			},
			Err(_) => {
				table.set_cell_value(cell.row, cell.col, "[ error ]" );
			},
		}
	}
}



