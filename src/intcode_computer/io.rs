use std::{
    sync::mpsc::{channel, Receiver, Sender},
    io::{
        Result,
        Error,
        ErrorKind
    }
};

pub trait IO {
    fn read(&mut self) -> Result<i64>;
    fn write(&mut self, val: i64);
}

pub struct NoIO;

impl IO for NoIO {
    fn read(&mut self) -> Result<i64> { Ok(0) }
    fn write(&mut self, _: i64) {}
}

pub struct SingleIO {
    val: i64,
}

impl SingleIO {
    pub fn new(val: i64) -> Self { Self { val } }
}

impl IO for SingleIO {
    fn read(&mut self) -> Result<i64> { Ok(self.val) }
    fn write(&mut self, val: i64) { self.val = val; }
}

pub struct AsyncIO {
    tx: Sender<i64>,
    rx: Receiver<i64>,
}

impl AsyncIO {
    pub fn new() -> (Self, Sender<i64>, Receiver<i64>) {
        let (tx_in, rx_in) = channel();
        let (tx_out, rx_out) = channel();

        let io = Self {
            tx: tx_out,
            rx: rx_in,
        };

        (io, tx_in, rx_out)
    }
}

impl IO for AsyncIO {
    fn read(&mut self) -> Result<i64> {
        self.rx.recv()
            .map_err(|e| Error::new(ErrorKind::BrokenPipe, e))
    }

    fn write(&mut self, val: i64) {
        let _ = self.tx.send(val);
    }
}

pub struct Pipe {
    rx: Receiver<i64>,
    tx: Vec<Sender<i64>>,
}

impl Pipe {
    pub fn new(rx: Receiver<i64>, tx: Vec<Sender<i64>>) -> Self {
        Self {
            tx,
            rx,
        }
    }

    pub fn run(&self) {
        while let Ok(val) = self.rx.recv() {
            for tx in &self.tx {
                let _ = tx.send(val);
            }
        }
    }
}