// Copyright 2021 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of various text layout features.
//!
//! I would like to make this a bit fancier (like the flex demo) but for now
//! lets keep it simple.

use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

use druid::piet::dwrite::FontCollection;
use druid::piet::DwriteFactory;
use druid::widget::{
    Button, CrossAxisAlignment, Flex, FlexParams, Label, MainAxisAlignment, SizedBox, Slider,
    Split, TextBox,
};
use druid::{
    AppLauncher, BoxConstraints, Color, Data, Env, FontDescriptor, FontFamily, Lens,
    LocalizedString, Menu, TextAlignment, UnitPoint, Widget, WidgetExt, WindowDesc, WindowId,
};
use log::info;
use regex::Regex;

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Regular Expression Tester");

#[derive(Clone, Data, Debug, Lens)]
struct AppState {
    pattern: String,
    string: String,
    results: String,
}

impl AppState {
    pub fn matches(&mut self) {
        self.results.clear();
        match Regex::new(&self.pattern) {
            Err(e) => self.results.push_str(&format!("Error with pattern: {}", e)),
            Ok(res) => {
                if res.is_match(&self.string) {
                    self.results.push_str(&format!("Matching: \"{}\"\nAgainst: \"{}\"\n\n", &self.pattern, &self.string));
                    let caps = res.captures(&self.string);
                    if let Some(caps) = caps {
                        for (i, c) in caps.iter().enumerate() {
                            if let Some(cc) = c {
                                self.results.push_str(&format!(
                                    "group[{}] = \"{}\"\n",
                                    i,
                                    cc.as_str()
                                ));
                            } else {
                                self.results.push_str(&format!("group[{}] = None\n", i));
                            }
                        }
                    } else {
                        self.results.push_str("There are no group captures");
                    }
                } else {
                    self.results.push_str(&format!(
                        "String:\n\"{}\"\nDoes not match Pattern:\n\"{}\"",
                        &self.string, &self.pattern
                    ));
                }
            }
        }
    }
    pub fn find(&mut self) {
        self.results.clear();
        match Regex::new(&self.pattern) {
            Err(e) => self.results.push_str(&format!("Error with pattern: {}", e)),
            Ok(res) => {
                self.results.push_str(&format!(
                    "Find pattern:\n\"{}\"\nIn:\n\"{}\"\n\n",
                    self.pattern, self.string
                ));
                let mut finds = 0;
                for (i, m) in res.find_iter(&self.string).enumerate() {
                    finds += 1;

                    self.results.push_str(&format!(
                        "Iteration {} found \"{}\" at ({:?})\n",
                        i,
                        m.as_str(),
                        m.range()
                    ));
                }
                if finds <= 0 {
                    self.results.push_str("Found nothing");
                }
            }
        }
    }

    pub fn split(&mut self) {
        self.results.clear();
        match Regex::new(&self.pattern) {
            Err(e) => self.results.push_str(&format!("Error with pattern: {}", e)),
            Ok(res) => {
                self.results.push_str(&format!(
                    "Splitting with pattern:\n\"{}\"\nString:\n\"{}\"\n\n",
                    self.pattern, self.string
                ));
                        let mut finds = 0;
                for (i, m) in res.split(&self.string).enumerate() {
                    finds += 1;

                    self.results
                        .push_str(&format!("Index {} is \"{}\"\n", i, m));
                }
                if finds <= 0 {
                    self.results.push_str("Found nothing");
                }
            }
        }
    }
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        //.menu(make_menu)
        .window_size((800.0, 600.0));

    // create the initial app state
    let initial_state = AppState {
        pattern: "".to_string().into(),
        string: "".to_string().into(),
        results: "".to_string().into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        // .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    /*
    RE      [            ]
    String  [            ]
    [match] [find] [replace]
    [                     ]
    [                     ]
    [                     ]
    */
    //FontFamily::MONOSPACE

    let mono_font = FontDescriptor::new(FontFamily::MONOSPACE).with_size(14.0);

    let re_tb = TextBox::new()
        .with_placeholder("Enter regular expression here")
        .with_font(mono_font.clone())
        .with_text_alignment(TextAlignment::Start)
        .expand_width()
        .lens(AppState::pattern);

    let lb1 = Label::new("Pattern").expand_width();

    let string_tb = TextBox::new()
        .with_placeholder("Enter text to test regular expression against here")
        .with_font(mono_font.clone())
        .with_text_alignment(TextAlignment::Start)
        .expand_width()
        .lens(AppState::string);

    let lb2 = Label::new("String").expand_width();

    let mut mc = Flex::column();

    let mut row1 = Flex::row() //cross_axis_alignment(CrossAxisAlignment::Start)
        .with_default_spacer()
        .with_flex_child(lb1,1.0)
        .with_default_spacer()
        .with_flex_child(re_tb,16.0)
        ;

    let mut row2 = Flex::row() //cross_axis_alignment(CrossAxisAlignment::Start)
        .with_default_spacer()
        .with_flex_child(lb2, 1.0)
        .with_default_spacer()
        .with_flex_child(string_tb, 16.0);

    let mut row3 = Flex::row()
        .with_flex_child(
            Button::new("Matches")
                .on_click(|ctx, data: &mut AppState, e: &Env| {
                    data.matches();
                })
                .expand_width(), //.lens(AppState::re)
            1.0,
        )
        .with_default_spacer()
        .with_flex_child(
            Button::new("Find")
                .on_click(|ctx, data: &mut AppState, e: &Env| {
                    data.find();
                })
                .expand_width(),
            1.0,
        )
        .with_default_spacer()
        .with_flex_child(
            Button::new("Split")
                .on_click(|ctx, data: &mut AppState, e: &Env| {
                    data.split();
                })
                .expand_width(),
            1.0,
        );

    use druid::widget::DisabledIf;

    let results_tb = TextBox::multiline()
        .with_placeholder("Results go here")
        .with_text_alignment(TextAlignment::Start)
        .with_font(mono_font.clone())
        .expand()
        .lens(AppState::results);

    mc.add_flex_child(row1, 1.0);
    mc.add_default_spacer();
    mc.add_flex_child(row2, 1.0);
    mc.add_default_spacer();
    mc.add_flex_child(row3, 1.0);
    mc.add_default_spacer();
    mc.add_flex_child(results_tb.disabled_if(|_, _| false), 10.0);
    mc.add_default_spacer();

    mc //.debug_paint_layout()
}

#[allow(unused_assignments, unused_mut)]
fn make_menu<T: Data>(_window: Option<WindowId>, _data: &AppState, _env: &Env) -> Menu<T> {
    let mut base = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        base = base.entry(druid::platform_menus::mac::application::default())
    }
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        base = base.entry(druid::platform_menus::win::file::default());
    }
    base.entry(
        Menu::new(LocalizedString::new("common-menu-edit-menu"))
            .entry(druid::platform_menus::common::undo())
            .entry(druid::platform_menus::common::redo())
            .separator()
            .entry(druid::platform_menus::common::cut())
            .entry(druid::platform_menus::common::copy())
            .entry(druid::platform_menus::common::paste()),
    )
}
