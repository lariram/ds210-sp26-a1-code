use kalosm::language::*;
use crate::solution::file_library::{self, save_chat_session_to_file};
use std::{collections::HashMap, hash::Hash};

pub struct ChatbotV4 {
    model: Llama,
    usernames: HashMap<String, Chat<Llama>>
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        // create the chatbot with name chat_session in model of Llama as a pirate:
        let chat_session = model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        // return the Chatbot with correct model and generate a new username:
        return ChatbotV4 {
            model: model,
            usernames: HashMap::new()
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        
        if !self.usernames.contains_key(&username) {
        let chat = match file_library::load_chat_session_from_file(&filename) {
            Some(session) => self.model.chat().with_session(session),
            None => self
                .model
                .chat()
                .with_system_prompt("The assistant will act like a pirate"),
        };

        self.usernames.insert(username.clone(), chat);
    }

        // TODO: You have to implement the rest:
        // You need to load the chat session from the file using file_library::load_chat_session_from_file(...).
        // Think about what needs to happen if the function returns None vs Some(session).
        // Hint: look at https://docs.rs/kalosm/latest/kalosm/language/struct.Chat.html#method.with_session
        
        // if the username is stored in the Hashmap, username has been used to talk with chatbot before:
        // fetch the same chat session of that username:
        let chat_session = self.usernames.get_mut(&username).unwrap();

        // Allow the user to input message:
        let asynchronous_output = chat_session.add_message(message); // returns response asynchronously

        // generate output of the chatbot:
        let output = asynchronous_output.await; // waits for response to be ready before returning

        // reveal the output as string:
        let output = output.unwrap(); // extracts the response as a string

        let session = chat_session.session().unwrap();

        save_chat_session_to_file(filename, &session);

        // return the output to the user:
        return output
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new(); // create a new vector of strings (to hold future messages)
            },
            Some(session) => {
                // TODO: what should happen here?
                
                let mut string_output = Vec::new(); // create a new vector of strings (to hold messages)
                for message in session.history().iter().skip(1) { // loop through messages of session history, skipping initial prompt
                    string_output.push(message.content().to_string()); // convert each message to a string and add it to the vector
                }

                return string_output; // return vector of string messages
            }
        }
    }
}