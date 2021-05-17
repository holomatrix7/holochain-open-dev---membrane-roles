use hdk::prelude::*;
use crate::err;

pub fn try_get_and_convert<T: TryFrom<SerializedBytes>>(entry_hash: EntryHash) -> ExternResult<T> {
    match get(entry_hash, GetOptions::default())? {
        Some(element) => try_from_element(element),
        None => Err(err("Entry not found")),
    }
}

pub fn try_from_element<T: TryFrom<SerializedBytes>>(element: Element) -> ExternResult<T> {
    match element.entry() {
        element::ElementEntry::Present(entry) => try_from_entry::<T>(entry.clone()),
        _ => Err(err("Could not convert element")),
    }
}

pub fn try_from_entry<T: TryFrom<SerializedBytes>>(entry: Entry) -> ExternResult<T> {
    match entry {
        Entry::App(content) => match T::try_from(content.into_sb()) {
            Ok(e) => Ok(e),
            Err(_) => Err(err("Could not convert entry")),
        },
        _ => Err(err("Could not convert entry")),
    }
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct StringLinkTag(String);
pub fn link_tag(tag: &str) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = StringLinkTag(tag.into()).try_into()?;
    Ok(LinkTag(sb.bytes().clone()))
}
pub fn tag_to_string(tag: LinkTag) -> ExternResult<String> {
    let bytes = SerializedBytes::from(UnsafeBytes::from(tag.0));
    let string_tag: StringLinkTag = bytes.try_into()?;

    Ok(string_tag.0)
}

pub fn pub_key_to_entry_hash(agent_pub_key: AgentPubKey) -> EntryHash {
    let agent_address: AnyDhtHash = agent_pub_key.into();
    agent_address.into()
}
