use std::ops::{Deref, DerefMut};

use rig::{
    OneOrMany,
    agent::Agent,
    completion::{Completion, PromptError},
    message::{AssistantContent, Message, ToolCall, ToolFunction, ToolResultContent, UserContent},
};
use serde_json::json;

pub struct MultiTurnAgent<M: rig::completion::CompletionModel + Send + Sync> {
    agent: Agent<M>,
    chat_history: Vec<Message>,
}

impl<M: rig::completion::CompletionModel> MultiTurnAgent<M> {
    pub fn new(agent: Agent<M>) -> Self {
        Self {
            agent,
            chat_history: Vec::new(),
        }
    }

    pub async fn multi_turn_prompt(
        &mut self,
        prompt: impl Into<Message> + Send,
    ) -> Result<String, PromptError> {
        let mut current_prompt: Message = prompt.into();
        loop {
            // println!("Current Prompt: {:?}\n", current_prompt);
            // for history in self.chat_history.iter() {
            //     println!("Chat History: {:?}\n", history);
            // }
            let resp = self
                .agent
                .completion(current_prompt.clone(), self.chat_history.clone())
                .await?
                .send()
                .await?;

            self.chat_history.push(current_prompt.clone());

            let mut final_text = None;
            let resp_choice_len = resp.choice.len();

            for content in resp.choice.into_iter() {
                match content {
                    AssistantContent::Text(text) => {
                        if resp_choice_len > 1 {
                            println!("Intermediate Response (CoT): {:?}", text.text);
                        }
                        final_text = Some(text.text.clone());
                        let response_message = Message::Assistant {
                            content: OneOrMany::one(AssistantContent::text(&text.text)),
                        };
                        self.chat_history.push(response_message);
                    }
                    AssistantContent::ToolCall(content) => {
                        let tool_call_msg = AssistantContent::ToolCall(content.clone());

                        self.chat_history.push(Message::Assistant {
                            content: OneOrMany::one(tool_call_msg),
                        });

                        let ToolCall {
                            id,
                            function: ToolFunction { name, arguments },
                        } = content;

                        let tool_result: ToolResult = (
                            name.clone(),
                            self.agent.tools.call(&name, arguments.to_string()).await?,
                        )
                            .into();

                        current_prompt = Message::User {
                            content: OneOrMany::one(UserContent::tool_result(
                                id,
                                OneOrMany::one(tool_result.into()),
                            )),
                        };

                        final_text = None;
                        break;
                    }
                }
            }

            if let Some(text) = final_text {
                return Ok(text);
            }
        }
    }

    pub async fn clear_history(&mut self) {
        self.chat_history.clear();
    }
}
impl<M: rig::completion::CompletionModel + Send + Sync> Deref for MultiTurnAgent<M> {
    type Target = Agent<M>;

    fn deref(&self) -> &Self::Target {
        &self.agent
    }
}
impl<M: rig::completion::CompletionModel + Send + Sync> DerefMut for MultiTurnAgent<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.agent
    }
}

pub struct ToolResult(String, String);
impl From<(String, String)> for ToolResult {
    fn from(value: (String, String)) -> Self {
        Self(value.0, value.1)
    }
}
impl From<ToolResult> for ToolResultContent {
    fn from(val: ToolResult) -> Self {
        ToolResultContent::text(
            json!({
                "name": val.0,
                "result": val.1
            })
            .to_string(),
        )
    }
}
