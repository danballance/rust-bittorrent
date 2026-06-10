use crate::bencode::handlers::{DictionaryHandler, IntegerHandler, ListHandler, StringHandler};
use crate::bencode::types::{BencodeKind, BencodeState, Context};

pub struct CharacterProcessor {}

impl CharacterProcessor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn next_character(&self, character: char, context: Context) -> Result<Context, String> {
        let active_kind = get_active_kind(&context);
        match active_kind {
            None => self._no_kind(character, context),
            Some(BencodeKind::Integer) => self._integer_kind(character, context),
            Some(BencodeKind::List) => self._no_kind(character, context),
            Some(BencodeKind::String) => self._string_kind(character, context),
            _ => Err("Unable to match on context.current_kind".into()),
        }
    }

    fn _no_kind(&self, character: char, mut ctxt: Context) -> Result<Context, String> {
        match character {
            'd' => DictionaryHandler::start(ctxt),
            'e' => {
                // when no Kind is set, 'e' is closing a container - List or Dictionary
                let (open_containers, open_container) = ctxt.pop_next_container()?;
                match open_container {
                    BencodeKind::List => ListHandler::end(ctxt, open_containers),
                    BencodeKind::Dictionary => DictionaryHandler::end(ctxt, open_containers),
                    _ => return Err("open_container was not a List or a Dictionary".to_string()),
                }
            }
            'i' => IntegerHandler::start(ctxt),
            'l' => ListHandler::start(ctxt),
            c if c.is_ascii_digit() || c == '-' => StringHandler::meta(ctxt, c),
            _ => Err("Unhandled encoded value".into()),
        }
    }

    fn _integer_kind(&self, character: char, mut ctxt: Context) -> Result<Context, String> {
        match character {
            'e' => IntegerHandler::end(ctxt),
            c if c.is_ascii_digit() || c == '-' => IntegerHandler::data(ctxt, c),
            _ => Err("Unable to match a character for _integer_kind".into()),
        }
    }

    fn _string_kind(&self, character: char, mut ctxt: Context) -> Result<Context, String> {
        match character {
            c if ctxt.character == Some(':') || ctxt.state == Some(BencodeState::Data) => {
                StringHandler::data(ctxt, c)
            }
            ':' if ctxt.state == Some(BencodeState::Meta) => StringHandler::meta(ctxt, ':'),
            c if c.is_ascii_digit() => StringHandler::meta(ctxt, c),
            other => Err(format!(
                "Unable to match character '{:?}' in _string_kind CharacterProcessor.",
                other
            )),
        }
    }
}

fn get_active_kind(context: &Context) -> Option<BencodeKind> {
    let kind_has_ended = context.state == Some(BencodeState::End);
    let just_opened_container = matches!(
        (context.kind, context.state),
        (
            Some(BencodeKind::List | BencodeKind::Dictionary),
            Some(BencodeState::Start)
        )
    );
    if kind_has_ended || just_opened_container {
        None
    } else {
        context.kind
    }
}
