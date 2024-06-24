use crate::text_rendering::TextRenderer;

pub struct ConsoleTextRenderer{

}

impl ConsoleTextRenderer{
    pub fn new() -> Box<ConsoleTextRenderer>{
        Box::new(ConsoleTextRenderer{})
    }

}

impl TextRenderer for ConsoleTextRenderer{

    fn render_text(&self, text: &str){
        println!("{text}");
    }
}

