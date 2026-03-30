use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    chat_session: Chat<Llama> // Create a model using Llama called chat_session
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        // set the chat model to be Llama and make the chatbot to be a pirate.
        let chat_session = model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");
        
        return ChatbotV2 {
            // return the ChatbotV2 with chatbot model as a pirate.
            chat_session
            // Whatever you decide to store in the struct
            // you need to make sure you pass here!
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        
        // allow the user to input message to the model:
        let asynchronous_output = self.chat_session.add_message(message);

        // Create a output of the model and don't interrupt other tasks running:
        let output = asynchronous_output.await;

        // reveal the content of the output as string.
        let output = output.unwrap();

        // You need to add your code here
        // You must find a way to add the given message to the chat_session!
        // consider https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.add_message
        // Hint: make sure you transform/extract the response message as a **String**.

        // show the output of chatbot to the user
        return output;
    }
}