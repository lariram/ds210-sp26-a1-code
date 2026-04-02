use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                // create a variable called chat:
                let mut chat = match file_library::load_chat_session_from_file(&filename) {
                    
                // if the chat session exists and saved before, load it to be chat:
                Some(session) => self.model.chat().with_session(session),

                // if there isn't any chat session file being saved before, create a new chatbot session:
                None => self
                    .model
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate")
                    };
                
                // Allow the user to input message:
                let asynchronous_output = chat.add_message(message); // returns response asynchronously

                // generate output of the chatbot:
                let output = asynchronous_output.await; // waits for response to be ready before returning

                // reveal the output as string:
                let output = output.unwrap(); // extracts the response as a string

                let session = chat.session().unwrap();

                //save the file:
                file_library::save_chat_session_to_file(filename, &session);

                let cloned_chat = self.model.chat().with_session(session.clone()); // create clone of chat session

                // insert cloned chat to the cache:
                self.cache.insert_chat(username.clone(), cloned_chat);
                // return the output to the user:
                return output
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                
                // if the username is stored in the Hashmap, username has been used to talk with chatbot before:

                // Allow the user to input message:
                let asynchronous_output = chat_session.add_message(message); // returns response asynchronously

                // generate output of the chatbot:
                let output = asynchronous_output.await; // waits for response to be ready before returning

                // reveal the output as string:
                let output = output.unwrap(); // extracts the response as a string

                let session = chat_session.session().unwrap();

                //save the file:
                file_library::save_chat_session_to_file(filename, &session);

                // return the output to the user:
                return output

            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        // ensuring the filename is in the right format using the username:
        let filename = &format!("{}.txt", username);

        // fetch the chat session from the cache using username:
        let cached_chat = self.cache.get_chat(&username);
        
        // see if the chat is in the cache:
        match cached_chat {

            // if the chat is not in the cache:
            None => {
                // print the information message:
                println!("get_history: {username} is not in the cache!");
                // TODO: The cache does not have the chat. What should you do?
                // Your code goes here.
                
                // to see if the chat is saved as file before:
                let chat = match file_library::load_chat_session_from_file(&filename) {
                    // if the file is saved before, load the chat to have the session contents:
                    Some(session) => self.model.chat().with_session(session),

                    // if it is not being saved as a file before, create a new model:
                    None => self
                    .model
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate"),
                };

                // creating a session with the correct type from chat:
                let session = chat.session().unwrap();

                // create a new vecter to store the output:
                let mut string_output = Vec::new();

                // loop through the history and make sure it is in the correct order:
                for message in session.history().iter().skip(1) {

                    // add each line of message to the output:
                    string_output.push(message.content().to_string());
                }

                let cloned_chat = self.model.chat().with_session(session.clone()); // create clone of chat session

                // insert cloned chat to the cache:
                self.cache.insert_chat(username.clone(), cloned_chat);

                // return the history in a vector of strings:
                return string_output;
            }

            // if the chat session exists in the cache:
            Some(chat_session) => {

                //print the information message:
                println!("get_history: {username} is in the cache! Nice!");
                // TODO: The cache has this chat. What should you do?
                // Your code goes here.

                // make the chat_session in the correct type:
                let tmp = chat_session.session().unwrap();

                // create a new vecter to store the output:
                let mut string_output = Vec::new();

                // loop through the history and make sure it is in the correct order:
                for message in tmp.history().iter().skip(1) {
                    // add each line of message to the output:
                    string_output.push(message.content().to_string());
                }

                // return the history in a vector of strings:
                return string_output;

            }
        }
    }
}