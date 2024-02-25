use crate::versions::ChartExtKindValuesUi;
use crate::versions::ChartExtVersionV1Beta1;
use crate::UiSchemaCollections;
use crate::UiSchemaInputError;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::HashMap;
use strum::{EnumDiscriminants, EnumString};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(untagged)]
pub enum UiSchema {
    V1Beta1(UiSchemaV1Beta1),
    V0(UiSchemaV0),
}

impl UiSchema {
    pub fn get_inputs(&self) -> &[UiSchemaInput] {
        match self {
            Self::V1Beta1(v1) => &v1.inner.inputs,
            Self::V0(v0) => &v0.inputs,
        }
    }

    pub fn get_outputs(&self) -> &UiSchemaOutputs {
        match self {
            Self::V1Beta1(v1) => &v1.inner.outputs,
            Self::V0(v0) => &v0.outputs,
        }
    }

    pub fn is_collection_in_inputs<C>(
        &self,
        inputs: &serde_json::Value,
        collection: &C,
        id: &str,
    ) -> bool
    where
        C: UiSchemaCollections,
    {
        let collection_value = serde_json::to_value(collection).unwrap();
        self.get_inputs().iter().any(|input| {
            let used_collection = match &input.input_type.single_type {
                UiSchemaInputSingleType::CollectionSelect { collection } => Some(collection),
                _ => None,
            };
            used_collection == Some(&collection_value) && inputs[&input.id] == id
        })
    }

    pub async fn get_values<C>(
        &self,
        env_id: Uuid,
        inputs: &serde_json::Value,
    ) -> Result<Map, UiSchemaInputError<C::Error>>
    where
        C: UiSchemaCollections,
    {
        let schema_inputs = self.get_inputs();
        let mut values = Map::new();
        for output in self.get_outputs().values.iter() {
            output
                .resolve_into::<C>(env_id, schema_inputs, inputs, &mut values)
                .await?;
        }
        Ok(values)
    }

    pub async fn get_secrets<C>(
        &self,
        env_id: Uuid,
        inputs: &serde_json::Value,
    ) -> Result<Vec<RenderedSecret>, UiSchemaInputError<C::Error>>
    where
        C: UiSchemaCollections,
    {
        let mut result: Vec<RenderedSecret> = Vec::new();
        let schema_inputs = self.get_inputs();
        for (secret_name, attrs_schema) in self.get_outputs().secrets.0.iter() {
            let mut attrs: BTreeMap<String, String> = Default::default();
            for (key, attr_schema) in attrs_schema.iter() {
                let value = match attr_schema
                    .resolve::<C>(env_id, schema_inputs, inputs)
                    .await
                {
                    Ok(x) => x,
                    Err(UiSchemaInputError::OptionalInputMissing(_)) => continue,
                    Err(other_err) => return Err(other_err),
                };
                attrs.insert(
                    key.clone(),
                    value
                        .as_str()
                        .map_or_else(|| value.to_string(), |v| v.to_owned()),
                );
            }

            if !attrs.is_empty() {
                result.push(RenderedSecret {
                    name: secret_name.to_owned(),
                    attrs,
                })
            }
        }
        Ok(result)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(deny_unknown_fields)]
pub struct UiSchemaV0 {
    pub inputs: Vec<UiSchemaInput>,
    #[serde(default)]
    pub outputs: UiSchemaOutputs,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct UiSchemaV1Beta1 {
    pub api_version: ChartExtVersionV1Beta1,
    pub kind: ChartExtKindValuesUi,
    #[serde(flatten)]
    pub inner: UiSchemaV0,
}

#[derive(Clone, Debug, Deserialize, Serialize, EnumString, EnumDiscriminants)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[strum_discriminants(derive(EnumString, strum::Display))]
#[strum_discriminants(strum(ascii_case_insensitive))]
pub enum UiSchemaInputSingleType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "number")]
    Number,
    CollectionSelect {
        collection: serde_json::Value,
    },
    RadioSelect,
    DaysAndHour,
    Checkbox,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(try_from = "SerializedUiSchemaInputType")]
#[serde(into = "SerializedUiSchemaInputType")]
pub struct UiSchemaInputType {
    pub single_type: UiSchemaInputSingleType,
    pub is_array: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SerializedUiSchemaInputType {
    r#type: String,
    item_type: Option<String>,
    collection: Option<serde_json::Value>,
}

impl TryFrom<SerializedUiSchemaInputType> for UiSchemaInputType {
    type Error = strum::ParseError;

    fn try_from(s: SerializedUiSchemaInputType) -> Result<Self, Self::Error> {
        let is_array = s.r#type == "array";
        let single_type_disc = if is_array {
            s.item_type.ok_or(strum::ParseError::VariantNotFound)?
        } else {
            s.r#type
        };

        let disc: UiSchemaInputSingleTypeDiscriminants = single_type_disc.parse()?;
        let single_type = match disc {
            UiSchemaInputSingleTypeDiscriminants::CollectionSelect => {
                UiSchemaInputSingleType::CollectionSelect {
                    collection: s.collection.ok_or(strum::ParseError::VariantNotFound)?,
                }
            }
            UiSchemaInputSingleTypeDiscriminants::Text => UiSchemaInputSingleType::Text,
            UiSchemaInputSingleTypeDiscriminants::Number => UiSchemaInputSingleType::Number,
            UiSchemaInputSingleTypeDiscriminants::RadioSelect => {
                UiSchemaInputSingleType::RadioSelect
            }
            UiSchemaInputSingleTypeDiscriminants::Checkbox => UiSchemaInputSingleType::Checkbox,
            UiSchemaInputSingleTypeDiscriminants::DaysAndHour => {
                UiSchemaInputSingleType::DaysAndHour
            }
        };
        Ok(Self {
            single_type,
            is_array,
        })
    }
}

impl From<UiSchemaInputType> for SerializedUiSchemaInputType {
    fn from(input_type: UiSchemaInputType) -> Self {
        let (r#type, collection) = match input_type.single_type {
            UiSchemaInputSingleType::Text => ("text".to_owned(), None),
            UiSchemaInputSingleType::Number => ("number".to_owned(), None),
            UiSchemaInputSingleType::CollectionSelect { collection } => {
                ("CollectionSelect".to_owned(), Some(collection))
            }
            UiSchemaInputSingleType::RadioSelect => ("RadioSelect".to_owned(), None),
            UiSchemaInputSingleType::DaysAndHour => ("DaysAndHour".to_owned(), None),
            UiSchemaInputSingleType::Checkbox => ("Checkbox".to_owned(), None),
        };
        let (r#type, item_type) = if input_type.is_array {
            ("array".to_owned(), Some(r#type))
        } else {
            (r#type, None)
        };
        Self {
            r#type,
            item_type,
            collection,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UiSchemaFieldValuePair {
    field: String,
    value: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct UiSchemaInput {
    pub id: String,
    #[serde(flatten)]
    #[cfg_attr(feature = "utoipa", schema(value_type = SerializedUiSchemaInputType))]
    pub input_type: UiSchemaInputType, // Parsed from actual fields: type, item_type and collection, see SerializedUiSchemaInputType
    label: String,
    #[serde(default)]
    initial_value: Option<serde_json::Value>,
    #[serde(default)]
    help_text: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub sensitive: bool,
    #[serde(default)]
    pub options: Option<Vec<UiSchemaInputFieldOption>>,
    #[serde(default)]
    show_if_all: Option<Vec<UiSchemaFieldValuePair>>,
    #[serde(default)]
    show_if: Option<serde_json::Value>,
    #[serde(default)]
    filters: Option<Vec<UiSchemaInputFieldValue>>,
    #[serde(default)]
    minimum: Option<Decimal>,
    #[serde(default)]
    maximum: Option<Decimal>,
    #[serde(default)]
    step: Option<Decimal>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct UiSchemaInputFieldOption {
    pub value: serde_json::Value,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub help_text: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UiSchemaInputFieldValue {
    pub field: String,
    pub value: serde_json::Value,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UiSchemaOutputSecrets(pub HashMap<String, HashMap<String, UiSchemaInputRef>>);

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UiSchemaOutputs {
    pub values: Vec<UiSchemaOutputValue>,
    #[serde(default)]
    pub secrets: UiSchemaOutputSecrets,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UiSchemaInputRef {
    FieldValue(UiSchemaInputRefField),
    FieldProperty(UiSchemaInputRefProperty),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UiSchemaInputRefField {
    pub input: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UiSchemaInputRefProperty {
    pub input: String,
    pub property: String,
}

impl UiSchemaInputRef {
    fn get_input_schema<'a, C>(
        input_schema: &'a [UiSchemaInput],
        id: &str,
    ) -> Result<&'a UiSchemaInput, UiSchemaInputError<C::Error>>
    where
        C: UiSchemaCollections,
    {
        input_schema
            .iter()
            .find(|i| i.id == id)
            .ok_or_else(|| UiSchemaInputError::MissingInputSchema(id.to_owned()))
    }

    fn get_input<C>(
        schema: &UiSchemaInput,
        inputs: &serde_json::Value,
        id: &str,
    ) -> Result<serde_json::Value, UiSchemaInputError<C::Error>>
    where
        C: UiSchemaCollections,
    {
        if let Some(show_if) = schema.show_if.as_ref() {
            let res = jsonlogic_rs::apply(show_if, inputs);
            if !matches!(res, Ok(serde_json::Value::Bool(true))) {
                return Err(UiSchemaInputError::OptionalInputMissing(id.to_owned()));
            }
        } else if let Some(show_if_all) = schema.show_if_all.as_ref() {
            if show_if_all
                .iter()
                .any(|fv| inputs.get(&fv.field) != Some(&fv.value))
            {
                return Err(UiSchemaInputError::OptionalInputMissing(id.to_owned()));
            }
        }
        Ok(inputs
            .get(id)
            .ok_or_else(|| {
                if schema.required {
                    UiSchemaInputError::MissingInputValue(id.to_owned())
                } else {
                    UiSchemaInputError::OptionalInputMissing(id.to_owned())
                }
            })?
            .clone())
    }

    pub async fn resolve<C>(
        &self,
        env_id: Uuid,
        input_schema: &[UiSchemaInput],
        inputs: &serde_json::Value,
    ) -> Result<serde_json::Value, UiSchemaInputError<C::Error>>
    where
        C: UiSchemaCollections,
    {
        match self {
            Self::FieldValue(fv) => Self::get_input::<C>(
                Self::get_input_schema::<C>(input_schema, &fv.input)?,
                inputs,
                &fv.input,
            ),
            Self::FieldProperty(fp) => {
                let schema = Self::get_input_schema::<C>(input_schema, &fp.input)?;
                match &schema.input_type.single_type {
                    UiSchemaInputSingleType::CollectionSelect { collection } => {
                        let collections: C = serde_json::from_value(collection.to_owned())
                            .map_err(|err| {
                                UiSchemaInputError::InvalidCollectionName(
                                    collection.to_owned(),
                                    err,
                                )
                            })?;
                        let id_value = Self::get_input::<C>(schema, inputs, &fp.input)?;
                        if schema.input_type.is_array {
                            let id_value_arr = id_value.as_array().ok_or_else(|| {
                                UiSchemaInputError::InputNotStringArray(fp.input.clone())
                            })?;

                            let mut resolved_arr = Vec::new();

                            for id_value in id_value_arr {
                                let id = id_value.as_str().ok_or_else(|| {
                                    UiSchemaInputError::InputNotStringArray(fp.input.clone())
                                })?;
                                let resolved_value =
                                    collections.resolve(env_id, id, &fp.property).await?;
                                resolved_arr.push(resolved_value);
                            }
                            Ok(serde_json::to_value(resolved_arr).unwrap())
                        } else {
                            let id = id_value.as_str().ok_or_else(|| {
                                UiSchemaInputError::InputNotString(fp.input.clone())
                            })?;
                            collections.resolve(env_id, id, &fp.property).await
                        }
                    }
                    _ => Err(UiSchemaInputError::InputNotACollection(fp.input.clone())),
                }
            }
        }
    }
}

type Map = serde_json::Map<String, serde_json::Value>;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UiSchemaOutputValue {
    pub path: Vec<String>,
    pub value: UiSchemaInputRef,
}

pub fn insert_into_map(map: &mut Map, path: &[String], value: serde_json::Value) {
    let mut cur_node = map;
    let mut iter = path.iter().peekable();

    while let Some(part) = iter.next() {
        if iter.peek().is_none() {
            cur_node.insert(part.to_owned(), value);
            return;
        }
        if !cur_node.contains_key(part) {
            cur_node.insert(part.to_owned(), serde_json::Value::Object(Map::new()));
        }
        cur_node = cur_node.get_mut(part).unwrap().as_object_mut().unwrap();
    }
}

impl UiSchemaOutputValue {
    pub async fn resolve_into<C>(
        &self,
        env_id: Uuid,
        input_schema: &[UiSchemaInput],
        inputs: &serde_json::Value,
        outputs: &mut Map,
    ) -> Result<(), UiSchemaInputError<C::Error>>
    where
        C: UiSchemaCollections,
    {
        match self.value.resolve::<C>(env_id, input_schema, inputs).await {
            Ok(value) => {
                insert_into_map_ex(outputs, &self.path, value);
                Ok(())
            }
            Err(UiSchemaInputError::OptionalInputMissing(_)) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

pub struct RenderedSecret {
    pub name: String,
    pub attrs: BTreeMap<String, String>,
}

/// Extends insert_into_map by supporting syntax for path items which are array cell's references.
///
/// A path element of [NUMBER] (with the bracket) will regard current location as an array and will create null
/// items as neccessary up to required.
///
/// A path element of [=] will reference the last existing element of the array (if empty, will create an element)
///
/// A path element of [+] will add an element in the end of the array and refer to it.
/// UiSchemaOutputs's values are resolved in order of appearance, so this creates a predictable array
///
/// Arrays of arrays, maps of arrays, arrays of maps, maps of maps, are all supported.
/// Array sizes are limited to protect from bad input.
///
/// A fair share of examples is present in the test_insert_into_map_ex function below.
///
/// Do note that if you supply the path in a yaml array (as in values-ui.yaml outputs), you'll have to quote around the brackets
/// Examples for values-ui.yaml:
/// outputs:
///   values:
///   - path:
///       - config
///       - selected
///       - "[+]"
///       - id
///     value:
///       FieldProperty:
///         input: conditional_select1
///         property: id
///   - path:
///       - config
///       - selected
///       - "[=]"
///       - name
///     value:
///       FieldProperty:
///         input: conditional_select1
///         property: name
///   - path:
///       - config
///       - selected
///       - "[+]"
///       - id
///     value:
///       FieldProperty:
///         input: conditional_select2
///         property: id
///   - path:
///       - config
///       - selected
///       - "[=]"
///       - name
///     value:
///       FieldProperty:
///         input: conditional_select2
///         property: name
///
/// In the example, we output an array "config.selected", whose items are values with keys "id", and "name"
fn insert_into_map_ex(map: &mut Map, path: &[String], value: serde_json::Value) {
    let mut iter: std::iter::Peekable<std::slice::Iter<'_, String>> = path.iter().peekable();
    if iter.peek().is_some() {
        recursively_insert_into_map(map, &mut iter, value);
    }
}

const MAX_ARRAY_SIZE: usize = 1024;

fn recursively_insert_into_map(
    map: &mut Map,
    iter: &mut std::iter::Peekable<std::slice::Iter<'_, String>>,
    value: serde_json::Value,
) {
    let part = iter.next().unwrap(); // Safe to unwrap since we alwasy peek before we next
    let next_part = iter.peek();

    match next_part {
        None => {
            map.insert(part.to_owned(), value);
        }
        Some(next_part) => {
            if map.contains_key(part) {
                match map.get_mut(part).unwrap() {
                    serde_json::Value::Array(inner_vec) => {
                        recursively_insert_into_vec(inner_vec, iter, value);
                    }
                    serde_json::Value::Object(inner_map) => {
                        recursively_insert_into_map(inner_map, iter, value);
                    }
                    _ => (),
                }
            } else if next_part.starts_with('[') && next_part.ends_with(']') {
                let inner_vec = map
                    .entry(part.to_owned())
                    .or_insert(serde_json::Value::Array(Vec::new()))
                    .as_array_mut()
                    .unwrap();
                recursively_insert_into_vec(inner_vec, iter, value);
            } else {
                let inner_map = map
                    .entry(part.to_owned())
                    .or_insert(serde_json::Value::Object(Map::new()))
                    .as_object_mut()
                    .unwrap();
                recursively_insert_into_map(inner_map, iter, value);
            }
        }
    };
}

fn recursively_insert_into_vec(
    vec: &mut Vec<serde_json::Value>,
    iter: &mut std::iter::Peekable<std::slice::Iter<'_, String>>,
    value: serde_json::Value,
) {
    let part = iter.next().unwrap(); // Safe to unwrap since we alwasy peek before we next

    let inner_part =
        if let Some(inner_part) = part.strip_prefix('[').and_then(|x| x.strip_suffix(']')) {
            inner_part
        } else {
            return;
        };

    let cell = if let Ok(number) = inner_part.parse::<usize>() {
        // Gotta protect ourselves from bad input
        if number >= MAX_ARRAY_SIZE {
            return;
        }
        if vec.len() < number + 1 {
            vec.resize(number + 1, Default::default());
        }
        &mut vec[number]
    } else if inner_part == "+" {
        if vec.len() >= MAX_ARRAY_SIZE {
            return;
        }

        vec.push(Default::default());
        vec.last_mut().unwrap()
    } else if inner_part == "=" {
        if vec.is_empty() {
            vec.push(Default::default());
        }
        vec.last_mut().unwrap()
    } else {
        return;
    };

    let next_part = iter.peek();

    match next_part {
        None => {
            *cell = value;
        }
        Some(next_part) => match cell {
            serde_json::Value::Object(inner_map) => {
                recursively_insert_into_map(inner_map, iter, value);
            }
            serde_json::Value::Array(inner_vec) => {
                recursively_insert_into_vec(inner_vec, iter, value);
            }
            serde_json::Value::Null => {
                if next_part.starts_with('[') && next_part.ends_with(']') {
                    *cell = serde_json::Value::Array(Vec::new());
                    recursively_insert_into_vec(cell.as_array_mut().unwrap(), iter, value);
                } else {
                    *cell = serde_json::Value::Object(Map::new());
                    recursively_insert_into_map(cell.as_object_mut().unwrap(), iter, value);
                }
            }
            _ => (),
        },
    }
}

#[cfg(test)]
mod test {
    use super::insert_into_map_ex;

    fn str_arr(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_insert_into_map_ex() {
        let mut value = serde_json::json!({});

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["v"]),
            "v_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["m", "k1"]),
            "m_k1_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["m", "k2"]),
            "m_k2_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["m", "m", "k1"]),
            "m_m_k1_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a1", "[=]"]),
            "a1_0_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a1", "[+]"]),
            "a1_1_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a2", "[=]", "k1"]),
            "a2_0_k1_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a2", "[=]", "k2"]),
            "a2_0_k2_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a2", "[+]", "k1"]),
            "a2_1_k1_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a2", "[+]", "k1"]),
            "a2_2_k1_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a2", "[1]", "k2"]),
            "a2_1_k2_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a3", "[=]", "[=]"]),
            "a3_0_0_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a3", "[=]", "[+]"]),
            "a3_0_1_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a3", "[=]", "[+]", "k1"]),
            "a3_0_2_k1_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a3", "[=]", "[=]", "k2"]),
            "a3_0_2_k2_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a3", "[+]", "[=]", "k1"]),
            "a3_1_0_k1_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a3", "[=]", "[2]"]),
            "a3_1_2_val".into(),
        );

        insert_into_map_ex(
            value.as_object_mut().unwrap(),
            &str_arr(&["a3", "[0]", "[=]", "k3"]),
            "a3_0_2_k3_val".into(),
        );

        let expected = serde_json::json!({
            "v": "v_val",
            "m": {
                "k1": "m_k1_val",
                "k2": "m_k2_val",
                "m": {
                    "k1" : "m_m_k1_val"
                }
            },
            "a1": [
                "a1_0_val",
                "a1_1_val"
            ],
            "a2": [
                {
                    "k1": "a2_0_k1_val",
                    "k2": "a2_0_k2_val"
                },
                {
                    "k1": "a2_1_k1_val",
                    "k2": "a2_1_k2_val"
                },
                {
                    "k1": "a2_2_k1_val",
                }
            ],
            "a3": [
                [
                    "a3_0_0_val",
                    "a3_0_1_val",
                    {
                        "k1": "a3_0_2_k1_val",
                        "k2": "a3_0_2_k2_val",
                        "k3": "a3_0_2_k3_val",
                    }
                ],
                [
                    {
                        "k1": "a3_1_0_k1_val"
                    },
                    null,
                    "a3_1_2_val"
                ]
            ]
        });

        assert_eq!(value, expected)
    }
}
