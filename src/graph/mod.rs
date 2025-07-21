use std::sync::Arc;

use glam::{DVec2, dvec2};
use iced::{
    Rectangle,
    advanced::Shell,
    event::Status,
    mouse::{self, Cursor},
    widget::shader,
};

use crate::{
    Message,
    graph::{
        fragment_shader_primitive::FragmentShaderPrimitive, graph_shader_pipeline::Controls,
        ops::Instruction,
    },
};

mod fragment_shader_primitive;
mod graph_shader_pipeline;
pub mod ops;

#[derive(Default)]
pub struct Graph {
    pub controls: Controls,
    pub instructions: Arc<Vec<Instruction>>,
    pub instructions_dirty: bool,
}

impl shader::Program<Message> for Graph {
    type State = (bool, DVec2);
    type Primitive = FragmentShaderPrimitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Self::Primitive {
        FragmentShaderPrimitive::new(
            self.controls,
            Arc::clone(&self.instructions),
            self.instructions_dirty,
        )
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: shader::Event,
        bounds: Rectangle,
        cursor: Cursor,
        _shell: &mut Shell<'_, Message>,
    ) -> (Status, Option<Message>) {
        // Zooming
        if let shader::Event::Mouse(mouse::Event::WheelScrolled { delta }) = event {
            if let Some(pos) = cursor.position_in(bounds) {
                let pos = DVec2::new(pos.x.into(), pos.y.into());
                let delta = match delta {
                    mouse::ScrollDelta::Lines { y, .. } => y,
                    mouse::ScrollDelta::Pixels { y, .. } => y,
                };
                return (
                    Status::Captured,
                    Some(Message::ZoomDelta(pos, bounds, delta.into())),
                );
            }
        }

        // Panning
        match state {
            (false, _) => match event {
                shader::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    if let Some(pos) = cursor.position_over(bounds) {
                        *state = (true, dvec2(pos.x.into(), pos.y.into()));
                        return (Status::Captured, None);
                    }
                }
                _ => {}
            },
            (true, prev_pos) => match event {
                shader::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    (*state).0 = false;
                }
                shader::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    let pos = DVec2::new(position.x.into(), position.y.into());
                    let delta = pos - *prev_pos;
                    *state = (true, pos);
                    return (Status::Captured, Some(Message::PanningDelta(delta)));
                }
                _ => {}
            },
        }

        (Status::Ignored, None)
    }
}
