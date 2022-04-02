use async_trait::async_trait;
use portable_pty::{Child, MasterPty};
use wezterm_term::Terminal;

use crate::{
    domain::DomainId,
    localpane::LocalPane,
    pane::{Pane, PaneId},
};

/// A TmuxPane act as a localpane except that we take effort to handle
/// history scrollback and sync with remote.
pub struct TmuxPane {
    history_size: u64,
    localpane: LocalPane,
}

#[async_trait(?Send)]
impl Pane for TmuxPane {
    fn pane_id(&self) -> crate::pane::PaneId {
        self.localpane.pane_id
    }

    fn get_cursor_position(&self) -> crate::renderable::StableCursorPosition {
        self.localpane.get_cursor_position()
    }

    fn get_current_seqno(&self) -> termwiz::surface::SequenceNo {
        self.localpane.get_current_seqno()
    }

    fn get_changed_since(
        &self,
        lines: std::ops::Range<wezterm_term::StableRowIndex>,
        seqno: termwiz::surface::SequenceNo,
    ) -> rangeset::RangeSet<wezterm_term::StableRowIndex> {
        self.localpane.get_changed_since(lines, seqno)
    }

    fn get_lines(
        &self,
        lines: std::ops::Range<wezterm_term::StableRowIndex>,
    ) -> (wezterm_term::StableRowIndex, Vec<wezterm_term::Line>) {
        // TODO: if get_lines out of fetched history, we need to perform
        // a capturep and use a clean terminal to render those lines, then swap
        // into current history lines
        self.localpane.get_lines(lines)
    }

    fn get_dimensions(&self) -> crate::renderable::RenderableDimensions {
        self.localpane.get_dimensions()
    }

    fn get_title(&self) -> String {
        self.localpane.get_title()
    }

    fn send_paste(&self, text: &str) -> anyhow::Result<()> {
        self.localpane.send_paste(text)
    }

    fn reader(&self) -> anyhow::Result<Option<Box<dyn std::io::Read + Send>>> {
        self.localpane.reader()
    }

    fn writer(&self) -> std::cell::RefMut<dyn std::io::Write> {
        self.localpane.writer()
    }

    fn resize(&self, size: portable_pty::PtySize) -> anyhow::Result<()> {
        self.localpane.resize(size)
    }

    fn key_down(
        &self,
        key: wezterm_term::KeyCode,
        mods: wezterm_term::KeyModifiers,
    ) -> anyhow::Result<()> {
        self.localpane.key_down(key, mods)
    }

    fn key_up(
        &self,
        key: wezterm_term::KeyCode,
        mods: wezterm_term::KeyModifiers,
    ) -> anyhow::Result<()> {
        self.localpane.key_up(key, mods)
    }

    fn mouse_event(&self, event: wezterm_term::MouseEvent) -> anyhow::Result<()> {
        self.localpane.mouse_event(event)
    }

    fn is_dead(&self) -> bool {
        self.localpane.is_dead()
    }

    fn palette(&self) -> wezterm_term::color::ColorPalette {
        self.localpane.palette()
    }

    fn domain_id(&self) -> crate::domain::DomainId {
        self.localpane.domain_id()
    }

    fn is_mouse_grabbed(&self) -> bool {
        self.localpane.is_mouse_grabbed()
    }

    fn is_alt_screen_active(&self) -> bool {
        self.localpane.is_alt_screen_active()
    }

    fn get_current_working_dir(&self) -> Option<url::Url> {
        self.localpane.get_current_working_dir()
    }
}

impl TmuxPane {
    pub fn new(
        pane_id: PaneId,
        terminal: Terminal,
        process: Box<dyn Child + Send>,
        pty: Box<dyn MasterPty>,
        domain_id: DomainId,
        history_size: u64,
    ) -> Self {
        Self {
            history_size,
            localpane: LocalPane::new(pane_id, terminal, process, pty, domain_id),
        }
    }
}
