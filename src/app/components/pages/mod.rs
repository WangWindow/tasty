pub mod home_page;
pub mod settings_page;

use home_page::HomePage;
use leptos::tachys::view;
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use settings_page::SettingsPage;
use wasm_bindgen::prelude::*;

pub fn page_selector(page: &str) -> impl IntoView {}
