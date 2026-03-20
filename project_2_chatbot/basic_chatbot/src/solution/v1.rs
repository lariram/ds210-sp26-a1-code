use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV1 {
    //set the model as Llama:
    model: Llama,
}

impl ChatbotV1 {
    // create the chatbot with the model Llama
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV1 {
        return ChatbotV1 { model: model };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        // set the model name as chat_session using chatbot model Llama and as a pirate.
        let mut chat_session: Chat<Llama> = self.model 
            .chat()
            .with_system_prompt("The assistant will act like a pirate"); 
        
        // Allow the user to input message:
        let asynchronous_output = chat_session.add_message(message); // returns response asynchronously

        // generate output of the chatbot:
        let output = asynchronous_output.await; // waits for response to be ready before returning

        // reveal the output as string:
        let output = output.unwrap(); // extracts the response as a string

        // return the output to the user:
        return output
    }
}