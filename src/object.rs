use crate::{
    core::{Core, CoreStruct},
    Catalog, Collection, Error, Item, CATALOG_TYPE, COLLECTION_TYPE, ITEM_TYPE,
};
use serde_json::Value;

const TYPE_FIELD: &str = "type";

/// An enum to hold any of the STAC object types.
#[derive(Debug)]
pub enum Object {
    /// A STAC Item.
    Item(Item),

    /// A STAC Catalog.
    Catalog(Catalog),

    /// A STAC Collection.
    Collection(Collection),
}

impl Object {
    /// Returns a reference to this object as a Catalog, or none if it is not a catalog.
    ///
    /// # Examples
    ///
    /// ```
    /// let catalog = stac::read("data/catalog.json").unwrap();
    /// println!("Description: {}", catalog.as_catalog().unwrap().description());
    /// ```
    pub fn as_catalog(&self) -> Option<&Catalog> {
        match self {
            Object::Catalog(catalog) => Some(catalog),
            _ => None,
        }
    }

    pub(crate) fn from_value(value: Value) -> Result<Object, Error> {
        if let Some(type_) = value.get(TYPE_FIELD) {
            if let Some(type_) = type_.as_str() {
                match type_ {
                    ITEM_TYPE => Ok(Object::Item(serde_json::from_value(value)?)),
                    CATALOG_TYPE => Ok(Object::Catalog(serde_json::from_value(value)?)),
                    COLLECTION_TYPE => Ok(Object::Collection(serde_json::from_value(value)?)),
                    _ => Err(Error::InvalidTypeValue(type_.to_string())),
                }
            } else {
                Err(Error::InvalidTypeField(type_.clone()))
            }
        } else {
            Err(Error::MissingType)
        }
    }
}

impl AsRef<CoreStruct> for Object {
    fn as_ref(&self) -> &CoreStruct {
        match self {
            Object::Item(item) => item.as_ref(),
            Object::Catalog(catalog) => catalog.as_ref(),
            Object::Collection(collection) => collection.as_ref(),
        }
    }
}

impl AsMut<CoreStruct> for Object {
    fn as_mut(&mut self) -> &mut CoreStruct {
        match self {
            Object::Item(item) => item.as_mut(),
            Object::Catalog(catalog) => catalog.as_mut(),
            Object::Collection(collection) => collection.as_mut(),
        }
    }
}

impl Core for Object {}
