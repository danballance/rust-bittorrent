use crate::bencode::types::{BencodeKind, BencodeState, Context};

pub struct CharacterProcessor {}

impl CharacterProcessor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn next_character(&self, character: char, context: Context) -> Result<Context, String> {
        let kind = if context.state == Some(BencodeState::End)
            || (context.kind == Some(BencodeKind::List)
                && context.state == Some(BencodeState::Start))
            || (context.kind == Some(BencodeKind::Dictionary)
                && context.state == Some(BencodeState::Start))
        {
            None
        } else {
            context.kind.as_ref()
        };

        match kind {
            None => self._no_kind(character, context),
            Some(BencodeKind::Integer) => self._integer_kind(character, context),
            Some(BencodeKind::List) => self._no_kind(character, context),
            Some(BencodeKind::String) => self._string_kind(character, context),
            _ => Err("Unable to match on context.current_kind".into()),
        }
    }

    fn _no_kind(&self, character: char, mut ctxt: Context) -> Result<Context, String> {
        match character {
            'd' => {
                ctxt.create_object()?;
                ctxt.clear_type();
                (ctxt.character, ctxt.kind, ctxt.state) = (
                    Some('d'),
                    Some(BencodeKind::Dictionary),
                    Some(BencodeState::Start),
                );
                ctxt.open_containers.push(BencodeKind::Dictionary);
                Ok(ctxt)
            }
            'e' => {
                // when there's no Kind set, 'e' is closing a container
                let mut open_containers = ctxt.open_containers.clone();
                let open_container = open_containers
                    .pop()
                    .ok_or_else(|| "Unexpected 'e': no open container to close".to_string())?;
                match open_container {
                    BencodeKind::List => (),
                    BencodeKind::Dictionary => (),
                    _ => return Err("open_container was not a List or a Dictionary".to_string()),
                }
                ctxt.end_nested_value();
                ctxt.clear_type();
                (ctxt.character, ctxt.kind, ctxt.state, ctxt.open_containers) = (
                    Some('e'),
                    Some(open_container),
                    Some(BencodeState::End),
                    open_containers,
                );
                Ok(ctxt)
            }
            'i' => {
                (ctxt.character, ctxt.kind, ctxt.state) = (
                    Some('i'),
                    Some(BencodeKind::Integer),
                    Some(BencodeState::Start),
                );
                Ok(ctxt)
            }
            'l' => {
                ctxt.create_array()?;
                ctxt.clear_type();
                (ctxt.character, ctxt.kind, ctxt.state) = (
                    Some('l'),
                    Some(BencodeKind::List),
                    Some(BencodeState::Start),
                );
                ctxt.open_containers.push(BencodeKind::List);
                Ok(ctxt)
            }
            c if c.is_ascii_digit() || c == '-' => {
                ctxt.meta_chars.push(c);
                (ctxt.character, ctxt.kind, ctxt.state) =
                    (Some(c), Some(BencodeKind::String), Some(BencodeState::Meta));
                Ok(ctxt)
            }
            _ => Err("Unhandled encoded value".into()),
        }
    }

    fn _integer_kind(&self, character: char, mut ctxt: Context) -> Result<Context, String> {
        match character {
            'e' => {
                let integer = ctxt
                    .data_chars
                    .parse::<isize>()
                    .map_err(|_| "invalid integer".to_string())?;

                let serde_integer = serde_json::Value::Number(integer.into());

                ctxt.update_value(serde_integer)?;
                ctxt.clear_type();
                (ctxt.character, ctxt.kind, ctxt.state) = (
                    Some('e'),
                    Some(BencodeKind::Integer),
                    Some(BencodeState::End),
                );
                Ok(ctxt)
            }
            c if c.is_ascii_digit() || c == '-' => {
                ctxt.data_chars.push(character);
                (ctxt.character, ctxt.kind, ctxt.state) = (
                    Some(c),
                    Some(BencodeKind::Integer),
                    Some(BencodeState::Data),
                );
                Ok(ctxt)
            }
            _ => Err("".into()),
        }
    }

    fn _string_kind(&self, character: char, mut ctxt: Context) -> Result<Context, String> {
        match character {
            c if ctxt.character == Some(':') || ctxt.state == Some(BencodeState::Data) => {
                ctxt.data_chars.push(c);
                if ctxt.data_chars.len() == ctxt.value_length {
                    let serde_string = serde_json::Value::String(ctxt.data_chars.clone());
                    ctxt.update_value(serde_string)?;
                    ctxt.clear_type();
                    ctxt.state = Some(BencodeState::End);
                } else {
                    ctxt.state = Some(BencodeState::Data);
                }
                (ctxt.character, ctxt.kind) = (Some(c), Some(BencodeKind::String));
                Ok(ctxt)
            }
            ':' if ctxt.state == Some(BencodeState::Meta) => {
                // resolve string length from meta chars
                ctxt.value_length = ctxt
                    .meta_chars
                    .parse::<usize>()
                    .map_err(|_| format!("Invalid string length: {}", ctxt.meta_chars))?;
                // reached end of string (zero length string)
                if ctxt.value_length == 0 {
                    let serde_string = serde_json::Value::String(ctxt.data_chars.clone());
                    ctxt.update_value(serde_string)?;
                    ctxt.clear_type();
                    ctxt.state = Some(BencodeState::End);
                } else {
                    ctxt.state = Some(BencodeState::Meta);
                }
                (ctxt.character, ctxt.kind) = (Some(':'), Some(BencodeKind::String));
                Ok(ctxt)
            }
            c if c.is_ascii_digit() => {
                ctxt.meta_chars.push(c);
                (ctxt.character, ctxt.kind, ctxt.state) =
                    (Some(c), Some(BencodeKind::String), Some(BencodeState::Meta));
                Ok(ctxt)
            }
            other => Err(format!(
                "Unable to match character '{:?}' in _string_kind CharacterProcessor.",
                other
            )),
        }
    }
}
