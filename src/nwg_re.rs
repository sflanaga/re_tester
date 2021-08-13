/*!
    A very simple application that show how to use a flexbox layout.

    Requires the following features: `cargo run --example flexbox --features "flexbox"`
*/

extern crate native_windows_gui as nwg;
use std::borrow::{Borrow, BorrowMut};

use nwg::{NativeUi, simple_message};
use serde::{Deserialize, Serialize};
use regex::Regex;

mod hist;
use hist::{Execution, History};

#[derive(Default)]
pub struct ReTesterApp {
    font_lbl: nwg::Font,
    font_io: nwg::Font,
    window: nwg::Window,
    
    col_layout: nwg::FlexboxLayout,
    layout1: nwg::FlexboxLayout,
    layout2: nwg::FlexboxLayout,
    layout3: nwg::FlexboxLayout,

    pattern_lb: nwg::Label,
    pattern_inp: nwg::TextInput,
    string_lb: nwg::Label,
    string_inp: nwg::TextInput,

    match_bt: nwg::Button,
    find_bt: nwg::Button,
    split_bt: nwg::Button,
    history_bt: nwg::Button,

    output_tb: nwg::TextBox,

    hist: History,
    // button2: nwg::Button,
    // button3: nwg::Button
}

impl ReTesterApp {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
    fn _match(&self) {
        self.output_tb.set_text("");
        let mut results = String::with_capacity(128);
        match Regex::new(&self.pattern_inp.text()) {
            Err(e) => {
                results.push_str(&format!("Error with pattern: {}", e))
            }
            Ok(res) => {
                if res.is_match(&self.string_inp.text()) {
                    results.push_str(&format!(
                        "Matching: \"{}\"\r\nAgainst: \"{}\"\r\n",
                        &self.pattern_inp.text(),
                        &self.string_inp.text()
                    ));
                    let s = self.string_inp.text().clone();
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
                    results.push_str(&format!(
                        "String:\r\n\t\"{}\"\r\nDoes not match Pattern:\r\n\t\"{}\"",
                        &self.string_inp.text(),
                        &self.pattern_inp.text()
                    ));
                }
            }
        }
        self.output_tb.set_text(&results);
        
        self.hist.add(Execution::new(
            "match",
            &self.pattern_inp.text(),
            &self.string_inp.text(),
        ));
    }

    fn find(&self) {
        self.output_tb.set_text("");

        let mut results = String::with_capacity(128);
        let pattern = self.pattern_inp.text().clone();
        let string = self.string_inp.text().clone();

        match Regex::new(&pattern) {
            Err(e) => {
                // self.out.set_text_color(Color::Red);
                results.push_str(&format!("Error with pattern: {}", e))
            }
            Ok(res) => {
                results.push_str(&format!(
                    "Find pattern:\r\n\"{}\"\r\nIn:\r\n\"{}\"\r\n",
                    pattern, string
                ));
                let mut finds = 0;
                for (i, m) in res.find_iter(&string).enumerate() {
                    finds += 1;

                    results.push_str(&format!(
                        "Iteration {} found \"{}\" at ({:?})\r\n",
                        i,
                        m.as_str(),
                        m.range()
                    ));
                }
                if finds <= 0 {
                    // self.out.set_text_color(Color::Red);
                    results.push_str("Found nothing");
                }
            }
        }
        self.output_tb.set_text(&results);
        self.hist
            .add(Execution::new("find", &self.pattern_inp.text(), &self.string_inp.text()));
    }

    fn split(&self) {
        self.output_tb.set_text("");

        let mut results = String::with_capacity(128);
        let pattern = self.pattern_inp.text().clone();
        let string = self.string_inp.text().clone();

        match Regex::new(&pattern) {
            Err(e) => {
                results.push_str(&format!("Error with pattern: {}", e))
            }
            Ok(res) => {
                results.push_str(&format!(
                    "Splitting with pattern:\r\n\"{}\"\r\nString:\r\n\"{}\"\r\n\n",
                    pattern, string
                ));
                let mut finds = 0;
                for (i, m) in res.split(&string).enumerate() {
                    finds += 1;
                    results.push_str(&format!("Index {} is \"{}\"\r\n", i, m));
                }
                if finds <= 0 {
                    results.push_str("Found nothing");
                }
            }
        }
        self.output_tb.set_text(&results);
        self.hist.add(Execution::new(
            "split",
            &self.pattern_inp.text(),
            &self.string_inp.text(),
        ));
    }

    fn history(&self) {
        self.output_tb.set_text(&self.hist.to_str());
    }

    fn load_history(&self) {
        match self.hist.load_from() {
            Err(e) => {simple_message("Error loading history", &format!("error loading history: {}", e));},
            _ => {},
        }
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod retester_app_ui {
    use super::*;
    use native_windows_gui as nwg;
    use nwg::stretch::style::AlignContent;
    use nwg::{Font, TextBoxFlags, VTextAlign};
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    pub struct FlexBoxAppUi {
        inner: Rc<ReTesterApp>,
        default_handler: RefCell<Option<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<FlexBoxAppUi> for ReTesterApp {
        fn build_ui(mut data: ReTesterApp) -> Result<FlexBoxAppUi, nwg::NwgError> {
            use nwg::Event as E;

            Font::builder().family("Courier New").size_absolute(14).build(&mut data.font_io);
            // Controls
            nwg::Window::builder()
                .size((500, 300))
                .position((300, 300))
                .title("Regular Expression Tester")
                .build(&mut data.window)?;

            nwg::Label::builder()
                .text("Pattern")
                .parent(&data.window)
                // .focus(fa)
                .build(&mut data.pattern_lb)?;

            nwg::TextInput::builder()
                .text("RE goes here")
                .parent(&data.window)
                .font(Some(&data.font_io))
                .focus(true)
                .build(&mut data.pattern_inp)?;

            nwg::Label::builder()
                .text("String")
                .parent(&data.window)
                // .focus(fa)
                .build(&mut data.string_lb)?;

            nwg::TextInput::builder()
                .text("Test String goes here")
                .parent(&data.window)
                .font(Some(&data.font_io))
                .focus(true)
                .build(&mut data.string_inp)?;

            nwg::Button::builder()
                .text("&Match")
                .parent(&data.window)
                .build(&mut data.match_bt)?;

            nwg::Button::builder()
                .text("&Find")
                .parent(&data.window)
                .build(&mut data.find_bt)?;

            nwg::Button::builder()
                .text("&Split")
                .parent(&data.window)
                .build(&mut data.split_bt)?;

            nwg::Button::builder()
                .text("&History")
                .parent(&data.window)
                .build(&mut data.history_bt)?;

                // use nwg::TextBoxFlags as TB;
            nwg::TextBox::builder()
                .text("Test String goes here")
                //.flags()
                .parent(&data.window)
                .focus(true)
                .font(Some(&data.font_io))
                // .flags(TB::VISIBLE|TB::AUTOHSCROLL|TB::AUTOVSCROLL|TB::VSCROLL)
                .readonly(true)
                .build(&mut data.output_tb)?;

            let ui = FlexBoxAppUi { inner: Rc::new(data), default_handler: Default::default() };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(evt_ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnWindowClose => {
                            if &handle == &evt_ui.window {
                                ReTesterApp::exit(&evt_ui);
                            }
                        },
                        E::OnButtonClick if &handle == &evt_ui.match_bt => ReTesterApp::_match(&evt_ui),
                        E::OnButtonClick if &handle == &evt_ui.find_bt => ReTesterApp::find(&evt_ui),
                        E::OnButtonClick if &handle == &evt_ui.split_bt => ReTesterApp::split(&evt_ui),
                        E::OnButtonClick if &handle == &evt_ui.history_bt => ReTesterApp::history(&evt_ui),
                        _ => {}
                    }
                }
            };

            *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
                &ui.window.handle,
                handle_events,
            ));

            // Layout
            use nwg::stretch::{
                geometry::{Rect, Size},
                style::{AlignSelf, Dimension as D, FlexDirection},
            };
            const LBL_WIDTH: D = D::Points(55.);
            const BUTTON_WIDTH: D = D::Points(65.);
            const HEIGHT_INP: D = D::Points(25.);
            const HEIGHT_BT: D = D::Points(25.);
            const PT_10: D = D::Points(5.0);
            const MIN_PAD: Rect<D> = Rect { start: PT_10, end: PT_10, top: PT_10, bottom: PT_10 };

            nwg::FlexboxLayout::builder()
                .parent(&ui.window)
                .flex_direction(FlexDirection::Row)
                .padding(MIN_PAD)
                .child(&ui.pattern_lb)
                .child_size(Size { width: LBL_WIDTH, height: HEIGHT_INP })
                .child(&ui.pattern_inp)
                .child_size(Size { width: D::Auto, height: HEIGHT_INP })
                .child_flex_grow(1.0)
                .build_partial(&ui.layout1)?;

            nwg::FlexboxLayout::builder()
                .parent(&ui.window)
                .flex_direction(FlexDirection::Row)
                .padding(MIN_PAD)
                .child(&ui.string_lb)
                .child_size(Size { width: LBL_WIDTH, height: HEIGHT_INP })
                .child(&ui.string_inp)
                .child_size(Size { width: D::Auto, height: HEIGHT_INP })
                .child_flex_grow(1.0)
                .build_partial(&ui.layout2)?;

            nwg::FlexboxLayout::builder()
                .parent(&ui.window)
                .flex_direction(FlexDirection::Row)
                .auto_spacing(None)
                .padding(MIN_PAD)
                .child(&ui.match_bt)
                .child_size(Size { width: BUTTON_WIDTH, height: HEIGHT_BT })
                .child(&ui.find_bt)
                .child_size(Size { width: BUTTON_WIDTH, height: HEIGHT_BT })
                .child(&ui.split_bt)
                .child_size(Size { width: BUTTON_WIDTH, height: HEIGHT_BT })
                .child(&ui.history_bt)
                .child_size(Size { width: BUTTON_WIDTH, height: HEIGHT_BT })
                // .child_flex_grow(1.0)
                .build_partial(&ui.layout3)?;

            nwg::FlexboxLayout::builder()
                .parent(&ui.window)
                .min_size(Size{width: D::Points(300.), height: D::Points(200.)})
                .flex_direction(FlexDirection::Column)
                .child_layout(&ui.layout1)
                .child_flex_grow(0.)
                .child_layout(&ui.layout2)
                .child_flex_grow(0.)
                .child_layout(&ui.layout3)
                .child_flex_grow(0.)
                .child(&ui.output_tb)
                .child_flex_grow(1.)
                .child_margin(MIN_PAD)
                .build(&ui.col_layout)?;

            ui.load_history();

            return Ok(ui);
        }
    }

    impl Drop for FlexBoxAppUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for FlexBoxAppUi {
        type Target = ReTesterApp;

        fn deref(&self) -> &ReTesterApp {
            &self.inner
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    

    let _ui = ReTesterApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
