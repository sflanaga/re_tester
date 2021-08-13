use std::{cell::RefCell, fmt::Display, fs::{File, create_dir_all}, io::{BufReader, BufWriter}, rc::Rc,ops::Deref};

use anyhow::Context;
use nwg::{MessageWindow, simple_message};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local, Utc};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
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
pub struct History {
    hist: Rc<RefCell<Vec<Execution>>>,
}

impl Default for History {
    fn default() -> Self {
        History::new()
    }
}


impl History {
    pub fn new() -> Self {
        println!("creating history");
        History {
            hist: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn load_from(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut path = dirs::home_dir().context("cannot get home directory to load prior state")?;
        path.push(".re_test");
        create_dir_all(&path)
            .with_context(|| format!("Unable to create directory {}", &path.to_string_lossy()))?;
        path.push("state.json");
        let mut f = File::open(&path)?;
        let mut rb = BufReader::new(&f);
        let res: Vec<Execution> = serde_json::from_reader(rb)?;

        for x in res.into_iter() {
            self.hist.borrow_mut().push(x);
        }
        println!("loading history");
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
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

    pub fn add(&self, mut exe: Execution) {
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
            Err(e) => { simple_message("Error", &format!("Unable to save history: {}", e)); }
            _ => {}
        }
    }

    pub fn to_str(&self) -> String {
        let mut s = String::with_capacity(1024);
        let h = self.hist.deref().borrow();
        if h.len() <= 0 {
            s.push_str("No history as yet");
        } else {
            println!("hist size: {}", h.len());
            for (i, o) in h.iter().enumerate().rev() {
                s.push_str(&format!("{}: {}\r\n", i, o));
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
