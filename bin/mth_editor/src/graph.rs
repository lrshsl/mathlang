use std::sync::{Arc, Mutex};

use glam::{dvec2, DVec2};
use iced::{
    mouse,
    widget::{shader, Action},
    Event, Rectangle,
};

use graph_canvas::{controls::Controls, FragmentShaderPrimitive, N_INSTRUCTIONS, N_PLOTS};
use mth_common::{ops::Instruction, plot_desc::PlotDesc};

use crate::message::Message;

impl Default for Graph {
    fn default() -> Self {
        Self {
            controls: Controls::default(),
            instructions: Arc::new(Mutex::new([Instruction::default(); N_INSTRUCTIONS])),
            plot_desc: [PlotDesc::default(); N_PLOTS],
            instructions_dirty: false,
        }
    }
}

pub struct Graph {
    pub controls: Controls,
    pub instructions: Arc<Mutex<[Instruction; N_INSTRUCTIONS]>>,
    pub plot_desc: [PlotDesc; N_PLOTS],
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
            self.plot_desc,
            self.instructions_dirty,
        )
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        // Zooming
        if let Event::Mouse(mouse::Event::WheelScrolled { delta }) = event {
            if let Some(pos) = cursor.position_in(bounds) {
                let pos = DVec2::new(pos.x.into(), pos.y.into());
                let delta = match delta {
                    mouse::ScrollDelta::Lines { y, .. } => y,
                    mouse::ScrollDelta::Pixels { y, .. } => y,
                };
                return Some(
                    Action::publish(Message::ZoomDelta(pos, bounds, *delta as f64)).and_capture(),
                );
            }
        }

        // Panning
        match state {
            (false, _) => match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    if let Some(pos) = cursor.position_over(bounds) {
                        *state = (true, dvec2(pos.x.into(), pos.y.into()));
                        return Some(Action::capture());
                    }
                }
                _ => {}
            },
            (true, prev_pos) => match event {
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    (*state).0 = false;
                }
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    let pos = DVec2::new(position.x.into(), position.y.into());
                    let delta = pos - *prev_pos;
                    *state = (true, pos);
                    return Some(Action::publish(Message::PanningDelta(delta)).and_capture());
                }
                _ => {}
            },
        }

        None
    }
}
