#![windows_subsystem = "windows"]

// use dioxus::events::{KeyCode, KeyboardEvent, FormData};
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

#[macro_use]
mod helper_macros;
mod win32_sound;
mod misc_defs;
mod cli_def;
mod command_logic;

use command_logic::*;
use cli_def::*;
use windows::Win32::Media::Audio::IAudioSessionControl2;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Menu {
    MAIN,
    MUTE,
    EQUALIZE
}

struct Store
{
    pub menu: Menu,
    pub mute_duration: u32,
    pub msg: String,
}
impl Store
{
    pub fn new () -> Store
    {
        Store
        {
            msg: "Idle".to_string(),
            menu: Menu::MAIN,
            mute_duration: 45,
        }
    }
}

fn use_store (cx: &ScopeState) -> UseSharedState<Store>
{
    use_shared_state::<Store>(cx).expect("Store settings not provided")
}



fn commands_menu(cx: Scope) -> Element {
    let store = use_store(&cx);
    let menu = store.read().menu;

    cx.render(rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            p { "Commands" }
            button { 
                background_color: if menu == Menu::MUTE { "var(--blue0)" } else { "var(--blue-1)" },
                onclick: move |_| store.write().menu = Menu::MUTE, "Mute" 
            }
            button { 
                margin_top: ".125rem",
                background_color: if menu == Menu::EQUALIZE { "var(--blue0)" } else { "var(--blue-1)" },
                onclick: move |_| store.write().menu = Menu::EQUALIZE, "Equalize" 
            }
        }
    ))
}

fn get_session_exe_name (sess: &IAudioSessionControl2) -> String {
    let raw = win32_sound::session_iden(sess).unwrap().to_string();
    let regex = regex::Regex::new(r"(?P<process>[\w\.-]*)\.exe").unwrap();

    match  regex.captures(raw.as_str()) {
        Some(captures) => captures["process"].to_string() + ".exe",
        None => "None!".to_string(),
    }
}

fn mute_menu<'a>(cx: Scope<'a>) -> Element {
    let store = use_store(&cx);
    let mute_dur = store.read().mute_duration;
    let device = win32_sound::default_device().unwrap();

    // figure out how to store these sessions
    let sessions = win32_sound::SessionIterator::new(&device);
    let chosen_session = use_state(&cx, || "".to_string());
    let chosen_session_str = chosen_session.get();

    cx.render(rsx! (
        div {
            width: "100%",
            display: "flex",
            flex_direction: "column",
            div {
                margin_top: ".5rem",
                display: "flex",
                input {
                    margin_right: ".5rem",
                    r#type: "number",
                    value: "{mute_dur}",
                    onchange: move |evt: FormEvent| store.write().mute_duration = evt.value.parse::<u32>().unwrap(),
                }
                div {
                    min_width: "20%",
                    "{chosen_session_str}"
                }
            }
            ul {
                padding: "0",
                margin: "0",
                list_style: "none",
                sessions.map(|session| { 
                    let name = get_session_exe_name(&session);
                    cx.render(rsx! (
                        li {
                            margin_top: ".125rem",
                            button {
                                background_color: if &name == chosen_session_str { "var(--blue0)" } else { "var(--blue-1" },
                                onclick: move |_| chosen_session.set(name.clone()),
                                "{name}"
                            }
                        }
                    )) 
                })
            }
            div {
                display: "flex",
                align_items: "flex-end",
                width: "100%",
                margin_top: ".5rem",
                button {
                    margin_left: "auto",
                    background_color: "var(--red0)",
                    onclick: move |_| {
                        store.write().msg = format!("Muting {chosen_session_str} for {mute_dur}");
                        mute_logic(MuteArgs { title: chosen_session_str.to_string(), duration: mute_dur as u64 });
                        store.write().msg = format!("Finished muting {chosen_session_str} for {mute_dur}");
                    },
                    "MUTE"
                }
            }
        }
    ))
}

fn equalize_menu(cx: Scope) -> Element {
    let store = use_store(&cx);

    cx.render(rsx! (
        div {
        }
    ))
}

fn empty (cx: Scope) -> Element {
    cx.render(rsx! (
        div{}
    ))
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(&cx, || Store::new());
    let store = use_store(&cx);

    let msg = store.read().msg.clone();

    let component = match use_store(&cx).read().menu {
        Menu::MAIN => empty,
        Menu::MUTE => mute_menu,
        Menu::EQUALIZE => equalize_menu,
    };

    return cx.render(rsx! (
        style { include_str!("./styles.css") },
        div {
            class: "main",
            "{msg}"
            div {
                display: "flex",
                flex_direction: "row",
                div {
                    margin_right: ".75rem",
                    padding_right: ".75rem",
                    border_right: "1px solid gray",
                    commands_menu {}
                }
                div {
                    component(cx)
                }
            }

        }
    ))
}


fn main () {
    let config = Config::new().with_window(
        WindowBuilder::default()
            .with_title("wmx")
            .with_inner_size(dioxus_desktop::LogicalSize::new(480.0, 300.0)),
    );

    dioxus_desktop::launch_cfg(app, config);
}
