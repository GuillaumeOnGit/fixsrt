#![allow(dead_code)]
#![allow(unused_imports)]
use std::io;
use std::io::Lines;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::process;
use std::str::FromStr;
use std::env;
use std::io::{Error, ErrorKind};

mod workfile;

#[derive(Default)]
struct Subtitle {
	num: u32,
	duration: String,
	text: [String; 5],
	text_count: u32
}

impl Subtitle {
	fn to_string(&self) -> String {
		let mut ret = self.num.to_string();
		ret.push_str("\r\n");
		ret.push_str(&self.duration);
		ret.push_str("\r\n");
		for text_index in 0..self.text_count as usize {
			ret.push_str(&self.text[text_index]);
			ret.push_str("\r\n");
		}
		ret.push_str("\r\n");
		ret
	}
}

fn parse_lines<L: IntoIterator<Item=io::Result<String>>>(lines: L)
	-> Result<Vec<Subtitle>,String> {
	let mut subtitles: Vec<Subtitle> = Vec::new();

	enum State {
	    WantsNum,
	    WantsDuration,
	    WantsText
	}
	let mut state: State = State::WantsNum;

	let mut subtitle: Subtitle = Default::default();
	for line_result in lines {
		let line = line_result.unwrap();
		match state {
			State::WantsNum => {
				subtitle.num = u32::from_str(&line).unwrap();
				state = State::WantsDuration;
			},
			State::WantsDuration => {
				subtitle.duration = line;
				state = State::WantsText;
			},
			State::WantsText => {
				if line.is_empty() {
					subtitles.push(subtitle);
					subtitle = Default::default();
					state = State::WantsNum;
				}
				else {
					let next_index = subtitle.text_count as usize;
					if next_index == subtitle.text.len() {
						return Err(format!("too much text for {}", subtitle.num))
					}
					else {
						subtitle.text[next_index] = line;
						subtitle.text_count += 1;
					}
				}
			}
		}
	}

	if subtitle.text_count > 0 {
		subtitles.push(subtitle);
	}
	Ok(subtitles)
}

fn is_separator(c: char) -> bool {
	return c == ' ' || c == '\u{A0}' || c == '.' || c == ',';
}

struct Text {
	line: String
}

impl Text {
	// Replaces a word or multiple words.
	// If the word to search ends with a *: a separator or letter can follow
	// If the word to search does not: only a separator can follow
	fn replace(&mut self, what: &str, with: &str) {
		self.line = {
			let mut new_line = String::new();
			let mut line_str = self.line.as_str();
			let (what_no_star, anything_can_follow) = {
				if what.ends_with("*") {
					(&what[..what.len()-1], true)
				}
				else {
					(what, false)
				}
			};

			loop {
				match line_str.find(what_no_star) {
					None => {
						new_line.push_str(line_str);
						break;
					},
					Some(index) => {
						// Found! Verify the next char
						let next_char = line_str[index + what_no_star.len()..].chars().next();
						let do_replace = match next_char {
							Some(c) => anything_can_follow || is_separator(c),
							None => true
						};
						let (left, right) = line_str.split_at(index);
						new_line.push_str(left);
						if do_replace {
							new_line.push_str(with);
						}
						else {
							new_line.push_str(what_no_star);
						}
						line_str = &right[what_no_star.len()..];
						if line_str.is_empty() {
							break;
						}
					}
				}
			}
			new_line
		};
	}
}

///////////////////////////////////////////////////////////////////////////////
fn replace_one(text: &str) -> String {
	let mut t = Text { line: text.to_string() };

	let rules = [
		("  ", " "),

		// Ordinaux
		("2ème",  "2e"),
		("3ème",  "3e"),
		("4ème",  "4e"),
		("5ème",  "5e"),
		("6ème",  "6e"),
		("25ème", "25e"),

		("Ca", "Ça"),

		("Ecartez",    "Écartez"),
		("Echantillon", "Échantillon"),
		("Ecole",      "École"),
		("Economis*",  "Économis*"),
		("Ecout*",     "Écout"),
		("Ecras*",     "Écras"),
		("Edition",    "Édition"),
		("Egoïste",    "Égoïste"),
		("Egypt*",     "Égypt"),
		("Elevé",      "Élevé"),
		("Eloign*",    "Éloign"),
		("Etat",       "État"),
		("Etais",      "Étais"),
		("Etant",      "Étant"),
		("Eteins ça",  "Éteins ça"),
		("Eteins-moi", "Éteins-moi"),
		("Eteins le",  "Éteins le"),
		("Eteins la",  "Éteins la"),
		("Etonnant",   "Étonnant"),
		("Evidemment", "Évidemment"),
		("Evite*",     "Évite"),

		("Etes", "Êtes"),
		("Etre", "Être"),
		
		("A bientôt",   "À bientôt"),
		("A cause",     "À cause"),
		("A ceci",      "À ceci"),
		("A ce",        "À ce"),
		("A chaque",    "À chaque"),
		("A combien",   "À combien"),
		("A commencer", "À commencer"),
		("A condition", "À condition"),
		("A croire",    "À croire"),
		("A demain",    "À demain"),
		("A déplacer",  "À déplacer"),
		("A des",      "À des"),
		("A deux",     "À deux"),
		("A emporter", "À emporter"),
		("A faire",  "À faire"),
		("A fumer",  "À fumer"),
		("A l'*",    "À l'"),
		("A me",     "À me"),
		("A moins",  "À moins"),
		("A mon",    "À mon"),
		("A genoux", "À genoux"),
		("A la",     "À la"),
		("A Los",    "À Los"),
		("A ne",     "À ne"),
		("A notre",   "À notre"),
		("A nous",    "À nous"),
		("A nouveau", "À nouveau"),
		("A part",   "À part"),
		("A peine,", "À peine,"),
		("A peine",  "À peine"),
		("A peu près", "À peu près"),
		("A plus",   "À plus"),
		("A propos", "À propos"),
		("A qu",     "À qu"),
		("A rien",   "À rien"),
		("A son",    "À son"),
		("A table",  "À table"),
		("A terre",  "À terre"),
		("A te",     "À te"),
		("A tes",    "À tes"),
		("A toi",    "À toi"),
		("A ton",    "À ton"),
		("A tou",    "À tou"),
		("A un",     "À un"),
		("A votre",  "À votre"),
		("A vous",   "À vous"),
		("A vos",    "À vos"),
		("A vrai dire", "À vrai dire"),
		("A vendredi",  "À vendredi"),
		
		// Ligature œ	
		("coeur",   "cœur"),
		("coeurs",  "cœurs"),
		("oeuf",    "œuf"),
		("oeufs",   "œufs"),
		("oei*",    "œi"),
		("noeud",   "nœud"),
		("noeuds",  "nœuds"),
		("soeur",   "sœur"),
		("soeurs",  "sœurs"),
		("oeuvre",  "œuvre"),
		("oeuvres", "œuvres"),
		("voeux",   "vœux"),

		// Misc
		("des qu'", "dès qu'"),
		("que tu ais", "que tu aies"),
		("Tous les 2", "Tous les deux"),
		("J'ai du vérifier", "J'ai dû vérifier"),
		("c'est règlé", "c'est réglé"),
		
		// Subjonctif présent 2e personne
		("N'ais", "N'aies"),

		// Participe passé au lieu d'infinitif
		(" se déplacé", " se déplacer"),
		
		// Conjuguaison
		("Je doit", "Je dois")
	];
	for rule_ref in rules.iter() {
		let &(what, with) = rule_ref;
		t.replace(what, with);
	}

	t.line
}

#[test]
fn test_replace_one() {
	assert_eq!("Ça va", replace_one("Ca va"));
	assert_eq!("Ça.", replace_one("Ca."));
	assert_eq!("Ça", replace_one("Ca"));
	assert!("Çaribou" != replace_one("Caribou"));
	assert_eq!("œizz", replace_one("oeizz"));
}

///////////////////////////////////////////////////////////////////////////////
fn do_replacements(subtitles: &mut Vec<Subtitle>) {
	for subtitle in subtitles.iter_mut() {
		subtitle.text[0] = replace_one(&subtitle.text[0]);
		//print!("{}", subtitle.to_string());
	}
}

const BOM: [u8;3] = [0xEF, 0xBB, 0xBF];

///////////////////////////////////////////////////////////////////////////////
fn save_subtitles(subtitles: &Vec<Subtitle>, file_path: &str) -> io::Result<()> {

	let mut work_file = match workfile::WorkFile::create(file_path) {
		Ok(file) => file,
		Err(err) => {
			println!("Cannot create file {}", err);
			return Err(err);
		}
	};
	match work_file.write(&BOM) {
		Ok(len) => if len != BOM.len() {
			println!("Cannot write BOM: not enough space");
			return Err(Error::new(ErrorKind::Other, "bad len BOM"));
		},
		Err(err) => {
			println!("Cannot write BOM: {}", err);
			return Err(err);
		}
	}
	for subtitle in subtitles.iter() {
		let data_str = subtitle.to_string();
		let data = data_str.as_bytes();
		match work_file.write(data) {
			Ok(len) => if len != data.len() {
				println!("Cannot write subtitle: not enough space");
				return Err(Error::new(ErrorKind::Other, "bad len"));
			},
			Err(err) => {
				println!("Cannot write subtitle: {}", err);
				return Err(err);
			}
		}
	}
	work_file.commit();
	Ok(())
}

///////////////////////////////////////////////////////////////////////////////
fn load_subtitles(file_path: &str) -> Result<Vec<Subtitle>,String> {
	let file = match File::open(file_path) {
		Ok(file) => file,
		Err(err) => {
			println!("Cannot open file: {}", err);
			std::process::exit(1);
		}
	};

	let mut buf_reader = BufReader::new(file);

	let remove_bom = {
		let maybe_bom = buf_reader.fill_buf().unwrap();
		if maybe_bom.len() >= 3
		&& maybe_bom[0..3] == BOM {
			true
		}
		else {
			false
		}
	};
	if remove_bom {
		buf_reader.consume(3);
	}
	return parse_lines(buf_reader.lines());
}

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct AppArgs {
	no_backup: bool,
	file_path: String,
	out_file_path: String
}

fn parse_app_args() -> AppArgs {
	let mut args = env::args();
	if args.len() == 1 {
		println!("fixsrt - Hadrien Nilsson - 2016");
		println!("usage: fixsrt [-nobak] SRTFILE [-out OUTFILE]");
		std::process::exit(0);
	}
	args.next().unwrap(); // Skip programe name

	let (no_backup, file_path) = {
		match args.next() {
			Some(arg) => {
				if arg == "-nobak" {
					match args.next() {
						Some(arg2) => (true, arg2),
						None => {
							println!("Missing file");
							std::process::exit(1);
						}
					}
				}
				else {
					(false, arg)
				}
			},
			None => {
				println!("No arg");
				std::process::exit(1);
			}
		}
	};

	let has_out = match args.next() {
		Some(arg) => if arg == "-out" {
			true
		}
		else {
			println!("Unexpected argument: {}", arg);
			std::process::exit(1);
		},
		None => {
			false
		}
	};

	let out_file_path = if has_out {
		match args.next() {
			Some(arg) => arg,
			None => {
				println!("Missing output file");
				std::process::exit(1);
			}
		}
	}
	else {
		file_path.clone() // Same as input file path
	};
	
	AppArgs {
		no_backup: no_backup,
		file_path: file_path,
		out_file_path: out_file_path
	}
}

///////////////////////////////////////////////////////////////////////////////
fn main() {
	let app_args = parse_app_args();

	let subtitles_res = load_subtitles(&app_args.file_path);
	match subtitles_res {
		Ok(_) => (),
		Err(ref err) => {
			println!("{}", err);
			std::process::exit(1);
		}
	}

	let mut subtitles = subtitles_res.unwrap();
	do_replacements(&mut subtitles);

	// Do backup
	if !app_args.no_backup {
		let backup_file_path = format!("{}~", &app_args.file_path);
		match std::fs::copy(&app_args.file_path, &backup_file_path) {
			Ok(_) => (),
			Err(err) => println!("Cannot create backup: {}", err)
		}
	}
	match save_subtitles(&subtitles, &app_args.out_file_path) {
		Ok(_) => (),
		Err(_) => {
			println!("Save failed");
			std::process::exit(1);
		}
	}
	println!("Done - {} subtitles", subtitles.len())
}
