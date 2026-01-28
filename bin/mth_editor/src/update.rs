use std::sync::Arc;

use crate::{MainState, ZOOM_WHEEL_SCALE, message::Message};

impl MainState {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::EditText(action) => {
                self.text.perform(action);
                let text = self.text.text();
                match mth_parser::parse_program(&text) {
                    Ok((_rem, module)) => match code_generator::compile_module(&module) {
                        Ok(instructions) => {
                            self.graph.instructions = Arc::new(instructions);
                            self.graph.instructions_dirty = true;
                            self.update(Message::ClearErrors);
                        }
                        Err(e) => {
                            self.update(Message::SetError(format!("Code generation failed: {e:?}")))
                        }
                    },
                    Err(e) => self.update(Message::SetError(format!("{e}"))),
                }
            }
            Message::PanningDelta(delta) => {
                self.graph.controls.center -= 2.0 * delta * self.graph.controls.scale();
            }
            Message::ZoomDelta(_pos, _bounds, delta) => {
                let delta = delta * ZOOM_WHEEL_SCALE;
                // let prev_scale = self.graph.controls.scale();
                let prev_zoom = self.graph.controls.zoom;
                self.graph.controls.zoom = prev_zoom + delta;

                // let vec = pos - dvec2(bounds.width.into(), bounds.height.into()) * 0.5;
                // let new_scale = self.graph.controls.scale();
                // self.graph.controls.center += vec * (prev_scale - new_scale) * 2.0;
            }
            Message::SetError(err_msg) => self.err_msg = Some(err_msg),
            Message::ClearErrors => self.err_msg = None,
        }
    }
}
