# Style Guide

For the most part, the defacto standards of the rust team / community should be followed, this is just a supplement of rules that while should be common, might not be a defacto standard.

## Use Standard Formatting

All code should be formatted with the standard formatting tool

## Struct Creation

While we should follow the `Struct::new()` pattern, we should also be following the struct builder pattern which means using a struct to pass in the data to `new()` for the creation of the structure instance. The build structure should:

- omit any field that require internal generation
- use `Option<>` for any optional fields
- set defaults when an optional field is given `None`

```rs
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub item_type: ItemType,
    pub equipment_type: EquipmentType,
    pub maximum_quantity: ItemQuantity,
}

pub struct ItemBuilder {
    pub name: String,
    pub item_type: Option<ItemType>,
    pub equipment_type: Option<EquipmentType>,
    pub maximum_quantity: Option<ItemQuantity>,
}

impl Item {
    pub fn new(options: ItemBuilder) -> Self {
        Item {
            id: Uuid::new_v4(),
            name: options.name,
            item_type: options.item_type.unwrap_or(ItemType::Resource),
            equipment_type: options.equipment_type.unwrap_or(EquipmentType::None),
            maximum_quantity: options.maximum_quantity.unwrap_or(1),
        }
    }
}
```

## Commenting Clone Usage

While it in general fine to clone objects (unless they have things like handles) it should be avoid when possible to avoid unnessacary object creation so when clone, we should comment why we are cloning instead of just using a reference which should work a lot of the time.
