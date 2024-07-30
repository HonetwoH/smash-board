use std::{
    process::Command,
    thread::{self},
    time::Duration,
};

pub enum Environment {
    Wayland,
    X11,
}

#[derive(Debug)]
pub enum ClipboardError {
    FailedToRunCommand,
    CommandReturnedUnSuccesfully,
}

pub struct Clipboard {
    environment: Environment,
    previous: Vec<u8>,
    polling_rate: u64,
}

impl Clipboard {
    pub fn new(environment: Environment, polling_rate: u64) -> Self {
        Self {
            environment,
            polling_rate,
            previous: Vec::new(),
        }
    }

    pub fn poll(&mut self) -> Result<Vec<u8>, ClipboardError> {
        loop {
            thread::sleep(Duration::from_secs(self.polling_rate));
            let paste = self.get_clipboard()?;
            if !are_equal(&self.previous, &paste) {
                self.previous = paste.clone();
                return Ok(paste);
            }
        }
    }

    fn get_clipboard(&self) -> Result<Vec<u8>, ClipboardError> {
        let output = match self.environment {
            Environment::Wayland => Command::new("wl-paste")
                .output()
                .map_err(|_| ClipboardError::FailedToRunCommand)?,
            Environment::X11 => Command::new("xclip")
                .arg("-o")
                .output()
                .map_err(|_| ClipboardError::FailedToRunCommand)?,
        };
        // Make this better
        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err(ClipboardError::CommandReturnedUnSuccesfully)
        }
    }
}

#[test]
fn test_get_clipboard() {
    let mut clip = Clipboard::new(Environment::Wayland, 2);
    _ = clip.poll();
}

fn are_equal(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        false
    } else {
        let l = a.len();
        for i in 0..l {
            if a[i] != b[i] {
                return false;
            }
        }
        true
    }
}
