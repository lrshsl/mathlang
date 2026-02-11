use graph_canvas::N_INSTRUCTIONS;
use mth_common::ops::Instruction;

use crate::{MainState, ZOOM_WHEEL_SCALE, message::Message};

impl MainState {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::EditText(action) => {
                self.text.perform(action);
                self.on_text_change();
            }
            Message::PanningDelta(delta) => {
                self.graph.controls.offset -= 2.0 * delta * self.graph.controls.pixel_ratio();
            }
            Message::ZoomDelta(_pos, _bounds, delta) => {
                let delta = delta * ZOOM_WHEEL_SCALE;
                let prev_zoom = self.graph.controls.zoom;
                self.graph.controls.zoom = prev_zoom + delta;
            }
            Message::SetError(err_msg) => self.err_msg = Some(err_msg),
            Message::ClearErrors => self.err_msg = None,
        }
    }

    pub fn on_text_change(&mut self) {
        let text = &self.text.text();

        // Parsing
        match mth_parser::parse_program(&text) {
            // Ok
            Ok((rem, module)) if rem.remainder.is_empty() => {
                println!("{module:#?}");

                // Codegen
                match code_generator::compile_module(&module) {

                    // Ok
                    Ok(instructions) if instructions.len() <= N_INSTRUCTIONS => {
                        self.write_instructions(&instructions);
                        self.update(Message::ClearErrors);
                    }

                    // Too many instructions
                    Ok(instructions) => self.update(Message::SetError(
                        format!("The generated instructions don't fit into the GPU instruction buffer. Got {n} instructions", n = instructions.len())
                    )),

                    // Error
                    Err(e) => {
                        self.update(Message::SetError(format!("Code generation failed: {e:?}")))
                    }
                }
            }

            // Parse error
            Err(e) => self.update(Message::SetError(format!("{e}"))),

            // Incomplete parse
            Ok((rem, _)) => self.update(Message::SetError(format!(
                "Couldn't parse everything. Unparsed input: {}",
                rem.remainder
            ))),
        }
    }

    /// Needs to be called with a instructions buffer of length `N_INSTRUCTIONS` or less. Also,
    /// requires the length of the GPU buffer (`self.graph.instructions`) to be equal or greater
    /// than `N_INSTRUCTIONS`
    fn write_instructions(&mut self, instructions: &[Instruction]) {
        let len = instructions.len();
        assert!(len <= N_INSTRUCTIONS);

        // Copy instructions into the graphs mutex
        {
            self.graph
                .instructions
                .lock()
                .expect("Could not lock instructions mutex in MainState::update")[..len]
                .copy_from_slice(&instructions[..len]);
        }
        self.graph.instruction_count = len;
        self.graph.instructions_dirty = true;
    }
}
