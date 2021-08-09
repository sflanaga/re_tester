use fltk::{
    app,
    button::Button,
    enums::{Align, Color, Event, Font, Shortcut},
    frame::Frame,
    group::{Group, Pack, PackType},
    input::Input,
    output::{MultilineOutput, Output},
    prelude::{ButtonExt, GroupExt, InputExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
};
use fltk_theme::{color_themes, ColorTheme, ThemeType, WidgetTheme};
use regex::Regex;

pub fn matches(pat_in: &Input, str_in: &Input, out: &mut MultilineOutput) {
    out.set_value("");
    let mut results = String::with_capacity(128);
    out.set_text_color(Color::Black);
    match Regex::new(&pat_in.value()) {
        Err(e) => {
            out.set_text_color(Color::Red);
            results.push_str(&format!("Error with pattern: {}", e))
        }
        Ok(res) => {
            if res.is_match(&str_in.value()) {
                results.push_str(&format!(
                    "Matching: \"{}\"\nAgainst: \"{}\"\n\n",
                    &pat_in.value(),
                    &str_in.value()
                ));
                let s = str_in.value().clone();
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
                out.set_text_color(Color::Red);

                results.push_str(&format!(
                    "String:\n\"{}\"\nDoes not match Pattern:\n\"{}\"",
                    &str_in.value(),
                    &pat_in.value()
                ));
            }
        }
    }
    out.set_value(&results);
}

pub fn find(pat_in: &Input, str_in: &Input, out: &mut MultilineOutput) {
    out.set_value("");
    out.set_text_color(Color::Black);

    let mut results = String::with_capacity(128);
    let pattern = pat_in.value().clone();
    let string = str_in.value().clone();
    match Regex::new(&pattern) {
        Err(e) => {
            out.set_text_color(Color::Red);
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
                out.set_text_color(Color::Red);
                results.push_str("Found nothing");
            }
        }
    }
    out.set_value(&results);
}

pub fn split(pat_in: &Input, str_in: &Input, out: &mut MultilineOutput) {
    out.set_value("");
    out.set_text_color(Color::Black);
    let mut results = String::with_capacity(128);
    let pattern = pat_in.value().clone();
    let string = str_in.value().clone();
    match Regex::new(&pattern) {
        Err(e) => {
            out.set_text_color(Color::Red);
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
                out.set_text_color(Color::Red);
                results.push_str("Found nothing");
            }
        }
    }
    out.set_value(&results);
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

    let (patc, strc, mut opc) = (pat.clone(), str.clone(), op.clone());
    matches_but.set_callback(move |b| matches(&patc, &strc, &mut opc));
    let (patc, strc, mut opc) = (pat.clone(), str.clone(), op.clone());
    find_but.set_callback(move |b| find(&patc, &strc, &mut opc));
    let (patc, strc, mut opc) = (pat.clone(), str.clone(), op.clone());
    split_but.set_callback(move |b| split(&patc, &strc, &mut opc));

    app.run().unwrap();
    /* Event handling */
}
