#![allow(unused_imports)]
use std::path::PathBuf;
use std::ops::Range;
use std::str::Split;
use std::io;
use std::fmt::Write;
use poppler::*;

#[derive(PartialEq)]
pub enum LupaTextColor {
    RGB(u8, u8, u8),
    RED,
    GREEN,
    YELLOW,
    BLACK,
    WHITE,
    BLUE
}

#[derive(PartialEq)]
pub enum LupaTextAlt {
    BOLD,
    COLOR(LupaTextColor),
}

pub struct LupaText {
    pub words: Vec<String>,
    alt: Vec<(Range<usize>, LupaTextAlt)>,
    pub len: usize,
}

impl LupaText {
    pub fn new() -> Self {
        Self {
            words: Vec::new(),
            alt: Vec::new(),
            len: 0
        }
    }

    pub fn get_words(&self) -> Vec<String> {
        self.words.clone()
    }

    pub fn build(&mut self, text: &str, alt: Option<(Range<usize>, LupaTextAlt)>) -> () {
        for word in text.split_whitespace() {
            self.words.push(word.to_owned());
            self.len += 1;
        }

        if alt != None {
            self.alt.push(alt.unwrap());
        } else {
            self.alt.push((Range { start: 0, end: 1 }, LupaTextAlt::BOLD));
        }

        println!("Words {:?}\nLenght {}", self.words, self.len);
    }
}


type BoldWord = (usize, usize, usize);

#[derive(Debug)]
pub enum Errno {
    OPT,
    SOP
}

pub enum FileExt {
    PDF,
    TXT,
    DOCX,
    NULL
}

pub struct Lupa {
    pub bold_word: BoldWord,
    file_path: PathBuf,
    file_name: String,
    file_ext: FileExt,
    pub file_content: String
}

pub struct LupaUi {
    ui: String,
}

impl Lupa {
    pub fn new(path: String, bold: Option<BoldWord>) -> Self {
        let tmp_path = PathBuf::from(path.clone().as_str());

        Self {
            bold_word: if bold == None { (1, 2, 3) } else { bold.unwrap() },
            file_path: tmp_path.clone(),
            file_name: format!("{:?}", tmp_path.file_name().expect("ERROR: is not file")),
            file_ext: match tmp_path.extension().unwrap().to_str() {
                Some("pdf") => FileExt::PDF,
                Some("txt") => FileExt::TXT,
                Some("docx") => FileExt::DOCX,
                None => FileExt::NULL,
                Some(&_) => todo!()
            },
            file_content: String::new(),
        }
    }

    pub fn build(&mut self) -> () {
        match self.file_ext {
            FileExt::PDF => {
                //todo!("PDF PARSER")
                let mut pdf = PopplerDocument::new_from_file(self.file_path.as_path(), "").unwrap();
                self.file_name = pdf.get_title().unwrap_or(self.file_name.clone());

                print!("{}\n", pdf.get_n_pages());

                for idx in 0..pdf.get_n_pages() {
                    let page = pdf.get_page(idx).unwrap();
                    self.file_content += &format!("{}", page.get_text().unwrap()).to_string();
                }
            },
            FileExt::TXT => {
                self.file_content = std::fs::read_to_string(self.file_path.as_path()).unwrap();
            },
            FileExt::DOCX => {
                todo!("DOCX PARSER")
            },
            FileExt::NULL => todo!(),
        }
    }
}

impl LupaUi {
    pub fn new() -> Self {
        Self {
            ui: String::new()
        }
    }

    pub fn get_html(&mut self, lp: Lupa) -> Result<String, ()> {
        let mut filecnt: LupaText = LupaText::new();
        filecnt.build(lp.file_content.trim_matches(|c: char| c.is_alphabetic() || c.is_numeric()), None);

        writeln!(self.ui, "<body>\n\t<p style='text-align:start; padding: 15px'>");
        for wordd in filecnt.words {
            let word = wordd.chars().collect::<Vec<char>>();
            let word_len = word.len();
            let word_bold_start = if word_len <= 2 {1} else if word_len >= 8 {3} else {2};
            let word_bold_end = word_bold_start;
            if word_len <= 1  {
                self.ui += format!("\t\t{} ", word.iter().cloned().collect::<String>()).as_str();
            } else if word_len > 1 {
                self.ui += format!("\t\t<b class='bld'>{}</b>{} ", &word[0..word_bold_start].iter().cloned().collect::<String>(), &word[word_bold_end..word_len].iter().cloned().collect::<String>()).as_str();
            }
        }
        writeln!(self.ui, "\t</p>\n</body>");

        Ok(self.ui.clone())
    }
}
