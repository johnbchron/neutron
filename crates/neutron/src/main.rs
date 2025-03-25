struct ItemId(ulid::Ulid);

struct Item {
  id:   ItemId,
  deps: DependencySet,
}

enum DependencySet {
  All(Vec<ItemId>),
}

fn main() {}
