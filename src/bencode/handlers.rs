use crate::bencode::types::{BencodeKind, BencodeState, Context};

pub(crate) struct DictionaryHandler;

impl DictionaryHandler {
    pub(crate) fn Start(mut ctxt: Context) -> Result<Context, String> {
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

    pub(crate) fn End(
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
    pub(crate) fn Start(mut ctxt: Context) -> Result<Context, String> {
        (ctxt.character, ctxt.kind, ctxt.state) = (
            Some('i'),
            Some(BencodeKind::Integer),
            Some(BencodeState::Start),
        );
        Ok(ctxt)
    }

    pub(crate) fn Data(mut ctxt: Context, character: char) -> Result<Context, String> {
        ctxt.data_chars.push(character);
        (ctxt.character, ctxt.kind, ctxt.state) = (
            Some(character),
            Some(BencodeKind::Integer),
            Some(BencodeState::Data),
        );
        Ok(ctxt)
    }

    pub(crate) fn End(mut ctxt: Context) -> Result<Context, String> {
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
    pub(crate) fn Start(mut ctxt: Context) -> Result<Context, String> {
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

    pub(crate) fn End(
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
    pub(crate) fn Meta(mut ctxt: Context, character: char) -> Result<Context, String> {
        ctxt.meta_chars.push(character);
        (ctxt.character, ctxt.kind, ctxt.state) = (
            Some(character),
            Some(BencodeKind::String),
            Some(BencodeState::Meta),
        );
        Ok(ctxt)
    }
}
