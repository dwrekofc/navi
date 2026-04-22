use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration as StdDuration;

use anyhow::{Context as AnyhowContext, Result, anyhow};
use chrono::{Duration, Local, NaiveDate};
use gpui::{
    App, Bounds, Context, CursorStyle, Decorations, FocusHandle, Focusable, HitboxBehavior,
    IntoElement, KeyDownEvent, MouseButton, ParentElement, Pixels, Point, QuitMode, Render,
    ResizeEdge, Size, Window, WindowBackgroundAppearance, WindowBounds, WindowDecorations,
    WindowKind, WindowOptions, canvas, div, point, prelude::*, px, rgb, size, transparent_black,
};
use gpui_platform::application;
use rusqlite::{Connection, params};
use serde::Deserialize;

const SURFACE_DARKEST: u32 = 0x383838;
const SURFACE_DARK: u32 = 0x4A4A4A;
const SURFACE_LIGHT: u32 = 0x525252;
const SURFACE_CONTROL: u32 = 0x5A5A5A;
const DISPLAY_BG: u32 = 0x404040;
const TEXT_PRIMARY: u32 = 0xFFFFFF;
const TEXT_SECONDARY: u32 = 0xB4B4B4;
const TEXT_MUTED: u32 = 0x909090;
const TEXT_DISABLED: u32 = 0x6A6A6A;
const ACCENT: u32 = 0xFF7A20;
const TAG_BG: u32 = 0x2A2A2A;
const CHAT_MINE: u32 = 0xCC5500;

#[derive(Clone)]
struct DailyNote {
    date: NaiveDate,
    key: String,
    path: PathBuf,
    content: String,
    exists: bool,
}

#[derive(Clone)]
struct Task {
    title: String,
    project: String,
    status: String,
    day_key: String,
    summary: String,
}

#[derive(Clone)]
struct Project {
    name: String,
    summary: String,
}

#[derive(Clone)]
enum ChatRole {
    User,
    Assistant,
    System,
}

#[derive(Clone)]
struct ChatMessage {
    role: ChatRole,
    text: String,
}

struct CaptureStore {
    root: PathBuf,
    vault_dir: PathBuf,
    daily_dir: PathBuf,
    db_path: PathBuf,
    logs_dir: PathBuf,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CaptureKind {
    Task,
    Bullet,
    Plain,
}

struct CaptureInput<'a> {
    kind: CaptureKind,
    body: &'a str,
}

impl<'a> CaptureInput<'a> {
    fn parse(input: &'a str) -> Self {
        if let Some(body) = input.strip_prefix("* ") {
            Self {
                kind: CaptureKind::Task,
                body: body.trim(),
            }
        } else if let Some(body) = input.strip_prefix("- ") {
            Self {
                kind: CaptureKind::Bullet,
                body: body.trim(),
            }
        } else {
            Self {
                kind: CaptureKind::Plain,
                body: input,
            }
        }
    }
}

impl CaptureStore {
    fn open(root: PathBuf) -> Result<Self> {
        let vault_dir = root.clone();
        let daily_dir = vault_dir.join("daily");
        let logs_dir = vault_dir.join("logs");
        let db_path = vault_dir.join("navi.sqlite3");
        fs::create_dir_all(&vault_dir)?;
        fs::create_dir_all(&daily_dir)?;
        fs::create_dir_all(vault_dir.join("context"))?;
        fs::create_dir_all(&logs_dir)?;

        let store = Self {
            root,
            vault_dir,
            daily_dir,
            db_path,
            logs_dir,
        };
        store.init_db()?;
        Ok(store)
    }

    fn init_db(&self) -> Result<()> {
        let conn = self.conn()?;
        conn.execute_batch(
            "
            PRAGMA journal_mode = WAL;
            CREATE TABLE IF NOT EXISTS daily_notes (
                day_key TEXT PRIMARY KEY,
                path TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS projects (
                name TEXT PRIMARY KEY,
                summary TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'open',
                project TEXT NOT NULL DEFAULT 'Inbox',
                day_key TEXT NOT NULL,
                summary TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS captures (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                day_key TEXT NOT NULL,
                role TEXT NOT NULL,
                body TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            ",
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO projects (name, summary, created_at) VALUES (?1, ?2, ?3)",
            params![
                "Inbox",
                "Default holding project for uncategorized quick captures.",
                now_iso()
            ],
        )?;
        Ok(())
    }

    fn conn(&self) -> Result<Connection> {
        Connection::open(&self.db_path).with_context(|| {
            format!(
                "failed to open SQLite database at {}",
                self.db_path.display()
            )
        })
    }

    fn load_days(&self, center: NaiveDate, radius: i64) -> Result<Vec<DailyNote>> {
        let mut days = Vec::new();
        for offset in -radius..=radius {
            let date = center + Duration::days(offset);
            let key = day_key(date);
            let path = self.daily_dir.join(format!("{key}.md"));
            let exists = path.exists();
            let content = if exists {
                fs::read_to_string(&path).unwrap_or_else(|_| String::new())
            } else {
                String::new()
            };
            days.push(DailyNote {
                date,
                key,
                path,
                content,
                exists,
            });
        }
        Ok(days)
    }

    fn load_tasks(&self) -> Result<Vec<Task>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT title, project, status, day_key, summary
             FROM tasks
             ORDER BY CASE status WHEN 'open' THEN 0 ELSE 1 END, id DESC
             LIMIT 80",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Task {
                title: row.get(0)?,
                project: row.get(1)?,
                status: row.get(2)?,
                day_key: row.get(3)?,
                summary: row.get(4)?,
            })
        })?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(Into::into)
    }

    fn load_projects(&self) -> Result<Vec<Project>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare("SELECT name, summary FROM projects ORDER BY name")?;
        let rows = stmt.query_map([], |row| {
            Ok(Project {
                name: row.get(0)?,
                summary: row.get(1)?,
            })
        })?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(Into::into)
    }

    fn capture(&self, input: &str) -> Result<()> {
        let text = input.trim();
        if text.is_empty() {
            return Ok(());
        }

        let today = Local::now().date_naive();
        let key = day_key(today);
        let path = self.daily_dir.join(format!("{key}.md"));
        let now = now_iso();
        let capture = CaptureInput::parse(text);
        let clean_text = capture.body.replace('\n', " ");
        let project = extract_project(&clean_text).unwrap_or_else(|| "Inbox".to_string());
        let line = match capture.kind {
            CaptureKind::Task => format!("- [ ] {clean_text}"),
            CaptureKind::Bullet => format!("- {clean_text}"),
            CaptureKind::Plain => clean_text.clone(),
        };

        if !path.exists() {
            fs::write(&path, format!("# {key}\n\n"))?;
        }
        let mut existing = fs::read_to_string(&path).unwrap_or_default();
        if !existing.ends_with('\n') {
            existing.push('\n');
        }
        existing.push_str(&format!("{}\n", line));
        fs::write(&path, existing)?;

        let conn = self.conn()?;
        conn.execute(
            "INSERT OR IGNORE INTO daily_notes (day_key, path, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?3)",
            params![key, path.to_string_lossy(), now],
        )?;
        conn.execute(
            "UPDATE daily_notes SET updated_at = ?1 WHERE day_key = ?2",
            params![now, key],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO projects (name, summary, created_at) VALUES (?1, ?2, ?3)",
            params![
                project,
                format!("Context profile for {project}."),
                now.clone()
            ],
        )?;
        conn.execute(
            "INSERT INTO captures (day_key, role, body, created_at) VALUES (?1, 'user', ?2, ?3)",
            params![key, text, now],
        )?;

        if matches!(capture.kind, CaptureKind::Task) {
            conn.execute(
                "INSERT INTO tasks (title, status, project, day_key, summary, created_at, updated_at)
                 VALUES (?1, 'open', ?2, ?3, ?4, ?5, ?5)",
                params![
                    clean_text,
                    project,
                    key,
                    summarize(text, 120),
                    now,
                ],
            )?;
        }
        Ok(())
    }

    fn preview_for_link(&self, key: &str, tasks: &[Task], projects: &[Project]) -> String {
        if key.len() == 8 && key.chars().all(|ch| ch.is_ascii_digit()) {
            return format!("Daily note for {key}.");
        }
        if let Some(task) = tasks.iter().find(|task| task.title == key) {
            return format!("{} [{} / {}]", task.summary, task.project, task.day_key);
        }
        if let Some(project) = projects.iter().find(|project| project.name == key) {
            return project.summary.clone();
        }
        format!("Linked note or context profile: {key}")
    }

    fn toggle_checkbox_line(&self, day_key: &str, line: &str) -> Result<()> {
        let path = self.daily_dir.join(format!("{day_key}.md"));
        let content = fs::read_to_string(&path)?;
        let toggled = if line.trim_start().starts_with("- [ ] ") {
            line.replacen("- [ ] ", "- [x] ", 1)
        } else if line.trim_start().starts_with("- [x] ") {
            line.replacen("- [x] ", "- [ ] ", 1)
        } else {
            return Ok(());
        };
        let next = content.replacen(line, &toggled, 1);
        fs::write(path, next)?;

        let title = toggled
            .trim_start()
            .trim_start_matches("- [x] ")
            .trim_start_matches("- [ ] ");
        let status = if toggled.trim_start().starts_with("- [x] ") {
            "done"
        } else {
            "open"
        };
        self.conn()?.execute(
            "UPDATE tasks SET status = ?1, updated_at = ?2 WHERE day_key = ?3 AND title = ?4",
            params![status, now_iso(), day_key, title],
        )?;
        Ok(())
    }
}

struct NaviApp {
    store: CaptureStore,
    focus_handle: FocusHandle,
    days: Vec<DailyNote>,
    tasks: Vec<Task>,
    projects: Vec<Project>,
    chat: Vec<ChatMessage>,
    draft: String,
    status: String,
    claude_session_id: Option<String>,
    claude_enabled: bool,
    collapsed: bool,
    cursor_visible: bool,
    cursor_task_started: bool,
    hover_preview: Option<LinkPreviewData>,
    chat_height: f32,
    split_dragging: bool,
    font_scale: f32,
}

#[derive(Clone)]
struct LinkPreviewData {
    title: String,
    summary: String,
}

impl NaviApp {
    fn sp(&self, value: f32) -> Pixels {
        px(value * self.font_scale)
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let store = CaptureStore::open(storage_root()).expect("failed to initialize capture store");
        let today = Local::now().date_naive();
        let days = store.load_days(today, 3).unwrap_or_default();
        let tasks = store.load_tasks().unwrap_or_default();
        let projects = store.load_projects().unwrap_or_default();
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);

        Self {
            store,
            focus_handle,
            days,
            tasks,
            projects,
            chat: vec![ChatMessage {
                role: ChatRole::System,
                text: "Ready. Type a note or task and press Return.".to_string(),
            }],
            draft: String::new(),
            status: "Local store ready".to_string(),
            claude_session_id: None,
            claude_enabled: std::env::var("NAVI_DISABLE_CLAUDE").ok().as_deref() != Some("1"),
            collapsed: false,
            cursor_visible: true,
            cursor_task_started: false,
            hover_preview: None,
            chat_height: 245.,
            split_dragging: false,
            font_scale: 1.,
        }
    }

    fn reload(&mut self) {
        let today = Local::now().date_naive();
        self.days = self.store.load_days(today, 3).unwrap_or_default();
        self.tasks = self.store.load_tasks().unwrap_or_default();
        self.projects = self.store.load_projects().unwrap_or_default();
    }

    fn handle_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let key = event.keystroke.key.as_str();
        if event.keystroke.modifiers.platform && key.eq_ignore_ascii_case("h") {
            cx.hide();
            cx.stop_propagation();
            return;
        }
        if event.keystroke.modifiers.platform && (key == "=" || key == "+") {
            self.font_scale = (self.font_scale + 0.08).min(1.6);
            cx.notify();
            cx.stop_propagation();
            return;
        }
        if event.keystroke.modifiers.platform && key == "-" {
            self.font_scale = (self.font_scale - 0.08).max(0.78);
            cx.notify();
            cx.stop_propagation();
            return;
        }
        match key {
            "enter" => {
                self.submit(window, cx);
                cx.stop_propagation();
            }
            "backspace" => {
                self.draft.pop();
                cx.notify();
                cx.stop_propagation();
            }
            "escape" => {
                self.draft.clear();
                cx.notify();
                cx.stop_propagation();
            }
            _ => {
                if event.keystroke.modifiers.platform || event.keystroke.modifiers.control {
                    return;
                }
                if let Some(ch) = event.keystroke.key_char.as_deref() {
                    if !ch.chars().any(|c| c.is_control()) {
                        self.draft.push_str(ch);
                        cx.notify();
                        cx.stop_propagation();
                    }
                } else if key == "space" {
                    self.draft.push(' ');
                    cx.notify();
                    cx.stop_propagation();
                }
            }
        }
    }

    fn toggle_collapsed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.collapsed = !self.collapsed;
        let width = window.viewport_size().width;
        let height = if self.collapsed { px(118.) } else { px(680.) };
        window.resize(size(width, height));
        cx.notify();
    }

    fn drag_split(&mut self, y: Pixels, window: &mut Window, cx: &mut Context<Self>) {
        let total_height = window.viewport_size().height.as_f32();
        let header = 34.;
        let status_and_padding = 38.;
        let desired = (total_height - y.as_f32() - status_and_padding)
            .clamp(140., total_height - header - 180.);
        self.chat_height = desired;
        cx.notify();
    }

    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let input = self.draft.trim().to_string();
        if input.is_empty() {
            return;
        }
        self.draft.clear();
        let claude_prompt = input
            .strip_prefix("@ ")
            .map(str::trim)
            .filter(|prompt| !prompt.is_empty())
            .map(ToOwned::to_owned);
        self.chat.push(ChatMessage {
            role: ChatRole::User,
            text: claude_prompt.clone().unwrap_or_else(|| input.clone()),
        });

        if claude_prompt.is_none() {
            match self.store.capture(&input) {
                Ok(()) => {
                    self.reload();
                    self.status = "Captured to markdown and SQLite".to_string();
                }
                Err(err) => {
                    self.status = format!("Capture failed: {err}");
                    self.chat.push(ChatMessage {
                        role: ChatRole::System,
                        text: self.status.clone(),
                    });
                }
            }
        }

        if self.claude_enabled || claude_prompt.is_some() {
            let prompt = if let Some(prompt) = claude_prompt {
                prompt
            } else {
                build_capture_prompt(&input, &self.store.vault_dir)
            };
            let session_id = self.claude_session_id.clone();
            let root = self.store.root.clone();
            let logs_dir = self.store.logs_dir.clone();
            self.status = "Claude is running".to_string();
            cx.spawn_in(window, async move |this, cx| {
                let result = cx
                    .background_executor()
                    .spawn(async move { ClaudeRunner::run(prompt, session_id, root, logs_dir) })
                    .await;
                let _ = this.update(cx, |app, cx| {
                    match result {
                        Ok(reply) => {
                            if let Some(session_id) = reply.session_id {
                                app.claude_session_id = Some(session_id);
                            }
                            for message in reply.messages {
                                app.chat.push(message);
                            }
                            app.reload();
                            app.status = "Claude finished".to_string();
                        }
                        Err(err) => {
                            app.status = format!("Claude failed: {err}");
                            app.chat.push(ChatMessage {
                                role: ChatRole::System,
                                text: app.status.clone(),
                            });
                        }
                    }
                    cx.notify();
                });
            })
            .detach();
        }

        cx.notify();
    }

    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h(px(34.))
            .px_2()
            .flex()
            .items_center()
            .justify_between()
            .bg(rgb(SURFACE_DARKEST))
            .text_size(self.sp(12.))
            .text_color(rgb(TEXT_PRIMARY))
            .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
                window.start_window_move();
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(traffic_button(0xFF5F57, |_window, cx| cx.hide()))
                    .child(traffic_button(0xFEBC2E, |window, _cx| {
                        window.minimize_window();
                    }))
                    .child(traffic_button(0x28C840, |window, _cx| {
                        window.toggle_fullscreen();
                    }))
                    .child(
                        div()
                            .id("collapse-button")
                            .ml_2()
                            .h(px(20.))
                            .px_2()
                            .flex()
                            .items_center()
                            .rounded(px(3.))
                            .bg(rgb(TAG_BG))
                            .text_size(self.sp(11.))
                            .text_color(rgb(TEXT_SECONDARY))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.toggle_collapsed(window, cx);
                            }))
                            .child(if self.collapsed { "expand" } else { "collapse" }),
                    )
                    .child(
                        div()
                            .ml_2()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("navi"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .text_size(self.sp(10.))
                    .text_color(rgb(TEXT_SECONDARY))
                    .child(format!(
                        "{} tasks",
                        self.tasks
                            .iter()
                            .filter(|task| task.status == "open")
                            .count()
                    ))
                    .child(if self.claude_enabled {
                        "Claude: on"
                    } else {
                        "Claude: local"
                    }),
            )
    }

    fn render_day(&self, day: &DailyNote, cx: &mut Context<Self>) -> impl IntoElement {
        let file_name = day
            .path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or(&day.key);
        let note_name = format!("{} {}", file_name, day.date.format("%A"));
        let placeholder = if day.exists {
            "Empty note."
        } else {
            "Type something, or insert a"
        };

        div()
            .flex()
            .flex_col()
            .gap_3()
            .py_6()
            .px_3()
            .border_t_1()
            .border_color(rgb(SURFACE_CONTROL))
            .child(
                div()
                    .text_size(self.sp(22.))
                    .line_height(self.sp(28.))
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(rgb(TEXT_PRIMARY))
                    .child(note_name),
            )
            .children(if day.content.trim().is_empty() {
                vec![
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .text_size(self.sp(14.))
                        .line_height(self.sp(22.))
                        .text_color(rgb(TEXT_DISABLED))
                        .child(placeholder)
                        .child(self.tag("Template"))
                        .into_any_element(),
                ]
            } else {
                day.content
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| {
                        self.render_markdown_line(&day.key, line, cx)
                            .into_any_element()
                    })
                    .collect::<Vec<_>>()
            })
    }

    fn render_markdown_line(
        &self,
        day_key: &str,
        line: &str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let trimmed = line.trim_start();
        let (prefix, body) = markdown_prefix(trimmed);
        let day_key = day_key.to_string();
        let original_line = line.to_string();
        div()
            .flex()
            .flex_wrap()
            .gap_2()
            .text_size(self.sp(15.))
            .line_height(self.sp(24.))
            .text_color(rgb(TEXT_PRIMARY))
            .whitespace_normal()
            .child(
                div()
                    .w(px(42.))
                    .text_color(rgb(0xD66A6A))
                    .child(self.render_markdown_prefix(
                        prefix.as_deref(),
                        day_key,
                        original_line,
                        cx,
                    )),
            )
            .children(
                split_wiki_links(body)
                    .into_iter()
                    .map(|segment| match segment {
                        TextSegment::Text(text) => {
                            div().whitespace_normal().child(text).into_any_element()
                        }
                        TextSegment::WikiLink(link) => {
                            let summary =
                                self.store
                                    .preview_for_link(&link, &self.tasks, &self.projects);
                            let preview = LinkPreviewData {
                                title: link.clone(),
                                summary,
                            };
                            let label = format!("[[{link}]]");
                            div()
                                .id(format!("wiki-link-{link}"))
                                .px_1()
                                .rounded(px(2.))
                                .bg(rgb(TAG_BG))
                                .text_color(rgb(ACCENT))
                                .on_hover(cx.listener(move |this, hovered, _window, cx| {
                                    if *hovered {
                                        this.hover_preview = Some(preview.clone());
                                    } else if this
                                        .hover_preview
                                        .as_ref()
                                        .is_some_and(|active| active.title == preview.title)
                                    {
                                        this.hover_preview = None;
                                    }
                                    cx.notify();
                                }))
                                .child(label)
                                .into_any_element()
                        }
                    }),
            )
    }

    fn render_notes(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let days = self
            .days
            .iter()
            .map(|day| self.render_day(day, cx).into_any_element())
            .collect::<Vec<_>>();
        div()
            .id("notes-scroll")
            .flex_1()
            .overflow_y_scroll()
            .px_3()
            .flex()
            .flex_col()
            .bg(rgb(SURFACE_DARKEST))
            .children(days)
    }

    fn render_chat(&self) -> impl IntoElement {
        div()
            .h(px(self.chat_height))
            .flex()
            .flex_col()
            .gap_2()
            .p(px(8.))
            .bg(rgb(SURFACE_DARK))
            .child(self.section_label("chat"))
            .child(
                div()
                    .id("chat-scroll")
                    .flex_1()
                    .overflow_y_scroll()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(
                        self.chat
                            .iter()
                            .rev()
                            .take(12)
                            .rev()
                            .map(|message| self.render_chat_message(message)),
                    ),
            )
            .child(self.render_composer())
            .child(
                div()
                    .text_size(self.sp(10.))
                    .text_color(rgb(TEXT_MUTED))
                    .child(self.status.clone()),
            )
    }

    fn render_composer(&self) -> impl IntoElement {
        let text = if self.draft.is_empty() {
            format!(
                "{}type a task, note, or project context",
                if self.cursor_visible { "|" } else { " " }
            )
        } else {
            format!(
                "{}{}",
                self.draft,
                if self.cursor_visible { "|" } else { "" }
            )
        };
        div()
            .id("composer")
            .h(px(66.))
            .px_3()
            .py_2()
            .flex()
            .items_start()
            .gap_2()
            .rounded(px(6.))
            .bg(rgb(DISPLAY_BG))
            .border_1()
            .border_color(rgb(SURFACE_CONTROL))
            .text_size(self.sp(13.))
            .line_height(self.sp(19.))
            .text_color(rgb(TEXT_PRIMARY))
            .cursor(CursorStyle::IBeam)
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .whitespace_normal()
                    .text_color(if self.draft.is_empty() {
                        rgb(TEXT_DISABLED)
                    } else {
                        rgb(TEXT_PRIMARY)
                    })
                    .child(text),
            )
    }

    fn render_splitter(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("notes-chat-splitter")
            .h(px(18.))
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(SURFACE_DARK))
            .cursor(CursorStyle::ResizeUpDown)
            .child(
                div()
                    .w(px(210.))
                    .h(px(8.))
                    .rounded(px(4.))
                    .bg(if self.split_dragging {
                        rgb(ACCENT)
                    } else {
                        rgb(0xFF4A2A)
                    }),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &gpui::MouseDownEvent, window, cx| {
                    this.split_dragging = true;
                    this.drag_split(event.position.y, window, cx);
                    cx.stop_propagation();
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _event, _window, cx| {
                    this.split_dragging = false;
                    cx.notify();
                    cx.stop_propagation();
                }),
            )
    }

    fn render_chat_message(&self, message: &ChatMessage) -> impl IntoElement {
        let (bg, align_end, text_color) = match message.role {
            ChatRole::User => (CHAT_MINE, true, TEXT_PRIMARY),
            ChatRole::Assistant => (SURFACE_DARKEST, false, TEXT_PRIMARY),
            ChatRole::System => (SURFACE_DARK, false, TEXT_MUTED),
        };
        div()
            .flex()
            .w_full()
            .when(align_end, |div| div.justify_end())
            .when(!align_end, |div| div.justify_start())
            .child(
                div()
                    .w_full()
                    .p(px(9.))
                    .rounded(px(8.))
                    .bg(rgb(bg))
                    .text_size(self.sp(13.))
                    .line_height(self.sp(19.))
                    .text_color(rgb(text_color))
                    .whitespace_normal()
                    .child(message.text.clone()),
            )
    }

    fn tag(&self, text: &str) -> impl IntoElement {
        div()
            .h(px(16.))
            .px_2()
            .flex()
            .items_center()
            .rounded(px(3.))
            .bg(rgb(TAG_BG))
            .text_size(self.sp(10.))
            .text_color(rgb(TEXT_SECONDARY))
            .child(text.to_string())
    }

    fn section_label(&self, text: &str) -> impl IntoElement {
        div()
            .h(px(24.))
            .flex()
            .items_center()
            .text_size(self.sp(9.))
            .font_weight(gpui::FontWeight::BOLD)
            .text_color(rgb(TEXT_PRIMARY))
            .child(text.to_uppercase())
    }

    fn render_markdown_prefix(
        &self,
        prefix: Option<&str>,
        day_key: String,
        line: String,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        match prefix {
            Some("[ ]") => div()
                .id(format!("checkbox-open-{day_key}-{line}"))
                .size(px(14.))
                .mt(px(5.))
                .rounded(px(7.))
                .border_2()
                .border_color(rgb(0xD66A6A))
                .cursor_pointer()
                .on_click(cx.listener(move |this, _event, _window, cx| {
                    match this.store.toggle_checkbox_line(&day_key, &line) {
                        Ok(()) => {
                            this.reload();
                            this.status = "Task updated".to_string();
                        }
                        Err(err) => {
                            this.status = format!("Task update failed: {err}");
                        }
                    }
                    cx.notify();
                }))
                .into_any_element(),
            Some("[x]") => div()
                .id(format!("checkbox-done-{day_key}-{line}"))
                .size(px(14.))
                .mt(px(5.))
                .rounded(px(7.))
                .border_2()
                .border_color(rgb(0xA3D977))
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .on_click(cx.listener(move |this, _event, _window, cx| {
                    match this.store.toggle_checkbox_line(&day_key, &line) {
                        Ok(()) => {
                            this.reload();
                            this.status = "Task updated".to_string();
                        }
                        Err(err) => {
                            this.status = format!("Task update failed: {err}");
                        }
                    }
                    cx.notify();
                }))
                .child(div().size(px(8.)).rounded(px(4.)).bg(rgb(0xA3D977)))
                .into_any_element(),
            Some(prefix) => div().child(prefix.to_string()).into_any_element(),
            None => div().into_any_element(),
        }
    }
}

impl Render for NaviApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.cursor_task_started {
            self.cursor_task_started = true;
            cx.spawn_in(window, async move |this, cx| {
                loop {
                    cx.background_executor()
                        .timer(StdDuration::from_millis(520))
                        .await;
                    if this
                        .update(cx, |app, cx| {
                            app.cursor_visible = !app.cursor_visible;
                            cx.notify();
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            })
            .detach();
        }

        let decorations = window.window_decorations();
        let resize_inset = px(8.);
        window.set_client_inset(resize_inset);

        div()
            .id("window-backdrop")
            .bg(transparent_black())
            .size_full()
            .child(
                canvas(
                    |_bounds, window, _cx| {
                        window.insert_hitbox(
                            Bounds::new(
                                point(px(0.), px(0.)),
                                window.window_bounds().get_bounds().size,
                            ),
                            HitboxBehavior::Normal,
                        )
                    },
                    move |_bounds, hitbox, window, _cx| {
                        let mouse = window.mouse_position();
                        let size = window.window_bounds().get_bounds().size;
                        let Some(edge) = resize_edge(mouse, resize_inset, size) else {
                            return;
                        };
                        window.set_cursor_style(resize_cursor(edge), &hitbox);
                    },
                )
                .absolute()
                .size_full(),
            )
            .child(
                div()
                    .size_full()
                    .map(|div| match decorations {
                        Decorations::Server => div,
                        Decorations::Client { .. } => div.p(resize_inset),
                    })
                    .on_mouse_move(|_event, window, _cx| {
                        window.refresh();
                    })
                    .on_mouse_down(MouseButton::Left, move |event, window, _cx| {
                        if let Some(edge) = resize_edge(
                            event.position,
                            resize_inset,
                            window.window_bounds().get_bounds().size,
                        ) {
                            window.start_window_resize(edge);
                        }
                    })
                    .child(
                        div()
                            .track_focus(&self.focus_handle)
                            .key_context("Navi")
                            .on_key_down(cx.listener(|this, event, window, cx| {
                                this.handle_key(event, window, cx);
                            }))
                            .on_mouse_move(cx.listener(
                                |this, event: &gpui::MouseMoveEvent, window, cx| {
                                    if this.split_dragging && event.dragging() {
                                        this.drag_split(event.position.y, window, cx);
                                        cx.stop_propagation();
                                    }
                                },
                            ))
                            .on_mouse_up(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    if this.split_dragging {
                                        this.split_dragging = false;
                                        cx.notify();
                                        cx.stop_propagation();
                                    }
                                }),
                            )
                            .size_full()
                            .flex()
                            .flex_col()
                            .bg(rgb(SURFACE_DARKEST))
                            .text_color(rgb(TEXT_PRIMARY))
                            .border_1()
                            .border_color(rgb(0x222222))
                            .rounded(px(8.))
                            .overflow_hidden()
                            .child(self.render_header(cx))
                            .when(self.collapsed, |div| div.child(self.render_composer()))
                            .when(!self.collapsed, |div| {
                                div.child(self.render_notes(cx))
                                    .child(self.render_splitter(cx))
                                    .child(self.render_chat())
                            })
                            .when_some(self.hover_preview.clone(), |container, preview| {
                                container.child(
                                    div()
                                        .absolute()
                                        .top(px(52.))
                                        .right(px(16.))
                                        .child(render_link_preview(preview.title, preview.summary)),
                                )
                            }),
                    ),
            )
    }
}

impl Focusable for NaviApp {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

fn traffic_button(
    color: u32,
    action: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(format!("traffic-{color:x}"))
        .size(px(12.))
        .rounded(px(6.))
        .bg(rgb(color))
        .cursor_pointer()
        .on_click(move |_event, window, cx| {
            action(window, cx);
        })
}

fn render_link_preview(title: String, summary: String) -> impl IntoElement {
    div()
        .w(px(260.))
        .p(px(8.))
        .rounded(px(2.))
        .bg(rgb(SURFACE_LIGHT))
        .border_1()
        .border_color(rgb(SURFACE_CONTROL))
        .text_size(px(11.))
        .text_color(rgb(TEXT_PRIMARY))
        .child(
            div()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(rgb(ACCENT))
                .child(title),
        )
        .child(div().text_color(rgb(TEXT_SECONDARY)).child(summary))
}

fn resize_edge(pos: Point<Pixels>, inset: Pixels, size: Size<Pixels>) -> Option<ResizeEdge> {
    if pos.y < inset && pos.x < inset {
        Some(ResizeEdge::TopLeft)
    } else if pos.y < inset && pos.x > size.width - inset {
        Some(ResizeEdge::TopRight)
    } else if pos.y < inset {
        Some(ResizeEdge::Top)
    } else if pos.y > size.height - inset && pos.x < inset {
        Some(ResizeEdge::BottomLeft)
    } else if pos.y > size.height - inset && pos.x > size.width - inset {
        Some(ResizeEdge::BottomRight)
    } else if pos.y > size.height - inset {
        Some(ResizeEdge::Bottom)
    } else if pos.x < inset {
        Some(ResizeEdge::Left)
    } else if pos.x > size.width - inset {
        Some(ResizeEdge::Right)
    } else {
        None
    }
}

fn resize_cursor(edge: ResizeEdge) -> CursorStyle {
    match edge {
        ResizeEdge::Top | ResizeEdge::Bottom => CursorStyle::ResizeUpDown,
        ResizeEdge::Left | ResizeEdge::Right => CursorStyle::ResizeLeftRight,
        ResizeEdge::TopLeft | ResizeEdge::BottomRight => CursorStyle::ResizeUpLeftDownRight,
        ResizeEdge::TopRight | ResizeEdge::BottomLeft => CursorStyle::ResizeUpRightDownLeft,
    }
}

fn markdown_prefix(line: &str) -> (Option<String>, &str) {
    if let Some((number, body)) = line.split_once(". ")
        && !number.is_empty()
        && number.chars().all(|ch| ch.is_ascii_digit())
    {
        return (Some(format!("{number}.")), body);
    }

    if let Some(body) = line.strip_prefix("- [ ] ") {
        return (Some("[ ]".to_string()), body);
    }
    if let Some(body) = line.strip_prefix("- [x] ") {
        return (Some("[x]".to_string()), body);
    }
    if let Some(body) = line.strip_prefix("- ") {
        return (Some("-".to_string()), body);
    }
    (None, line)
}

struct ClaudeRun {
    messages: Vec<ChatMessage>,
    session_id: Option<String>,
}

struct ClaudeRunner;

impl ClaudeRunner {
    fn run(
        prompt: String,
        session_id: Option<String>,
        root: PathBuf,
        logs_dir: PathBuf,
    ) -> Result<ClaudeRun> {
        fs::create_dir_all(&root)?;
        fs::create_dir_all(&logs_dir)?;
        let log_path = logs_dir.join(format!(
            "claude_{}.jsonl",
            Local::now().format("%Y%m%d_%H%M%S")
        ));
        let mut log_file = fs::File::create(&log_path)?;
        let model = std::env::var("NAVI_CLAUDE_MODEL").unwrap_or_else(|_| "sonnet".to_string());
        let mut cmd = Command::new("claude");
        cmd.arg("-p")
            .arg("--dangerously-skip-permissions")
            .arg("--output-format=stream-json")
            .arg("--model")
            .arg(model)
            .arg("--verbose")
            .current_dir(&root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        if let Some(session_id) = session_id {
            cmd.arg("--resume").arg(session_id);
        }

        let mut child = cmd.spawn().context("failed to spawn claude CLI")?;
        child
            .stdin
            .take()
            .context("failed to open claude stdin")?
            .write_all(prompt.as_bytes())?;

        let stdout = child
            .stdout
            .take()
            .context("failed to read claude stdout")?;
        let reader = BufReader::new(stdout);
        let mut messages = Vec::new();
        let mut parsed_session_id = None;

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            writeln!(log_file, "{line}")?;
            let parsed = parse_claude_line(&line)?;
            if parsed_session_id.is_none() {
                parsed_session_id = parsed.session_id;
            }
            messages.extend(parsed.messages);
        }

        let status = child.wait()?;
        if !status.success() {
            return Err(anyhow!("claude exited with status {status}"));
        }

        if messages.is_empty() {
            messages.push(ChatMessage {
                role: ChatRole::Assistant,
                text: "Claude finished without user-facing text.".to_string(),
            });
        }

        Ok(ClaudeRun {
            messages,
            session_id: parsed_session_id,
        })
    }
}

#[derive(Default)]
struct ParsedClaudeLine {
    messages: Vec<ChatMessage>,
    session_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClaudeEntry {
    User {
        message: ClaudeUserMessage,
        #[serde(default)]
        session_id: Option<String>,
        #[serde(default, rename = "sessionId")]
        session_id_camel: Option<String>,
    },
    Assistant {
        message: ClaudeAssistantMessage,
        #[serde(default)]
        session_id: Option<String>,
        #[serde(default, rename = "sessionId")]
        session_id_camel: Option<String>,
    },
    System {
        #[serde(default)]
        session_id: Option<String>,
        #[serde(default, rename = "sessionId")]
        session_id_camel: Option<String>,
    },
    #[serde(other)]
    Other,
}

#[derive(Deserialize)]
struct ClaudeUserMessage {
    content: ClaudeUserContent,
}

#[derive(Deserialize)]
struct ClaudeAssistantMessage {
    content: Vec<ClaudeContentBlock>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ClaudeUserContent {
    String(String),
    Blocks(Vec<ClaudeContentBlock>),
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClaudeContentBlock {
    Text {
        text: String,
    },
    #[serde(other)]
    Other,
}

fn parse_claude_line(line: &str) -> Result<ParsedClaudeLine> {
    let entry: ClaudeEntry = serde_json::from_str(line)?;
    let mut parsed = ParsedClaudeLine::default();
    match entry {
        ClaudeEntry::User {
            message,
            session_id,
            session_id_camel,
        } => {
            parsed.session_id = session_id.or(session_id_camel);
            let text = match message.content {
                ClaudeUserContent::String(text) => text,
                ClaudeUserContent::Blocks(blocks) => extract_text(blocks),
            };
            if !text.trim().is_empty() {
                parsed.messages.push(ChatMessage {
                    role: ChatRole::User,
                    text,
                });
            }
        }
        ClaudeEntry::Assistant {
            message,
            session_id,
            session_id_camel,
        } => {
            parsed.session_id = session_id.or(session_id_camel);
            let text = extract_text(message.content);
            if !text.trim().is_empty() {
                parsed.messages.push(ChatMessage {
                    role: ChatRole::Assistant,
                    text,
                });
            }
        }
        ClaudeEntry::System {
            session_id,
            session_id_camel,
        } => {
            parsed.session_id = session_id.or(session_id_camel);
        }
        ClaudeEntry::Other => {}
    }
    Ok(parsed)
}

fn extract_text(blocks: Vec<ClaudeContentBlock>) -> String {
    blocks
        .into_iter()
        .filter_map(|block| match block {
            ClaudeContentBlock::Text { text } => Some(text),
            ClaudeContentBlock::Other => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn build_capture_prompt(input: &str, vault_dir: &Path) -> String {
    format!(
        "You are Navi, a concise daily note and task capture agent. \
         You are running in {}. Update markdown files in this folder and keep task/project data coherent. \
         Use [[yyyymmdd]] day links and [[Project]]/[[Task]] wiki links. \
         Capture this input and ask at most one short clarifying question if needed:\n\n{}",
        vault_dir.display(),
        input
    )
}

fn storage_root() -> PathBuf {
    std::env::var_os("NAVI_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join("navi")))
        .unwrap_or_else(|| PathBuf::from("navi"))
}

#[derive(Debug, Eq, PartialEq)]
enum TextSegment {
    Text(String),
    WikiLink(String),
}

fn split_wiki_links(line: &str) -> Vec<TextSegment> {
    let mut segments = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find("[[") {
        if start > 0 {
            segments.push(TextSegment::Text(rest[..start].to_string()));
        }
        let after = &rest[start + 2..];
        if let Some(end) = after.find("]]") {
            segments.push(TextSegment::WikiLink(after[..end].to_string()));
            rest = &after[end + 2..];
        } else {
            segments.push(TextSegment::Text(rest[start..].to_string()));
            rest = "";
        }
    }
    if !rest.is_empty() {
        segments.push(TextSegment::Text(rest.to_string()));
    }
    segments
}

fn extract_project(text: &str) -> Option<String> {
    let segments = split_wiki_links(text);
    segments.into_iter().find_map(|segment| match segment {
        TextSegment::WikiLink(link) if !link.chars().all(|ch| ch.is_ascii_digit()) => Some(link),
        _ => None,
    })
}

fn summarize(text: &str, max_len: usize) -> String {
    let compact = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.len() <= max_len {
        compact
    } else {
        format!("{}...", &compact[..max_len.saturating_sub(3)])
    }
}

fn day_key(date: NaiveDate) -> String {
    date.format("%Y%m%d").to_string()
}

fn now_iso() -> String {
    Local::now().to_rfc3339()
}

fn run_app() {
    application()
        .with_quit_mode(QuitMode::Explicit)
        .run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(460.), px(680.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: None,
                    window_background: WindowBackgroundAppearance::Transparent,
                    window_decorations: Some(WindowDecorations::Client),
                    kind: WindowKind::PopUp,
                    is_movable: true,
                    is_resizable: true,
                    is_minimizable: true,
                    app_id: Some("dev.navi.capture".to_string()),
                    window_min_size: Some(size(px(360.), px(118.))),
                    ..Default::default()
                },
                |window, cx| cx.new(|cx| NaviApp::new(window, cx)),
            )
            .unwrap();
            cx.activate(true);
        });
}

fn main() {
    run_app();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_only_visible_claude_text() {
        let line = r#"{"type":"assistant","session_id":"abc","message":{"role":"assistant","content":[{"type":"thinking","thinking":"hidden"},{"type":"text","text":"Captured the task."},{"type":"tool_use","name":"Edit","input":{}}]}}"#;
        let parsed = parse_claude_line(line).unwrap();
        assert_eq!(parsed.session_id.as_deref(), Some("abc"));
        assert_eq!(parsed.messages.len(), 1);
        assert_eq!(parsed.messages[0].text, "Captured the task.");
    }

    #[test]
    fn parses_user_text_blocks() {
        let line = r#"{"type":"user","message":{"role":"user","content":[{"type":"text","text":"today I need to ship Navi"},{"type":"tool_result","content":"ignored"}]}}"#;
        let parsed = parse_claude_line(line).unwrap();
        assert_eq!(parsed.messages.len(), 1);
        assert_eq!(parsed.messages[0].text, "today I need to ship Navi");
    }

    #[test]
    fn splits_wiki_links() {
        assert_eq!(
            split_wiki_links("Ship [[Navi]] by [[20260801]]."),
            vec![
                TextSegment::Text("Ship ".to_string()),
                TextSegment::WikiLink("Navi".to_string()),
                TextSegment::Text(" by ".to_string()),
                TextSegment::WikiLink("20260801".to_string()),
                TextSegment::Text(".".to_string()),
            ]
        );
    }

    #[test]
    fn parses_capture_prefixes() {
        let task = CaptureInput::parse("* completed task");
        assert_eq!(task.kind, CaptureKind::Task);
        assert_eq!(task.body, "completed task");

        let bullet = CaptureInput::parse("- project context");
        assert_eq!(bullet.kind, CaptureKind::Bullet);
        assert_eq!(bullet.body, "project context");

        let plain = CaptureInput::parse("quick note");
        assert_eq!(plain.kind, CaptureKind::Plain);
        assert_eq!(plain.body, "quick note");
    }
}
