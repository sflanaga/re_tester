/*!
    A very simple application that show how to use a flexbox layout.

    Requires the following features: `cargo run --example flexbox --features "flexbox"`
*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;
//use regex::Regex;
#[derive(Default)]
pub struct ReTesterApp {
    font_lbl: nwg::Font,
    font_io: nwg::Font,
    window: nwg::Window,
    col_layout: nwg::FlexboxLayout,
    layout1: nwg::FlexboxLayout,
    layout2: nwg::FlexboxLayout,
    pattern_lb: nwg::Label,
    pattern_inp: nwg::TextInput,
    string_lb: nwg::Label,
    string_inp: nwg::TextInput,
    output_tb: nwg::TextBox,

    // button2: nwg::Button,
    // button3: nwg::Button
}

impl ReTesterApp {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod retester_app_ui {
    use super::*;
    use native_windows_gui as nwg;
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
                .size((500, 200))
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
                        }
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
            const HEIGHT_INP: D = D::Points(25.);
            // const PT_10: D = D::Points(10.0);
            const PT_10: D = D::Points(5.0);
            const PT_5: D = D::Points(3.0);
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
                .padding(MIN_PAD)
                .min_size(Size{width: D::Points(300.), height: D::Points(200.)})
                .flex_direction(FlexDirection::Column)
                .child_layout(&ui.layout1)
                .child_flex_grow(0.)
                .child_layout(&ui.layout2)
                .child_flex_grow(0.)
                .child(&ui.output_tb)
                .child_flex_grow(1.)
                .build(&ui.col_layout)?;

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
