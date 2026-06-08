use serde_json::{Map, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BencodeState {
    // 4 phases: Start -> Meta -> Data -> End
    // Maps to parts of a type:
    // 5:hello [][5:][hello][] (String has no Start or End)
    // i108e [i][][108][e] (Integer has no Meta)
    Start,
    Meta,
    Data,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BencodeKind {
    Dictionary,
    Integer,
    List,
    String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context {
    pub(crate) character: Option<char>,
    pub(crate) kind: Option<BencodeKind>,
    pub(crate) state: Option<BencodeState>,
    pub(crate) open_containers: Vec<BencodeKind>,
    pub(crate) data_chars: String,
    pub(crate) meta_chars: String,
    pub(crate) value: Value,
    pub(crate) value_length: usize,
    pub(crate) value_path: Vec<PathItem>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            character: None,
            state: None,
            kind: None,
            open_containers: vec![],
            data_chars: String::new(),
            meta_chars: String::new(),
            value: Value::Null,
            value_length: 0,
            value_path: vec![],
        }
    }

    pub fn update_value(&mut self, value: Value) -> Result<(), String> {
        let current = self.current_value().ok_or("invalid value location")?;
        match current {
            Value::Array(array) => {
                array.push(value.clone());
            }
            Value::Object(obj) => {
                // Check if there is already a key whose value is Value::Null
                // @TODO dupe - refactor
                let null_key = obj.iter().find_map(|(key, map_value)| {
                    if matches!(map_value, Value::Null) {
                        Some(key.clone())
                    } else {
                        None
                    }
                });

                if let Some(key) = null_key {
                    // If there is, insert the new value into the map at that key
                    obj.insert(key, value);
                } else {
                    // If there isn't, ensure the value is a String
                    let string_value = match value {
                        Value::String(string) => string,
                        other => return Err(format!("Value is not a String {:?}", other)),
                    };
                    // Finally insert the string as a key with Value::Null
                    obj.insert(string_value, Value::Null);
                }
            }
            slot => {
                *slot = value;
            }
        }
        Ok(())
    }

    fn current_value(&mut self) -> Option<&mut Value> {
        // start at the root of the value
        let mut current = &mut self.value;
        // iterate through the path to find the current value
        for item in &self.value_path {
            current = match (current, item) {
                (Value::Array(array), PathItem::Index(index)) => array.get_mut(*index)?,
                (Value::Object(object), PathItem::Key(key)) => object.get_mut(key)?,
                _ => return None,
            };
        }
        Some(current)
    }

    /*
     * Create a new array. If the current value is the Null -
     * replace it with a new array. If the current value is an array,
     * push a new array to the array.
     */
    pub(crate) fn create_array(&mut self) -> Result<(), String> {
        let mut new_child_index = None;
        let mut new_child_key: Option<String> = None;
        {
            let current = self.current_value().ok_or("invalid value location")?;
            match current {
                // Already in an array, add a new array to the current array
                Value::Array(array) => {
                    let index = array.len();
                    array.push(Value::Array(vec![]));
                    new_child_index = Some(index);
                }
                Value::Object(obj) => {
                    // Check if there is already a key whose value is Value::Null
                    // @TODO dupe - refactor
                    let null_key = obj.iter().find_map(|(key, map_value)| {
                        if matches!(map_value, Value::Null) {
                            Some(key.clone())
                        } else {
                            None
                        }
                    });

                    if let Some(key) = null_key {
                        // If there is, insert the new value into the map at that key
                        new_child_key = Some(key.clone());
                        obj.insert(key, Value::Array(vec![]));
                    } else {
                        return Err(format!(
                            "Unable to fine NULL value to place new object: {:?}",
                            obj
                        ));
                    }
                }
                // We are at a Null location.
                // Replace this location with an array.
                // The path does not change because this is not a child.
                Value::Null => *current = Value::Array(vec![]),
                other => {
                    return Err(format!("cannot create array at current value: {:?}", other));
                }
            }
        }

        // Now the mutable borrow from current_value_mut() has ended,
        // so we can safely mutate self.value_path.
        if let Some(index) = new_child_index {
            self.value_path.push(PathItem::Index(index));
        }
        if let Some(key) = new_child_key {
            self.value_path.push(PathItem::Key(key));
        }
        Ok(())
    }

    /*
     * Create a new object. If the current value is the Null root -
     * replace it with a new object.
     * If the current value is an array, then add a new object to the array.
     * Objects can't be keys in this schema - can only be a value.
     */
    pub(crate) fn create_object(&mut self) -> Result<(), String> {
        let mut new_child_index: Option<usize> = None;
        let mut new_child_key: Option<String> = None;
        {
            let current = self.current_value().ok_or("invalid value location")?;
            match current {
                // Already in an object, look for a NULL value to insert a new object into
                Value::Object(obj) => {
                    // Check if there is already a key whose value is Value::Null
                    // @TODO dupe - refactor
                    let null_key = obj.iter().find_map(|(key, map_value)| {
                        if matches!(map_value, Value::Null) {
                            Some(key.clone())
                        } else {
                            None
                        }
                    });

                    if let Some(key) = null_key {
                        // If there is, insert the new value into the map at that key
                        new_child_key = Some(key.clone());
                        obj.insert(key, Value::Object(Map::new()));
                    } else {
                        return Err(format!(
                            "Unable to fine NULL value to place new object: {:?}",
                            obj
                        ));
                    }
                }
                // Already in an array, add a new object to the current array
                Value::Array(array) => {
                    let index = array.len();
                    new_child_index = Some(index);
                    array.push(Value::Object(Map::new()));
                }
                // We are at a Null location.
                // The path does not change because this is not a child.
                Value::Null => *current = Value::Object(Map::new()),
                other => {
                    return Err(format!(
                        "cannot create object at current value: {:?}",
                        other
                    ));
                }
            }
        }

        // Now the mutable borrow from current_value_mut() has ended,
        // so we can safely mutate self.value_path.
        if let Some(index) = new_child_index {
            self.value_path.push(PathItem::Index(index));
        }
        if let Some(key) = new_child_key {
            self.value_path.push(PathItem::Key(key));
        }
        Ok(())
    }
    pub(crate) fn end_nested_value(&mut self) {
        self.value_path.pop();
    }

    pub(crate) fn clear_type(&mut self) -> () {
        self.data_chars.clear();
        self.meta_chars.clear();
        self.value_length = 0;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathItem {
    Key(String),
    Index(usize),
}
