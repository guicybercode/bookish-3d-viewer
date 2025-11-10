use crate::transform::Transform;
use crate::selection::Selection;
use crate::color_picker::ColorPicker;

pub struct Editor {
    pub transform: Transform,
    pub selection: Selection,
    pub color_picker: ColorPicker,
    pub history: Vec<EditorState>,
    pub history_index: usize,
    pub max_history: usize,
}

#[derive(Clone)]
pub struct EditorState {
    transform: Transform,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            selection: Selection::new(),
            color_picker: ColorPicker::new(),
            history: vec![EditorState {
                transform: Transform::new(),
            }],
            history_index: 0,
            max_history: 100,
        }
    }

    pub fn save_state(&mut self) {
        let state = EditorState {
            transform: self.transform.clone(),
        };

        self.history.truncate(self.history_index + 1);
        self.history.push(state);
        self.history_index += 1;

        if self.history.len() > self.max_history {
            self.history.remove(0);
            self.history_index -= 1;
        }
    }

    pub fn undo(&mut self) -> bool {
        if self.history_index > 0 {
            self.history_index -= 1;
            let state = &self.history[self.history_index];
            self.transform = state.transform.clone();
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            let state = &self.history[self.history_index];
            self.transform = state.transform.clone();
            true
        } else {
            false
        }
    }

    pub fn can_undo(&self) -> bool {
        self.history_index > 0
    }

    pub fn can_redo(&self) -> bool {
        self.history_index < self.history.len() - 1
    }

    pub fn reset(&mut self) {
        self.transform.reset();
        self.selection.clear();
        self.save_state();
    }
}

