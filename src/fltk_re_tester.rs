#![windows_subsystem = "windows"]

use std::{cell::RefCell, fmt::Display, rc::Rc, time::SystemTime};

use chrono::{DateTime, Local, Utc};
use fltk::{
    app,
    button::Button,
    enums::{Align, Color, Font, },
    frame::Frame,
    group::{Pack, PackType},
    input::Input,
    output::{MultilineOutput},
    prelude::{GroupExt, InputExt, WidgetExt, WindowExt},
    window::Window,
};
use fltk_theme::{ThemeType, WidgetTheme};
use regex::Regex;


#[derive(Debug, Clone)]
struct Execution {
    time: chrono::DateTime<Local>,
    operation: String,
    pattern: String,
    string: String,
}

impl Execution {
    pub fn new(o: &str, p: &str, s: &str) -> Self {
        Execution {
            time: chrono::Local::now(),
            operation: o.into(),
            pattern: p.into(),
            string: s.into(),
        }
    }
}

impl Display for Execution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Op: \"{}\" RE: \"{}\" str: \"{}\"", self.time.to_rfc3339(), self.operation, self.pattern, self.string)
    }
}

#[derive(Debug, Clone)]
struct ReTest {
    out: MultilineOutput,
    inp: Input,
    pat: Input,
    hist: Rc<RefCell<Vec<Execution>>>,
}

impl ReTest {
    pub fn new(out: &MultilineOutput, inp: &Input, pat: &Input) -> Self {
        let r = ReTest {
            out: out.clone(),
            inp: inp.clone(),
            pat: pat.clone(),
            hist: Rc::new(RefCell::new(Vec::new())),
        };
        r
    }
    pub fn history(&mut self) {
        let mut s = String::with_capacity(1024);
        if self.hist.borrow().len() <= 0 {
            s.push_str("No history as yet");
        } else {
            for (i,x) in self.hist.borrow().iter().enumerate().rev() {
                s.push_str(&format!("{}: {}\n", i, x));
            }
        }
        self.out.set_value(&s);
    }

    pub fn matches(&mut self) {
        self.out.set_value("");
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
        self.out.set_value(&results);
        self.hist.borrow_mut().push(Execution::new("matches", &self.pat.value(), &self.inp.value(), ))
    }
    
    pub fn find(&mut self) {
        self.out.set_value("");
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
        self.out.set_value(&results);
        self.hist.borrow_mut().push(Execution::new("find", &self.pat.value(), &self.inp.value(), ))
    }
    
    pub fn split(&mut self) {
        self.out.set_value("");
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
        self.out.set_value(&results);
        self.hist.borrow_mut().push(Execution::new("split", &self.pat.value(), &self.inp.value(), ))
    }
}


fn main() {
    let app = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::Metro);
    widget_theme.apply();

    let font = Font::by_name("Courier");

    let mut wind = Window::default()
        .with_size(600, 400)
        .center_screen()
        .with_label("Counter");
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

    button_pack.end();
    button_pack.set_type(PackType::Horizontal);
    button_pack.set_spacing(20);
    button_pack.set_align(Align::Center);

    let f1 = Frame::default().with_size(0, 5);

    let mut op = MultilineOutput::default().with_size(600, 300);
    op.set_text_font(font);
    op.set_align(Align::Inside | Align::Left | Align::Top);

    main_group.end();
    main_group.set_type(PackType::Vertical);
    main_group.resizable(&op);

    // pack1.make_resizable(true);

    wind.make_resizable(true);
    wind.end();
    wind.show();

    let r_ =  ReTest::new(&op, &str,&pat);

    let mut r =  r_.clone();
    matches_but.set_callback(move |b| r.matches());
    let mut r= r_.clone();
    find_but.set_callback(move |b| r.find());
    let mut r = r_.clone();
    split_but.set_callback(move |b| r.split());
    let mut r = r_.clone();
    hist_but.set_callback(move |b| r.history());


    app.run().unwrap();
}
