use std::{
    fmt,
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
};

pub mod arithmetic;
#[cfg(feature = "parsing")]
pub mod parsers;
#[cfg(feature = "types_2d")]
pub mod types_2d;

/// Generic error for when the value of something is wrong
///
/// Fine for simple parsing of types from Strings
#[derive(Clone, Debug)]
pub struct ValueError<T: fmt::Display + fmt::Debug>(pub T);

impl<T> std::error::Error for ValueError<T> where T: fmt::Display + fmt::Debug {}
impl<T> fmt::Display for ValueError<T>
where
    T: fmt::Display + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid value: {}", self.0)
    }
}

pub struct Msg<T>
where
    T: Send,
{
    pub part: u8,
    pub value: T,
}

impl<T> Msg<T>
where
    T: Send,
{
    fn new(part: u8, value: T) -> Self {
        Msg { part, value }
    }
}

pub fn run_part_threaded<A, F>(
    part: u8,
    input: Arc<String>,
    solver: F,
    channel: Sender<Msg<A>>,
) -> JoinHandle<()>
where
    A: Send + 'static,
    F: Fn(&str) -> A + Send + 'static,
{
    thread::spawn(move || {
        let answer = solver(input.as_ref());
        channel.send(Msg::new(part, answer)).expect("Send answer");
    })
}

pub fn receive_answers<A: Send + fmt::Display>(rx: Receiver<Msg<A>>) {
    while let Ok(Msg {
        value: answer,
        part,
    }) = rx.recv()
    {
        println!("Got {} for part {}", answer, part);
    }
}
