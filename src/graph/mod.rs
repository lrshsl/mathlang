use glam::Vec2;
use iced::{
    Rectangle,
    advanced::Shell,
    event::Status,
    mouse::{self, Cursor},
    widget::shader,
};

use crate::{graph::graph_shader_pipeline::Controls, Message};
mod graph_shader_pipeline;
use graph_shader_pipeline::FragmentShaderPrimitive;

#[derive(Default)]
pub struct Graph {
    controls: Controls,
}

impl shader::Program<Message> for Graph {
    type State = ();
    type Primitive = FragmentShaderPrimitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Self::Primitive {
        FragmentShaderPrimitive::new(self.controls)
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: shader::Event,
        bounds: Rectangle,
        cursor: Cursor,
        _shell: &mut Shell<'_, Message>,
    ) -> (Status, Option<Message>) {
        if let shader::Event::Mouse(mouse::Event::WheelScrolled { delta }) = event {
            if let Some(pos) = cursor.position_in(bounds) {
                let pos = Vec2::new(pos.x, pos.y);
                let delta = match delta {
                    mouse::ScrollDelta::Lines { y, .. } => y,
                    mouse::ScrollDelta::Pixels { y, .. } => y,
                };
                return (
                    Status::Captured,
                    Some(Message::ZoomDelta(pos, bounds, delta)),
                );
            }
        }

        (Status::Ignored, None)
    }
}
