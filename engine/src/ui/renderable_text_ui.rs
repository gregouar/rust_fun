use super::TextUiOrientation;

pub struct RenderableUiOption<'a> {
    pub label: &'a str,
    pub shortcut: char,
}

pub struct RenderableTextUi<'a> {
    orientation: TextUiOrientation,
    options: Vec<RenderableUiOption<'a>>,
}

impl<'a> RenderableTextUi<'a> {
    pub fn new(orientation: TextUiOrientation, options: Vec<RenderableUiOption<'a>>) -> Self {
        RenderableTextUi {
            orientation,
            options,
        }
    }

    pub fn options_iter(&self) -> std::slice::Iter<RenderableUiOption<'a>> {
        self.options.iter()
    }

    pub fn orientation(&self) -> TextUiOrientation {
        self.orientation
    }
}
