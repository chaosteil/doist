use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

fn find_id_index(array: &[serde_json::Value], id: u64) -> Option<usize> {
    for (i, item) in array.iter().enumerate() {
        if let Some(item_id) = item
            .as_object()
            .and_then(|i| i.get("id"))
            .and_then(|i| i.as_u64())
        {
            if item_id == id {
                return Some(i);
            }
        }
    }
    None
}

fn merge_with_deleted_arrays(
    original: &mut Vec<serde_json::Value>,
    patch: &Vec<serde_json::Value>,
) {
    if original.is_empty() && patch.is_empty() {
        return;
    }
    // Find the item which has an "id" and an "is_deleted" key.
    // If both items are present and fully valid, proceed with the fancy array merge logic
    for (item, id, deleted) in patch
        .iter()
        .filter(|map| map.is_object())
        .filter_map(|item| {
            let map = item.as_object().unwrap();
            let id = map.get("id").and_then(|id| id.as_u64());
            // API has is_deleted with both bools and numbers
            let deleted = map.get("is_deleted").and_then(|deleted| {
                deleted
                    .as_bool()
                    .map_or_else(|| deleted.as_u64().map(|v| v == 1), Some)
            });
            if let Some(id) = id {
                if let Some(deleted) = deleted {
                    return Some((item, id, deleted));
                }
            }
            None
        })
    {
        let idx = find_id_index(original, id);
        if deleted {
            if let Some(i) = idx {
                original.remove(i);
            }
        } else if let Some(i) = idx {
            original[i] = item.clone();
        } else {
            original.push(item.clone());
        }
    }
}

pub fn merge_sync_state(original: &mut serde_json::Value, patch: &serde_json::Value) {
    if original.is_array() && patch.is_array() {
        merge_with_deleted_arrays(original.as_array_mut().unwrap(), patch.as_array().unwrap());
        return;
    }
    if !patch.is_object() {
        *original = patch.clone();
        return;
    }

    if !original.is_object() {
        *original = serde_json::Value::Object(serde_json::Map::new());
    }
    let map = original.as_object_mut().unwrap();
    for (key, value) in patch.as_object().unwrap() {
        if value.is_null() {
            map.remove(key.as_str());
        } else {
            merge_sync_state(
                map.entry(key.as_str()).or_insert(serde_json::Value::Null),
                value,
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct State {
    pub sync_token: String,
    pub full_sync: bool,
    pub user: User,
    pub projects: Vec<Project>,
    pub items: Vec<Item>,
    pub notes: Vec<Note>,
    pub project_notes: Vec<ProjectNote>,
    pub sections: Vec<Section>,
    pub labels: Vec<Label>,
    pub filters: Vec<Filter>,
    pub reminders: Vec<Reminder>,
    pub collaborators: Vec<Collaborator>,
    pub collaborator_states: Vec<CollaboratorState>,
    pub live_notifications: Vec<LiveNotification>,
    pub day_orders: DayOrders,
    pub live_notifications_last_read_id: i64,
    pub user_settings: UserSettings,
    pub user_plan_limits: UserPlanLimits,
}

// TODO: -1 for no order, 0-> for an order. Option type maybe?
pub type Order = i32;

pub type UserID = u64;

pub type BusinessAccountID = u64;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TimezoneInfo {
    pub gmt_string: String, // -03:00
    pub hours: i8,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_dst: bool,
    pub minutes: i32,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub auto_reminder: i32,
    pub avatar_big: Option<String>,
    pub avatar_medium: Option<String>,
    pub avatar_s640: Option<String>,
    pub avatar_small: Option<String>,
    pub business_account_id: Option<BusinessAccountID>,
    pub daily_goal: i32,
    pub date_format: i8, // TODO: enum 0 -> DD-MM-YYYY, 1 -> MM-DD-YYYY
    pub dateist_inline_disabled: bool,
    pub dateist_lang: Option<String>,
    pub days_off: Vec<i8>, // 1-> Monday, 7-> Sunday
    pub email: String,
    pub full_name: String,
    pub id: UserID,
    pub image_id: Option<String>,
    pub inbox_project: ProjectID,
    pub is_biz_admin: bool,
    pub is_premium: bool,
    pub join_date: Option<DateTime<Utc>>,
    pub karma: f64,
    pub karma_trend: String, // TODO: up/down enum?
    pub lang: String,
    pub legacy_inbox_project: Option<ProjectID>,
    pub legacy_team_inbox: Option<ProjectID>,
    pub next_week: i8, // 1-> Monday, 7-> Sunday
    pub premium_until: Option<DateTime<Utc>>,
    pub sort_order: i8, // 0-> oldest first, 1-> oldest last
    pub start_day: i8,  // 1-> Monday, 7-> Sunday
    pub start_page: String,
    pub team_inbox: Option<ProjectID>,
    pub theme: i8,
    pub time_format: i8, // 0->24h, 1->12h
    pub token: String,
    pub tz_info: TimezoneInfo,
    pub weekly_goal: u32,
}

pub type ProjectID = u64;

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u16)]
pub enum Color {
    Unknown,
    BerryRed = 30,
    Red,
    Orange,
    Yellow,
    OliveGreen,
    LimeGreen,
    Green,
    MintGreen,
    Teal,
    SkyBlue,
    LightBlue,
    Blue,
    Grape,
    Violet,
    Lavender,
    Magenta,
    Salmon,
    Charcoal,
    Grey,
    Taupe,
}

impl Default for Color {
    fn default() -> Self {
        Color::Unknown
    }
}

pub type SyncID = u64;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Project {
    pub id: ProjectID,
    pub legacy_id: Option<ProjectID>,
    pub name: String,
    pub color: Color,
    pub parent_id: Option<ProjectID>,
    pub legacy_parent_id: Option<ProjectID>,
    pub child_order: Order,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub collapsed: bool,
    pub shared: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_archived: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_favorite: bool,
    pub sync_id: Option<SyncID>,
    pub inbox_project: Option<bool>,
    pub team_inbox: Option<bool>,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_deleted: bool,
}

pub type ItemID = u64;

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum Priority {
    Natural = 1,
    High,
    Urgent,
    VeryUrgent,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Natural
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Item {
    pub id: ItemID,
    pub legacy_id: Option<ItemID>,
    pub user_id: UserID,
    pub project_id: ProjectID,
    pub legacy_project_id: Option<ProjectID>,
    pub content: String,
    pub description: String,
    pub due: Option<DueDate>,
    pub priority: Priority,
    pub parent_id: Option<ItemID>,
    pub legacy_parent_id: Option<ItemID>,
    pub child_order: Order,
    pub section_id: Option<SectionID>,
    pub day_order: Order,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub collapsed: bool,
    pub labels: Vec<LabelID>,
    pub added_by_uid: UserID,
    pub assigned_by_uid: Option<UserID>,
    pub responsible_uid: Option<UserID>,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub checked: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_deleted: bool,
    pub sync_id: Option<SyncID>,
    pub date_completed: Option<DateTime<Utc>>,
    pub date_added: Option<DateTime<Utc>>,
}

type NoteID = u64;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Note {
    pub id: NoteID,
    pub legacy_id: Option<NoteID>,
    pub posted_uid: UserID,
    pub item_id: ItemID,
    pub legacy_item_id: Option<ItemID>,
    pub project_id: ProjectID,
    pub legacy_project_id: Option<ProjectID>,
    pub content: String,
    pub file_attachment: Option<FileAttachment>,
    pub uids_to_notify: Vec<UserID>,
}

pub type ProjectNoteID = u64;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectNote {
    pub id: ProjectNoteID,
    pub posted_uid: UserID,
    pub content: String,
    pub file_attachment: Option<FileAttachment>,
    pub uids_to_notify: Vec<UserID>,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_deleted: bool,
    pub posted: Option<DateTime<Utc>>,
    pub reactions: HashMap<String, Vec<UserID>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UploadState {
    Unknown,
    Pending,
    Completed,
    // TODO: more?
}

impl Default for UploadState {
    fn default() -> Self {
        UploadState::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FileAttachment {
    pub file_name: String,
    pub file_size: u64,
    pub file_type: String,
    pub file_url: String,
    pub upload_state: UploadState,
}

pub type SectionID = u64;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Section {
    pub id: SectionID,
    pub name: String,
    pub project_id: ProjectID,
    pub legacy_project_id: Option<ProjectID>,
    pub section_order: Order,
    pub collapsed: bool,
    pub sync_id: Option<SyncID>,
    pub is_deleted: bool,
    pub is_archived: bool,
    pub date_archived: Option<DateTime<Utc>>,
    pub date_added: Option<DateTime<Utc>>,
}

pub type LabelID = u64;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Label {
    pub id: LabelID,
    pub name: String,
    pub color: Color,
    pub item_order: Order,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_deleted: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_favorite: bool,
}

pub type FilterID = u64;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Filter {
    pub id: FilterID,
    pub name: String,
    pub query: String,
    pub color: Color,
    pub item_order: Order,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_deleted: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_favorite: bool,
}

pub type ReminderID = u64;

#[derive(Debug, Serialize, Deserialize)]
pub enum Service {
    Unknown,
    Email,
    Mobile,
    Push,
}

impl Default for Service {
    fn default() -> Self {
        Service::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReminderType {
    Unknown,
    #[serde(rename = "relative")]
    Relative {
        due: DueDate,
        mm_offset: i32,
    },
    #[serde(rename = "absolute")]
    Absolute {
        due: DueDate,
    },
    #[serde(rename = "location")]
    Location {
        name: String,
        loc_lat: String,
        loc_long: String,
        radius: i32,
    },
}

impl Default for ReminderType {
    fn default() -> Self {
        ReminderType::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Reminder {
    pub id: ReminderID,
    pub notify_uid: UserID,
    pub item_id: ItemID,
    pub service: Service,
    #[serde(flatten)]
    pub reminder_type: ReminderType,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DueDateType {
    None {},
    FullDay {
        date: NaiveDate,
    },
    Time {
        date: NaiveDateTime,
    },
    Fixed {
        date: DateTime<Utc>,
        timezone: chrono_tz::Tz,
    },
}

impl Default for DueDateType {
    fn default() -> Self {
        DueDateType::None {}
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DueDate {
    #[serde(flatten)]
    pub due: DueDateType,
    pub string: String,
    pub lang: String,
    pub is_recurring: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DayOrders {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Collaborator {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CollaboratorState {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LiveNotification {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserSettings {
    pub reminder_push: bool,
    pub reminder_desktop: bool,
    pub reminder_email: bool,
    pub completed_sound_desktop: bool,
    pub completed_sound_mobile: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserPlanLimits {}
