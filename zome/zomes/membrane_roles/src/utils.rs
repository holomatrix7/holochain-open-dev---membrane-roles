use hc_utils::WrappedAgentPubKey;
use hdk3::prelude::*;
use holo_hash;

pub fn try_get_and_convert<T: TryFrom<SerializedBytes>>(entry_hash: EntryHash) -> ExternResult<T> {
    match get(entry_hash, GetOptions)? {
        Some(element) => try_from_element(element),
        None => crate::error("Entry not found"),
    }
}

pub fn try_from_element<T: TryFrom<SerializedBytes>>(element: Element) -> ExternResult<T> {
    match element.entry() {
        element::ElementEntry::Present(entry) => try_from_entry::<T>(entry.clone()),
        _ => crate::error("Could not convert element"),
    }
}

pub fn try_from_entry<T: TryFrom<SerializedBytes>>(entry: Entry) -> ExternResult<T> {
    match entry {
        Entry::App(content) => match T::try_from(content.into_sb()) {
            Ok(e) => Ok(e),
            Err(_) => crate::error("Could not convert entry"),
        },
        _ => crate::error("Could not convert entry"),
    }
}

#[derive(Serialize, Deserialize, SerializedBytes)]
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

pub fn _pub_key_to_tag(agent_pub_key: WrappedAgentPubKey) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = agent_pub_key.try_into()?;

    Ok(LinkTag(sb.bytes().clone()))
}

pub fn _tag_to_pub_key(tag: LinkTag) -> ExternResult<WrappedAgentPubKey> {
    let sb = SerializedBytes::from(UnsafeBytes::from(tag.0));

    let pub_key = WrappedAgentPubKey::try_from(sb)?;

    Ok(pub_key)
}

pub fn _entry_hash_to_pub_key(entry_hash: EntryHash) -> AgentPubKey {
    entry_hash.retype(holo_hash::hash_type::Agent)
}

pub fn pub_key_to_entry_hash(agent_pub_key: AgentPubKey) -> EntryHash {
    let agent_address: AnyDhtHash = agent_pub_key.into();
    agent_address.into()
}
