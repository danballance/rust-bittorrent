use crate::bencode::types::{BencodeKind, BencodeState, Context};

pub(crate) struct DictionaryHandler;

impl DictionaryHandler {
    pub(crate) fn start(mut ctxt: Context) -> Result<Context, String> {
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

    pub(crate) fn end(
        mut ctxt: Context,
        open_containers: Vec<BencodeKind>,
    ) -> Result<Context, String> {
        ctxt.end_nested_value();
        ctxt.clear_type();
        (ctxt.character, ctxt.kind, ctxt.state, ctxt.open_containers) = (
            Some('e'),
            Some(BencodeKind::Dictionary),
            Some(BencodeState::End),
            open_containers,
        );
        Ok(ctxt)
    }
}

pub(crate) struct IntegerHandler;

impl IntegerHandler {
    pub(crate) fn start(mut ctxt: Context) -> Result<Context, String> {
        (ctxt.character, ctxt.kind, ctxt.state) = (
            Some('i'),
            Some(BencodeKind::Integer),
            Some(BencodeState::Start),
        );
        Ok(ctxt)
    }

    pub(crate) fn data(mut ctxt: Context, character: char) -> Result<Context, String> {
        ctxt.data_chars.push(character);
        (ctxt.character, ctxt.kind, ctxt.state) = (
            Some(character),
            Some(BencodeKind::Integer),
            Some(BencodeState::Data),
        );
        Ok(ctxt)
    }

    pub(crate) fn end(mut ctxt: Context) -> Result<Context, String> {
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
}

pub(crate) struct ListHandler;

impl ListHandler {
    pub(crate) fn start(mut ctxt: Context) -> Result<Context, String> {
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

    pub(crate) fn end(
        mut ctxt: Context,
        open_containers: Vec<BencodeKind>,
    ) -> Result<Context, String> {
        ctxt.end_nested_value();
        ctxt.clear_type();
        (ctxt.character, ctxt.kind, ctxt.state, ctxt.open_containers) = (
            Some('e'),
            Some(BencodeKind::List),
            Some(BencodeState::End),
            open_containers,
        );
        Ok(ctxt)
    }
}

pub(crate) struct StringHandler;

impl StringHandler {
    pub(crate) fn data(mut ctxt: Context, character: char) -> Result<Context, String> {
        ctxt.data_chars.push(character);
        if ctxt.data_chars.len() == ctxt.value_length {
            let serde_string = serde_json::Value::String(ctxt.data_chars.clone());
            ctxt.update_value(serde_string)?;
            ctxt.clear_type();
            ctxt.state = Some(BencodeState::End);
        } else {
            ctxt.state = Some(BencodeState::Data);
        }
        (ctxt.character, ctxt.kind) = (Some(character), Some(BencodeKind::String));
        Ok(ctxt)
    }

    pub(crate) fn meta(mut ctxt: Context, character: char) -> Result<Context, String> {
        if character == ':' {
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
        } else {
            ctxt.meta_chars.push(character);
            (ctxt.character, ctxt.kind, ctxt.state) = (
                Some(character),
                Some(BencodeKind::String),
                Some(BencodeState::Meta),
            );
        }
        Ok(ctxt)
    }
}
