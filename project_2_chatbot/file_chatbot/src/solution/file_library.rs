use kalosm::language::*;

// Look at the docs for std::fs
// https://doc.rust-lang.org/std/fs/index.html
// std::fs provides functions that write to a file, read from a file,
// check if a file exists, etc.
use std::fs;

// LlamaChatSession provides helpful functions for loading and storing sessions.
// Look at https://docs.rs/kalosm/latest/kalosm/language/trait.ChatSession.html#saving-and-loading-sessions
// for some examples!

// Implement this
pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) {
    // look at fs::write(...)
    let session_as_bytes = session.to_bytes().unwrap();

    // And write those bytes to a file
    std::fs::write(filename, session_as_bytes).unwrap();
}


// Implement this
pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
    // look at fs::read(...)
    // also look at LlamaChatSession::from_bytes(...)
    println!("Loading chat session from file {filename}");

    // Try to read the file
    let session_as_bytes = match fs::read(filename) {
        // if there is a file, called it bytes:
        Ok(bytes) => bytes,

        // if there is no a file, print that the file is failed to read and return None:
        Err(err) => {
            println!("Failed to read file: {:?}", err);
            return None;
        }
    };

    // Try to correct the type of session
    let session = match LlamaChatSession::from_bytes(&session_as_bytes) {
        // if the type of the session is corect, use it as session:
        Ok(session) => session,

        // if the type is incorrect, return error message:
        Err(err) => {
            println!("Failed to analyze session: {:?}", err);
            return None;
        }
    };

    //revel the contents of session:
    Some(session)
}