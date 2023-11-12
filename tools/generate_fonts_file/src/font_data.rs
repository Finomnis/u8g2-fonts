use miette::{miette, Result};

use crate::u8_compression::write_byte;

#[derive(Debug)]
enum State {
    Padding,
    Data,
    Number,
    Number2(u8),
    Number3(u8),
}

enum Action {
    Continue,
    Transition(State),
    Repeat(State),
    Finished,
}

pub fn consume_font_data<'a>(mut data: &'a [u8], out: &mut Vec<u8>) -> Result<usize> {
    let mut state = State::Padding;

    let mut num_produced = 0;

    let mut produce = |c| {
        write_byte(out, c);
        num_produced += 1;
    };

    loop {
        let next_ch = *data.get(0).ok_or(miette!("Unexpected end of file"))?;

        let action = match (&state, next_ch) {
            (State::Padding, b' ') => Action::Continue,
            (State::Padding, b'\n') => Action::Continue,
            (State::Padding, b'\r') => Action::Continue,
            (State::Padding, b'\t') => Action::Continue,
            (State::Padding, b'"') => Action::Transition(State::Data),
            (State::Padding, b';') => Action::Finished,
            (State::Data, b'"') => Action::Transition(State::Padding),
            (State::Data, b'\\') => Action::Transition(State::Number),
            (State::Data, c) => {
                produce(c);
                Action::Continue
            }

            (State::Number, c) if c >= b'0' && c < b'8' => {
                Action::Transition(State::Number2(c - b'0'))
            }
            (State::Number2(v), c) if c >= b'0' && c < b'8' => {
                Action::Transition(State::Number3(8 * v + (c - b'0')))
            }
            (State::Number3(v), c) if c >= b'0' && c < b'8' => {
                produce(8 * v + (c - b'0'));
                Action::Transition(State::Data)
            }
            (State::Number2(c), _) | (State::Number3(c), _) => {
                produce(*c);
                Action::Repeat(State::Data)
            }

            (state, next_ch) => {
                miette::bail!(
                    "Unexpected character {:?} for state '{:?}'",
                    next_ch as char,
                    state
                )
            }
        };

        match action {
            Action::Continue => {
                data = &data[1..];
            }
            Action::Transition(new_state) => {
                data = &data[1..];
                state = new_state
            }
            Action::Repeat(new_state) => {
                state = new_state;
            }
            Action::Finished => {
                //data = &data[1..];
                break;
            }
        }
    }

    produce(0);
    Ok(num_produced)
}
