use std::{collections::HashMap, hash::Hash};

use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV3 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.

    // set the model to be Llama chatbot:
    model: Llama,

    // create a HashMap containing usernames and their corresponding chatbot:
    usernames: HashMap<String, Chat<Llama>>

}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {

        // create the chatbot with name chat_session in model of Llama as a pirate:
        let chat_session = model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        // return the Chatbot with correct model and generate a new username:
        return ChatbotV3 {
            // Make sure you initialize your struct members here
            model,
            usernames: HashMap::new()
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.
        
        // if the username is not contained in the Hashmap of usernames, new username:
        if !self.usernames.contains_key(&username) {
            // create new chat session with correct model and as a pirate chatbot:
            let chat_session = self
                .model
                .chat()
                .with_system_prompt("The assistant will act like a pirate");
            
            // insert the username into Hashmap as well as the contents of chat session:
            self.usernames.insert(username.clone(), chat_session);
        }

        // if the username is stored in the Hashmap, username has been used to talk with chatbot before:
        // fetch the same chat session of that username:
        let chat_session = self.usernames.get_mut(&username).unwrap();

        // allow the user to input message to the chatbot:
        let asynchronous_output = chat_session.add_message(message);

        // Create a output of the model and don't interrupt other tasks running:
        let output = asynchronous_output.await;

        // reveal the content of the output as string.
        let output = output.unwrap();

        // return the message from the chatbot as replying:
        return output;
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        // Extract the chat message history for the given username
        // Hint: think of how you can retrieve the Chat object for that user, when you retrieve it
        // you may want to use https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.session
        // to then retrieve the history!
    
    // if the username hasn't been used before:
    if !self.usernames.contains_key(&username) {
        // return a new and empty chat session (vecter of strings):
        return Vec::new();
    }

    // if the username has been used before:
    // reveal the contents of the chat session of the username:
    let chat_session = self.usernames.get(&username).unwrap();

    // fetch the history of the chat session using history function as a vectors of strings:
    let history = chat_session.session().unwrap().history();

    // create an empty output of the history for now:
    let mut string_output: Vec<String> = Vec::new();

    // loop through every messages in the history of the chat session and skip the first system message:
    // so we have the correct order of the chat messages:
    for message in history.into_iter().skip(1) {

        // fetch each message from the history of chat session:
        let content: String = String::from(message.content());

        // add that message to the output:
        string_output.push(content);
    }

    // print the string output in terminal too:
    println!("{:?}", string_output);

    // return the history output on the user web page:
    return string_output;
    
    }
}