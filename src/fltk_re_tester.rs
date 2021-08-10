#![windows_subsystem = "windows"]

use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    fmt::Display,
    fs::{create_dir_all, DirBuilder, File},
    io::{BufReader, BufWriter},
    ops::Deref,
    rc::Rc,
    time::SystemTime,
};

use anyhow::Context;
use chrono::{DateTime, Local, Utc};
use cpu_time::{ProcessTime, ThreadTime};
use fltk::{app, button::Button, dialog, enums::{Align, Color, Font}, frame::Frame, group::{Pack, PackType}, image::PngImage, input::Input, prelude::{DisplayExt, GroupExt, InputExt, WidgetBase, WidgetExt, WindowExt}, text::{self, TextEditor}, window::Window};
use fltk_theme::{ThemeType, WidgetTheme};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Execution {
    time: chrono::DateTime<Local>,
    operation: String,
    pattern: String,
    string: String,
    count: u32,
}

impl Execution {
    pub fn new(o: &str, p: &str, s: &str) -> Self {
        Execution {
            time: chrono::Local::now(),
            operation: o.into(),
            pattern: p.into(),
            string: s.into(),
            count: 0,
        }
    }
}

impl Display for Execution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} Op: \"{}\" RE: \"{}\" str: \"{}\"",
            self.count,
            self.time.to_rfc3339(),
            self.operation,
            self.pattern,
            self.string
        )
    }
}

#[derive(Debug, Clone)]
struct History {
    hist: Rc<RefCell<Vec<Execution>>>,
}

impl History {
    pub fn new() -> Self {
        History {
            hist: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn load_from() -> Result<History, Box<dyn std::error::Error>> {
        let mut path = dirs::home_dir().context("cannot get home directory to load prior state")?;
        path.push(".re_test");
        create_dir_all(&path)
            .with_context(|| format!("Unable to create directory {}", &path.to_string_lossy()))?;
        path.push("state.json");
        let mut f = File::open(&path)?;
        let mut rb = BufReader::new(&f);
        let res: Vec<Execution> = serde_json::from_reader(rb)?;

        Ok(History {
            hist: Rc::new(RefCell::new(res)),
        })
    }

    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut path = dirs::home_dir().context("cannot get home directory to load prior stuff")?;

        // let v: Value = serde_json::from_str(data)?;
        path.push(".re_test");
        create_dir_all(&path)
            .with_context(|| format!("Unable to create directory {}", &path.to_string_lossy()))?;
        path.push("state.json");
        let mut f = File::create(&path)?; // .with_context(||"cannot open state file: {}", &path)?;
        let bw = BufWriter::new(&f);
        let h = self.hist.deref().borrow();
        serde_json::to_writer_pretty(bw, &*h);
        Ok(())
    }

    pub fn add(&mut self, mut exe: Execution) {
        let mut found = usize::max_value();
        for (i, e) in self.hist.deref().borrow().iter().enumerate().rev() {
            if e.pattern == exe.pattern && e.string == exe.string {
                found = i;
                break;
            }
        }
        exe.count = if found != usize::max_value() {
            (self.hist.deref().borrow_mut().remove(found).count + 1u32)
        } else {
            1u32
        };
        let mut h = self.hist.deref().borrow_mut().push(exe);
        match self.save() {
            Err(e) => dialog::alert(200, 200, &format!("Unable to save result: {}", e)),
            _ => {}
        }
    }

    pub fn to_str(&self) -> String {
        let mut s = String::with_capacity(1024);
        let h = self.hist.deref().borrow();
        if h.len() <= 0 {
            s.push_str("No history as yet");
        } else {
            for (i, o) in h.iter().enumerate().rev() {
                s.push_str(&format!("{}: {}\n", i, o));
            }
        }
        s
    }

    pub fn last(&self) -> Option<Execution> {
        let h = self.hist.deref().borrow();
        if h.len() <= 0 {
            None
        } else {
            Some(h.last().unwrap().clone())
        }
    }
}

#[derive(Debug, Clone)]
struct ReTest {
    out: TextEditor,
    buff: text::TextBuffer,
    inp: Input,
    pat: Input,
    cpu_frame: Frame,
    cpu_time: Rc<ProcessTime>,
    hist: History,
}

impl ReTest {
    pub fn new(
        out: &TextEditor,
        buff: &text::TextBuffer,
        inp: &Input,
        pat: &Input,
        cpu_frame: &Frame,
        cpu_time: &ProcessTime,
        hist: History,
    ) -> Self {
        let r = ReTest {
            out: out.clone(),
            buff: buff.clone(),
            inp: inp.clone(),
            pat: pat.clone(),
            cpu_frame: cpu_frame.clone(),
            cpu_time: Rc::new(*cpu_time),
            hist,
        };
        r
    }

    pub fn update_cpu(&mut self) {
        self.cpu_frame.set_label(&format!("{:?}", &self.cpu_time.elapsed()));
        self.cpu_frame.redraw();
    }

    pub fn history(&mut self) {
        self.buff.set_text(&self.hist.to_str());
        self.update_cpu();
    }

    pub fn matches(&mut self) {
        self.buff.set_text("");
        let mut results = String::with_capacity(128);
        self.out.set_text_color(Color::Black);
        match Regex::new(&self.pat.value()) {
            Err(e) => {
                self.out.set_text_color(Color::Red);
                results.push_str(&format!("Error with pattern: {}", e))
            }
            Ok(res) => {
                if res.is_match(&self.inp.value()) {
                    results.push_str(&format!(
                        "Matching: \"{}\"\nAgainst: \"{}\"\n\n",
                        &self.pat.value(),
                        &self.inp.value()
                    ));
                    let s = self.inp.value().clone();
                    let caps = res.captures(&s);
                    if let Some(caps) = caps {
                        for (i, c) in caps.iter().enumerate() {
                            if let Some(cc) = c {
                                results.push_str(&format!("group[{}] = \"{}\"\n", i, cc.as_str()));
                            } else {
                                results.push_str(&format!("group[{}] = None\n", i));
                            }
                        }
                    } else {
                        results.push_str("There are no group captures");
                    }
                } else {
                    self.out.set_text_color(Color::Red);

                    results.push_str(&format!(
                        "String:\n\"{}\"\nDoes not match Pattern:\n\"{}\"",
                        &self.inp.value(),
                        &self.pat.value()
                    ));
                }
            }
        }
        self.buff.set_text(&results);
        self.hist.add(Execution::new(
            "match",
            &self.pat.value(),
            &self.inp.value(),
        ));
        self.update_cpu();
    }

    pub fn find(&mut self) {
        self.out.set_text_color(Color::Black);
        self.buff.set_text("");

        let mut results = String::with_capacity(128);
        let pattern = self.pat.value().clone();
        let string = self.inp.value().clone();

        match Regex::new(&pattern) {
            Err(e) => {
                self.out.set_text_color(Color::Red);
                results.push_str(&format!("Error with pattern: {}", e))
            }
            Ok(res) => {
                results.push_str(&format!(
                    "Find pattern:\n\"{}\"\nIn:\n\"{}\"\n\n",
                    pattern, string
                ));
                let mut finds = 0;
                for (i, m) in res.find_iter(&string).enumerate() {
                    finds += 1;

                    results.push_str(&format!(
                        "Iteration {} found \"{}\" at ({:?})\n",
                        i,
                        m.as_str(),
                        m.range()
                    ));
                }
                if finds <= 0 {
                    self.out.set_text_color(Color::Red);
                    results.push_str("Found nothing");
                }
            }
        }
        self.buff.set_text(&results);
        self.hist
            .add(Execution::new("find", &self.pat.value(), &self.inp.value()));
        self.update_cpu();
    }

    pub fn split(&mut self) {
        self.buff.set_text("");
        self.out.set_text_color(Color::Black);

        let mut results = String::with_capacity(128);
        let pattern = self.pat.value().clone();
        let string = self.inp.value().clone();

        match Regex::new(&pattern) {
            Err(e) => {
                self.out.set_text_color(Color::Red);
                results.push_str(&format!("Error with pattern: {}", e))
            }
            Ok(res) => {
                results.push_str(&format!(
                    "Splitting with pattern:\n\"{}\"\nString:\n\"{}\"\n\n",
                    pattern, string
                ));
                let mut finds = 0;
                for (i, m) in res.split(&string).enumerate() {
                    finds += 1;
                    results.push_str(&format!("Index {} is \"{}\"\n", i, m));
                }
                if finds <= 0 {
                    self.out.set_text_color(Color::Red);
                    results.push_str("Found nothing");
                }
            }
        }
        self.buff.set_text(&results);
        self.hist.add(Execution::new(
            "split",
            &self.pat.value(),
            &self.inp.value(),
        ));
        self.update_cpu();
    }
}

fn main() {
    let start_cpu = cpu_time::ProcessTime::now();
    let app = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::Metro);
    widget_theme.apply();

    let font = Font::by_name("Courier");

    let mut wind = Window::default()
        .with_size(600, 400)
        .center_screen()
        .with_label("Regular Expression Tester");

    let icon_bytes = std::include_bytes!("../asset/icon3.png");
    let im = match PngImage::from_data(icon_bytes) {
        Err(e) => None,
        Ok(i) => Some(i),
    };
    wind.set_icon(im);
    wind.size_range(600, 400, 0, 0);

    let mut main_group = Pack::new(0, 0, 600, 400, "");

    let f1 = Frame::default().with_size(0, 5);

    let mut pattern_pack = Pack::default().with_size(600, 25);

    let mut pat_lab = Frame::default()
        .with_size(60, 25)
        .with_label("Pattern: ")
        .with_align(Align::Inside | Align::Left);
    //pat_lab.set_label_type(fltk::enums::LabelType::Engraved);
    let mut pat = Input::new(0, 0, 500, 25, "").left_of(&pat_lab, 4);
    pat.set_text_font(font.clone());

    pattern_pack.resizable(&pat);
    pattern_pack.end();
    pattern_pack.set_type(PackType::Horizontal);

    let f1 = Frame::default().with_size(0, 5);

    let mut string_pack = Pack::default().with_size(600, 25).with_pos(0, 25);

    let mut str_lab = Frame::default()
        .with_size(60, 25)
        .with_label("String: ")
        .with_align(Align::Inside | Align::Left);
    let mut str = Input::new(0, 0, 500, 25, "").left_of(&str_lab, 4);
    str.set_text_font(font.clone());

    string_pack.resizable(&str);
    string_pack.end();
    string_pack.set_type(PackType::Horizontal);

    let f1 = Frame::default().with_size(0, 5);

    let mut button_pack = Pack::default()
        .with_size(600, 25)
        .with_pos(0, 25)
        .with_align(Align::Center);

    let f1 = Frame::default().with_size(5, 0);

    let mut matches_but = Button::default()
        .with_size(60, 25)
        .with_pos(25, 25)
        .with_label("&Matches");
    // matches_but.set_shortcut(Shortcut::Enter | 'm');
    matches_but.set_visible_focus();

    let mut find_but = Button::default().with_size(60, 25).with_label("&Find");
    let mut split_but = Button::default().with_size(60, 25).with_label("&Split");
    let mut hist_but = Button::default().with_size(60, 25).with_label("&History");
    let mut cpu_frame = Frame::default().with_size(90,25).with_label("cpu time");

    button_pack.end();
    button_pack.set_type(PackType::Horizontal);
    button_pack.set_spacing(20);
    button_pack.set_align(Align::Center);

    let f1 = Frame::default().with_size(0, 5);

    let mut buff = text::TextBuffer::default();
    buff.set_tab_distance(4);

    let mut op = TextEditor::default().with_size(600, 300);
    op.set_buffer(buff.clone());
    op.set_scrollbar_size(16);
    op.set_text_font(font);
    op.set_align(Align::Inside | Align::Left | Align::Top);

    main_group.end();
    main_group.set_type(PackType::Vertical);
    main_group.resizable(&op);

    // pack1.make_resizable(true);

    wind.make_resizable(true);
    wind.end();
    wind.show();

    let hist = match History::load_from() {
        Err(e) => {
            dialog::alert(
                200,
                200,
                &format!("Could not load prior state/history: \n\t{}", e),
            );
            History::new()
        }
        Ok(h) => h,
    };

    if let Some(last) = hist.last() {
        str.set_value(&last.string);
        pat.set_value(&last.pattern);
    }

    let mut r_ = ReTest::new(&op, &buff, &str, &pat, &cpu_frame, &start_cpu, hist);

    let mut r = r_.clone();
    matches_but.set_callback(move |b| r.matches());
    let mut r = r_.clone();
    find_but.set_callback(move |b| r.find());
    let mut r = r_.clone();
    split_but.set_callback(move |b| r.split());
    let mut r = r_.clone();
    hist_but.set_callback(move |b| r.history());


    wind.handle(move|x,y| {
        r_.update_cpu();
        false
    });

    app.run().unwrap();
}
