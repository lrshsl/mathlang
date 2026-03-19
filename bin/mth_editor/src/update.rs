use graph_canvas::{N_INSTRUCTIONS, N_PLOTS};
use mth_common::{ops::Instruction, plot_desc::PlotDesc};

use crate::{message::Message, MainState, ZOOM_WHEEL_SCALE};

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

                #[cfg(not(target_arch = "wasm32"))]
                std::fs::write("output/last_ast", format!("{module:#?}"))
                    .expect("Couldn't write ast to file: output dir doesn't exists?");

                // Codegen
                match code_generator::compile_module(&module) {

                    // Ok
                    Ok((mut instructions, plot_desc)) if instructions.len() <= N_INSTRUCTIONS => {
                        self.write_instructions(&mut instructions, &plot_desc);
                        self.update(Message::ClearErrors);
                    }

                    // Too many instructions
                    Ok((instructions, _plot_desc)) => self.update(Message::SetError(
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
    fn write_instructions(
        &mut self,
        instructions: &mut Vec<Instruction>,
        plot_desc: &[PlotDesc; N_PLOTS],
    ) {
        if instructions.len() % 2 == 1 {
            // Can only write a even amount to the shader
            instructions.push(Instruction::default())
        }

        let len = instructions.len();
        assert!(len <= N_INSTRUCTIONS); // Still true after push anyways

        let len = instructions.len();

        // Copy instructions into the graphs mutex
        {
            self.graph
                .instructions
                .lock()
                .expect("Could not lock instructions mutex in MainState::update")[..len]
                .copy_from_slice(&instructions[..len]);
        }
        self.graph.plot_desc = *plot_desc;
        self.graph.instructions_dirty = true;
    }
}
