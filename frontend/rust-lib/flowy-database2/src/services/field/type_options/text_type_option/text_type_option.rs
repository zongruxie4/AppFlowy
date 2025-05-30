use collab::util::AnyMapExt;
use std::cmp::Ordering;

use collab_database::fields::Field;
use collab_database::fields::text_type_option::RichTextTypeOption;
use collab_database::rows::{Cell, new_cell_builder};
use collab_database::template::util::ToCellString;
use flowy_error::{FlowyError, FlowyResult};

use crate::entities::{FieldType, TextFilterPB};
use crate::services::cell::{CellDataChangeset, CellDataDecoder, stringify_cell};
use crate::services::field::type_options::util::ProtobufStr;
use crate::services::field::{
  CELL_DATA, CellDataProtobufEncoder, TypeOption, TypeOptionCellData, TypeOptionCellDataCompare,
  TypeOptionCellDataFilter, TypeOptionTransform,
};
use crate::services::sort::SortCondition;

impl TypeOption for RichTextTypeOption {
  type CellData = StringCellData;
  type CellChangeset = String;
  type CellProtobufType = ProtobufStr;
  type CellFilter = TextFilterPB;
}

impl TypeOptionTransform for RichTextTypeOption {}

impl CellDataProtobufEncoder for RichTextTypeOption {
  fn protobuf_encode(
    &self,
    cell_data: <Self as TypeOption>::CellData,
  ) -> <Self as TypeOption>::CellProtobufType {
    ProtobufStr::from(cell_data.0)
  }
}

impl CellDataDecoder for RichTextTypeOption {
  fn decode_cell_with_transform(
    &self,
    cell: &Cell,
    from_field_type: FieldType,
    field: &Field,
  ) -> Option<<Self as TypeOption>::CellData> {
    match from_field_type {
      FieldType::RichText
      | FieldType::Number
      | FieldType::DateTime
      | FieldType::SingleSelect
      | FieldType::MultiSelect
      | FieldType::Checkbox
      | FieldType::URL
      | FieldType::Summary
      | FieldType::Translate
      | FieldType::Media
      | FieldType::Time => Some(StringCellData::from(stringify_cell(cell, field))),
      FieldType::Checklist
      | FieldType::LastEditedTime
      | FieldType::CreatedTime
      | FieldType::Relation => None,
    }
  }

  fn stringify_cell_data(&self, cell_data: <Self as TypeOption>::CellData) -> String {
    cell_data.to_string()
  }
}

impl CellDataChangeset for RichTextTypeOption {
  fn apply_changeset(
    &self,
    changeset: <Self as TypeOption>::CellChangeset,
    _cell: Option<Cell>,
  ) -> FlowyResult<(Cell, <Self as TypeOption>::CellData)> {
    if changeset.len() > 10000 {
      Err(
        FlowyError::text_too_long()
          .with_context("The len of the text should not be more than 10000"),
      )
    } else {
      let text_cell_data = StringCellData(changeset);
      Ok((text_cell_data.clone().into(), text_cell_data))
    }
  }
}

impl TypeOptionCellDataFilter for RichTextTypeOption {
  fn apply_filter(
    &self,
    filter: &<Self as TypeOption>::CellFilter,
    cell_data: &<Self as TypeOption>::CellData,
  ) -> bool {
    filter.is_visible(cell_data)
  }
}

impl TypeOptionCellDataCompare for RichTextTypeOption {
  fn apply_cmp(
    &self,
    cell_data: &<Self as TypeOption>::CellData,
    other_cell_data: &<Self as TypeOption>::CellData,
    sort_condition: SortCondition,
  ) -> Ordering {
    match (cell_data.is_cell_empty(), other_cell_data.is_cell_empty()) {
      (true, true) => Ordering::Equal,
      (true, false) => Ordering::Greater,
      (false, true) => Ordering::Less,
      (false, false) => {
        let order = cell_data.0.cmp(&other_cell_data.0);
        sort_condition.evaluate_order(order)
      },
    }
  }
}

#[derive(Default, Debug, Clone)]
pub struct StringCellData(pub String);
impl StringCellData {
  pub fn into_inner(self) -> String {
    self.0
  }
}
impl std::ops::Deref for StringCellData {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl TypeOptionCellData for StringCellData {
  fn is_cell_empty(&self) -> bool {
    self.0.is_empty()
  }
}

impl From<&Cell> for StringCellData {
  fn from(cell: &Cell) -> Self {
    Self(cell.get_as(CELL_DATA).unwrap_or_default())
  }
}

impl From<StringCellData> for Cell {
  fn from(data: StringCellData) -> Self {
    let mut cell = new_cell_builder(FieldType::RichText);
    cell.insert(CELL_DATA.into(), data.0.into());
    cell
  }
}

impl std::ops::DerefMut for StringCellData {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl std::convert::From<String> for StringCellData {
  fn from(s: String) -> Self {
    Self(s)
  }
}

impl ToCellString for StringCellData {
  fn to_cell_string(&self) -> String {
    self.0.clone()
  }
}

impl std::convert::From<StringCellData> for String {
  fn from(value: StringCellData) -> Self {
    value.0
  }
}

impl std::convert::From<&str> for StringCellData {
  fn from(s: &str) -> Self {
    Self(s.to_owned())
  }
}

impl AsRef<str> for StringCellData {
  fn as_ref(&self) -> &str {
    self.0.as_str()
  }
}
