use std::{
    collections::VecDeque,
    sync::mpsc::{channel, Receiver, Sender}
};

pub trait IO {
    fn push_input(&mut self, val: i64);
    fn pop_input(&mut self) -> i64;

    fn push_output(&mut self, val: i64);
    fn pop_output(&mut self) -> i64;
}

pub struct NoIO;

impl IO for NoIO {
    fn push_input(&mut self, _: i64) {}
    fn pop_input(&mut self) -> i64 { 0 }

    fn push_output(&mut self, _: i64) {}
    fn pop_output(&mut self) -> i64 { 0 }
}

pub struct SingleIO {
    val: i64,
}

impl SingleIO {
    pub fn new(val: i64) -> Self { Self { val } }
}

impl IO for SingleIO {
    fn push_input(&mut self, val: i64) { self.val = val; }
    fn pop_input(&mut self) -> i64 { self.val }

    fn push_output(&mut self, val: i64) { self.val = val; }
    fn pop_output(&mut self) -> i64 { self.val }
}

pub struct QueueIO {
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

impl QueueIO {
    pub fn new() -> Self {
        Self {
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn new_init(init: Vec<i64>) -> Self {
        Self {
            input: VecDeque::from(init),
            output: VecDeque::new(),
        }
    }
}

impl IO for QueueIO {
    fn push_input(&mut self, val: i64) { self.input.push_back(val); }
    fn pop_input(&mut self) -> i64 { self.input.pop_front().expect("No input available") }

    fn push_output(&mut self, val: i64) { self.output.push_back(val); }
    fn pop_output(&mut self) -> i64 { self.output.pop_front().expect("No output available") }
}

pub struct AsyncIO {
    tx: Vec<Sender<i64>>,
    rx: Option<Receiver<i64>>,
    input: Vec<i64>,
}

impl AsyncIO {
    pub fn new() -> Self {
        Self {
            tx: Vec::new(),
            rx: None,
            input: Vec::new(),
        }
    }

    pub fn new_init(input: Vec<i64>) -> Self {
        Self {
            tx: Vec::new(),
            rx: None,
            input,
        }
    }

    pub fn get_receiver(&mut self) -> Receiver<i64> {
        let (tx, rx) = channel();
        self.tx.push(tx);
        rx
    }

    pub fn set_receiver(&mut self, rx: Receiver<i64>) {
        self.rx = Some(rx);
    }
}

impl IO for AsyncIO {
    fn push_input(&mut self, val: i64) {
        self.input.push(val);
    }

    fn pop_input(&mut self) -> i64 {
        if let Some(val) = self.input.pop() {
            val
        }
        else if let Some(rx) = &self.rx {
            rx.recv().unwrap_or(0)
        }
        else {
            0
        }
    }

    fn push_output(&mut self, val: i64) {
        for tx in &self.tx {
            let _ = tx.send(val);
        }
    }

    fn pop_output(&mut self) -> i64 {
        unimplemented!()
    }
}