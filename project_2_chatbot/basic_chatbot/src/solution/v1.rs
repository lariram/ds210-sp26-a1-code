use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV1 {
    model: Llama,
}

impl ChatbotV1 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV1 {
        return ChatbotV1 { model: model };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate"); 
        
        
        let asynchronous_output = chat_session.add_message(message); // returns response asynchronously
        let output = asynchronous_output.await; // waits for response to be ready before returning
        let output = output.unwrap(); // extracts the response as a string

        return output
    }
}