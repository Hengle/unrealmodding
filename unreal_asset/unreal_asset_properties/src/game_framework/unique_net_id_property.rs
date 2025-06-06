//! Unique network id property

use crate::property_prelude::*;

/// Unique network id
#[derive(FNameContainer, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct UniqueNetId {
    /// Type
    pub ty: FName,
    /// Contents
    pub contents: Option<String>,
}

/// Unique network id property
#[derive(FNameContainer, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct UniqueNetIdProperty {
    /// Name
    pub name: FName,
    /// Property ancestry
    pub ancestry: Ancestry,
    /// Property guid
    pub property_guid: Option<Guid>,
    /// Property duplication index
    pub duplication_index: i32,
    /// Value
    pub value: Option<UniqueNetId>,
}
impl_property_data_trait!(UniqueNetIdProperty);

impl UniqueNetIdProperty {
    /// Read a `UniqueNetIdProperty` from an asset
    pub fn new<Reader: ArchiveReader<impl PackageIndexTrait>>(
        asset: &mut Reader,
        name: FName,
        ancestry: Ancestry,
        include_header: bool,
        _length: i64,
        duplication_index: i32,
    ) -> Result<Self, Error> {
        let property_guid = optional_guid!(asset, include_header);

        let size = asset.read_i32::<LE>()?;
        let value = match size > 0 {
            true => Some(UniqueNetId {
                ty: asset.read_fname()?,
                contents: asset.read_fstring()?,
            }),
            false => None,
        };

        Ok(UniqueNetIdProperty {
            name,
            ancestry,
            property_guid,
            duplication_index,
            value,
        })
    }
}

impl PropertyTrait for UniqueNetIdProperty {
    fn write<Writer: ArchiveWriter<impl PackageIndexTrait>>(
        &self,
        asset: &mut Writer,
        include_header: bool,
    ) -> Result<usize, Error> {
        optional_guid_write!(self, asset, include_header);

        let begin = asset.position();

        match &self.value {
            Some(value) => {
                let mut length = 3 * size_of::<i32>();
                asset.write_fname(&value.ty)?;
                length += asset.write_fstring(value.contents.as_deref())?;

                let end = asset.position();
                asset.seek(SeekFrom::Start(begin))?;
                asset.write_i32::<LE>(length as i32)?;
                asset.seek(SeekFrom::Start(end))?;
            }
            None => {
                asset.write_i32::<LE>(0)?;
            }
        }

        Ok((asset.position() - begin) as usize)
    }
}
