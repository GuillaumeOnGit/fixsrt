extern crate clap;

use clap::{Arg, App, Error};

mod workfile;
mod srt;
mod txt_rep;
mod rules;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

///////////////////////////////////////////////////////////////////////////////
fn do_replacements(subtitles: &mut Vec<srt::Subtitle>, language: &str) {

	for subtitle in subtitles.iter_mut() {
		for text_index in 0..subtitle.text_count as usize {
			subtitle.texts[text_index] = txt_rep::replace_one(&subtitle.texts[text_index], &language);
		}
		//print!("{}", subtitle.to_string());
	}
}

fn do_time_changes(subtitles: &mut Vec<srt::Subtitle>, language: &str,
                   time_shift_ms: i32, time_stretch_ms: i32) {

	if time_shift_ms == 0 && time_stretch_ms == 0 {
		return;
	}
	for subtitle in subtitles.iter_mut() {
		for text_index in 0..subtitle.text_count as usize {
			subtitle.texts[text_index] = txt_rep::replace_one(&subtitle.texts[text_index], &language);
		}
		//print!("{}", subtitle.to_string());
	}
}


///////////////////////////////////////////////////////////////////////////////
fn main() {

	let matches = App::new("fixsrt")
		.version(VERSION)
		.author("Hadrien Nilsson")
		.about("Fix spelling and encoding mistakes in SRT subtitle files")
		.arg(Arg::with_name("nobak")
			.long("nobak")
			.help("Avoids creating a backup file"))
		.arg(Arg::with_name("SRTFILE")
			.required(true)
			.multiple(true)
			.help("SRT file to update"))
		.arg(Arg::with_name("out")
			.short("o")
			.long("out")
			.takes_value(true)
			.help("Outputs to another file (single input only)"))
		.arg(Arg::with_name("time-shift")
			.long("time-shift")
			.takes_value(true)
			.help("Shifts all subtitles of several seconds"))
		.arg(Arg::with_name("time-stretch")
			.long("time-stretch")
			.takes_value(true)
			.help("Stretches all subtitles after adding several seconds to the last one"))
		.arg(Arg::with_name("lang")
			.long("lang")
			.takes_value(true)
			.help("Selects the language of the SRT file(s) (fr/en)"))
		.get_matches();

	let no_backup = matches.is_present("nobak");
	let in_file_paths: Vec<_> = matches.values_of("SRTFILE").unwrap().collect();
	let out_file_path = matches.value_of("out");
	let time_shift_ms = match matches.value_of("time-shift") {
		Some(tos) => match srt::parse_srt_time(tos) {
			Some(tos_ms) => tos_ms,
			None => {
				let err = Error { message: "--time-shift invalid argument".into(),
					kind: clap::ErrorKind::TooManyValues,
					info: None };
				err.exit();
			}
		},
		None => 0
	};
	let time_stretch_ms = match matches.value_of("time-stretch") {
		Some(tos) => match srt::parse_srt_time(tos) {
			Some(tos_ms) => tos_ms,
			None => {
				let err = Error { message: "--time-stretch invalid argument".into(),
					kind: clap::ErrorKind::TooManyValues,
					info: None };
				err.exit();
			}
		},
		None => 0
	};
	let language = matches.value_of("lang").unwrap_or("fr");

	if language != "en" && language != "fr" {
		println!("Bad language");
		std::process::exit(1);
	}

	// Additional check. Is there a way to do it with clap?
	if out_file_path.is_some() && in_file_paths.len() > 1 {
		let err = Error { message: "--out works only with single input".into(),
			kind: clap::ErrorKind::TooManyValues,
			info: None};
		err.exit();
	}

	/////////////////////////////////////////////////////////////////
	for in_file_path in in_file_paths {
		print!("{} ... ", in_file_path);

		let subtitles_res = srt::load_subtitles(&in_file_path);
		match subtitles_res {
			Ok(_) => (),
			Err(ref err) => {
				println!("{}", err);
				std::process::exit(1);
			}
		}

		let mut subtitles = subtitles_res.unwrap();
		do_replacements(&mut subtitles, &language);
		do_time_changes(&mut subtitles, &language, time_shift_ms, time_stretch_ms);

		// Do backup
		if !no_backup {
			let backup_file_path = format!("{}~", in_file_path);
			match std::fs::copy(&in_file_path, &backup_file_path) {
				Ok(_) => (),
				Err(err) => println!("Cannot create backup: {}", err)
			}
		}

		let final_out_file_path = if out_file_path.is_none() {
			in_file_path
		}
		else {
			out_file_path.unwrap()
		};

		match srt::save_subtitles(&subtitles, &final_out_file_path) {
			Ok(_) => (),
			Err(_) => {
				println!("Save failed");
				std::process::exit(1);
			}
		}
		println!("done: {} subtitles", subtitles.len());
	}
}
