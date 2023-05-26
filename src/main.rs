use std::fmt::Display;
use std::io::Write;
use std::path::Display;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use rand;
use iced::widget::{Column, Text, container, text, column, text_input, button, row, pick_list};
use llm::{Model, InferenceFeedback, InferenceResponse};
use iced::{Application, Length, Command};
use iced::{executor, Theme, Settings, Element};
use tray_icon::{TrayIconBuilder, menu::Menu};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display)]
pub enum LLMOptions {
    #[default]
    Normal,
    Creative,
    Precise
}
impl std::fmt::Display for LLMOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LLMOptions::Normal=> "Normal",
                LLMOptions::Creative=> "Creative",
                LLMOptions::Precise=> "Precise",
            }
        )
    }
}
enum LlamaState {
    Loading,
    Finished,
    NotRunning
}

#[derive(Default, Clone)]
struct App {    
    prompt: String,
    messages: Vec<String>
}
#[derive(Debug, Clone)]
enum AppMessage {
    GeneratePressed,
    PromptChanged(String),
    A,
    LlamaState
}
fn on_select(selected: LLMOptions) {

}
impl Application for App {
    type Message = AppMessage;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn view(&self) -> Element<AppMessage> {
        
        for message in self.messages {
            text
        }

            container(column![
                text("Chat With LlaMa"),
                // pick_list(LLMOptions, LLMOptions::Normal, on_select),
                row![
                text_input("Enter a message", &self.prompt).on_input(AppMessage::PromptChanged)],
                button("Send")
            ])
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }
    fn title(&self) -> std::string::String { String::from("UwU")}
    fn new(_flags: ()) -> (App, Command<AppMessage>) {
        (App {
            prompt: String::from("Hewwo World"),
            messages: Vec::new()
        }, Command::none())
    }
    fn update(&mut self, message: AppMessage) -> iced::Command<AppMessage> { 
        println!("Updated!");
        match message {
            AppMessage::PromptChanged(a) => {
                self.prompt = a
            }
            _ => todo!()
        }
        Command::none()
     }

}
fn predictLlama(tx: Sender<String>) {
    println!("Hello, world!");
    let params = llm::InferenceParameters::default();

    let llama = llm::load::<llm::models::Llama> (
        std::path::Path::new("model.bin"),
        Default::default(),
        None,
        llm::load_progress_callback_stdout
    )
    .unwrap();

    let mut session = llama.start_session(Default::default());
    session.feed_prompt(&llama, &params, "Expand the following: Hello World!", &mut Default::default(), llm::feed_prompt_callback(|resp| match resp {
        InferenceResponse::InferredToken(data) | InferenceResponse::PromptToken(data) => {println!("{}", data)}
        _ => Ok(InferenceFeedback::Continue)
    })).unwrap();
let _res = session.infer::<std::convert::Infallible>(
    // model to use for text generation
    &llama,
    // randomness provider
    &mut rand::thread_rng(),
    // the prompt to use for text generation, as well as other
    // inference parameters
    &llm::InferenceRequest {
        prompt: llm::Prompt::Text("You are a language learning model assistant. Respond in a few words to whatever prompt the user asks. User: What is the difference between the stack and the heap?"),
        parameters: &params,
        play_back_previous_tokens: false,
        maximum_token_count: None,
    },
    // llm::OutputRequest
    &mut Default::default(),
    // output callback
    |t| {
        
        match t {
            llm::InferenceResponse::InferredToken(data) => tx.send(data).unwrap(),
            llm::InferenceResponse::SnapshotToken(_) => println!("snapshot"),
            llm::InferenceResponse::PromptToken(data) => println!("{}", data),
            llm::InferenceResponse::EotToken => println!("end of token"),
        };

        // // print!("{t}");
        // std::io::stdout().flush().unwrap();
        Ok(InferenceFeedback::Continue)
    }
);
    }

fn main() -> iced::Result {
    let (tx, rx) = channel();


let tray_menu = Menu::new();
let tray_icon = TrayIconBuilder::new()
    .with_menu(Box::new(tray_menu))
    .with_tooltip("system-tray - tray icon library!")
    .build()
    .unwrap();

    thread::spawn(move || predictLlama(tx));
    thread::spawn(move || {
        loop {
            let val = rx.recv().unwrap();
            println!("{}", val);
        }
    });

    


    App::run(Settings::default())
}
