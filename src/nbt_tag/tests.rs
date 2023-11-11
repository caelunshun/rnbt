#[cfg(test)]

use super::*;

#[test]
fn test_nbt_tag_type_ids() {
    assert_eq!(NbtTagType::End.id()         ,0);
    assert_eq!(NbtTagType::Byte.id()        ,1);
    assert_eq!(NbtTagType::Short.id()       ,2);
    assert_eq!(NbtTagType::Int.id()         ,3);
    assert_eq!(NbtTagType::Long.id()        ,4);
    assert_eq!(NbtTagType::Float.id()       ,5);
    assert_eq!(NbtTagType::Double.id()      ,6);
    assert_eq!(NbtTagType::ByteArray.id()   ,7);
    assert_eq!(NbtTagType::String.id()      ,8);
    assert_eq!(NbtTagType::List.id()        ,9);
    assert_eq!(NbtTagType::Compound.id()    ,10);
    assert_eq!(NbtTagType::IntArray.id()    ,11);
    assert_eq!(NbtTagType::LongArray.id()   ,12);
    }

#[test]
fn test_nbt_tag_type_from_id() {
    assert_eq!(NbtTagType::from_id(0), Some(NbtTagType::End));
    assert_eq!(NbtTagType::from_id(1), Some(NbtTagType::Byte));
    assert_eq!(NbtTagType::from_id(2), Some(NbtTagType::Short));
    assert_eq!(NbtTagType::from_id(3), Some(NbtTagType::Int));
    assert_eq!(NbtTagType::from_id(4), Some(NbtTagType::Long));
    assert_eq!(NbtTagType::from_id(5), Some(NbtTagType::Float));
    assert_eq!(NbtTagType::from_id(6), Some(NbtTagType::Double));
    assert_eq!(NbtTagType::from_id(7), Some(NbtTagType::ByteArray));
    assert_eq!(NbtTagType::from_id(8), Some(NbtTagType::String));
    assert_eq!(NbtTagType::from_id(9), Some(NbtTagType::List));
    assert_eq!(NbtTagType::from_id(10), Some(NbtTagType::Compound));
    assert_eq!(NbtTagType::from_id(11), Some(NbtTagType::IntArray));
    assert_eq!(NbtTagType::from_id(12), Some(NbtTagType::LongArray));
    assert_eq!(NbtTagType::from_id(255), None); // Test an invalid ID
}

