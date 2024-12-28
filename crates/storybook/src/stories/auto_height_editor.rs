use editor::Editor;
use gpui::{
    div, white, AppContext, IntoElement, KeyBinding, ParentElement, Render, Styled, View,
    ViewContext, VisualContext, Window,
};

pub struct AutoHeightEditorStory {
    editor: View<Editor>,
}

impl AutoHeightEditorStory {
    pub fn new(window: &mut Window, cx: &mut AppContext) -> View<Self> {
        cx.bind_keys([KeyBinding::new(
            "enter",
            editor::actions::Newline,
            Some("Editor"),
        )]);
        window.new_view(cx, |cx| Self {
            editor: cx.new_view(|cx| {
                let mut editor = Editor::auto_height(3, cx);
                editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
                editor
            }),
        })
    }
}

impl Render for AutoHeightEditorStory {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(white())
            .text_sm()
            .child(div().w_32().bg(gpui::black()).child(self.editor.clone()))
    }
}
