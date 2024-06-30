pub struct RenderableUiOption<'a> {
    pub label: &'a str,
    pub shortcut: char,
}

pub struct RenderableTextUi<'a> {
    options: Vec<RenderableUiOption<'a>>,
}

impl<'a> RenderableTextUi<'a> {
    pub fn new(options: Vec<RenderableUiOption<'a>>) -> Self {
        RenderableTextUi { options }
    }
    pub fn options_iter(&self) -> std::slice::Iter<RenderableUiOption<'a>> {
        self.options.iter()
    }
}
