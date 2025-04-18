use std::{fmt, time::SystemTime};

use nanorand::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ItemId(ulid::Ulid);

impl fmt::Debug for ItemId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("ItemId").field(&self.0.to_string()).finish()
  }
}

impl ItemId {
  pub fn new() -> Self {
    ItemId(ulid::Ulid::from_parts(
      SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("non-monotonic clock")
        .as_millis()
        .try_into()
        .expect("millis since unix epoch doesn't fit into u64"),
      nanorand::tls_rng().generate::<u128>(),
    ))
  }
  pub fn inner(&self) -> &ulid::Ulid { &self.0 }
}

impl Default for ItemId {
  fn default() -> Self { Self::new() }
}

pub struct Item {
  id:   ItemId,
  meta: ItemMetadata,
  data: ItemType,
}

impl Item {
  pub fn new(meta: ItemMetadata, data: ItemType) -> Self {
    Item {
      id: ItemId::new(),
      meta,
      data,
    }
  }

  pub fn id(&self) -> ItemId { self.id }
  pub fn meta(&self) -> &ItemMetadata { &self.meta }
  pub fn data(&self) -> &ItemType { &self.data }
}

pub struct ItemMetadata {
  pub content:     String,
  pub description: Option<String>,
}

pub enum ItemType {
  Condition(Condition),
  Task(Task),
}

pub enum Condition {
  Atomic { satisfied: bool },
  Compound,
}

pub struct Task {
  pub completed:  bool,
  pub dependence: Dependence,
}

pub enum Dependence {
  None,
  Single(ItemId),
  All(Vec<Dependence>),
  Any(Vec<Dependence>),
  Not(Box<Dependence>),
}
