use std::collections::HashMap;

use eyre::{ensure, Result};
use ratatui::layout::Rect;

use crate::window::pane::{Pane, PaneFlattened, PaneManager, PaneView, Point};

pub const SMALL_SCREEN_STR: &str = "Defualt (Small)";
pub const LARGE_SCREEN_STR: &str = "Defualt (Large)";

/// Trace the focus, to ensure the pane switching is backed by a state machine.
pub struct ScreenManager {
    pub panes: HashMap<String, PaneManager>,
    pub current_pane: String,
    pub use_default_pane: bool,
    pub full_screen: bool,
    pub pending_mouse_move: Option<Point>,
}

impl ScreenManager {
    pub fn new() -> Result<Self> {
        let mut manager = Self {
            panes: HashMap::new(),
            current_pane: String::new(),
            full_screen: false,
            use_default_pane: true,
            pending_mouse_move: None,
        };

        manager.add_pane_manager(SMALL_SCREEN_STR, PaneManager::default_small_screen()?);
        manager.add_pane_manager(LARGE_SCREEN_STR, PaneManager::default_large_screen()?);
        manager.current_pane = SMALL_SCREEN_STR.to_string();

        Ok(manager)
    }

    pub fn get_focused_pane(&mut self) -> Result<&mut Pane> {
        self.get_current_pane_mut()?.get_focused_pane_mut()
    }

    pub fn get_focused_view(&mut self) -> Result<PaneView> {
        self.get_current_pane_mut()?.get_focused_view()
    }

    pub fn get_available_pane_profiles(&self) -> Vec<String> {
        self.panes.keys().cloned().collect()
    }

    pub fn use_default_pane_profile(&mut self, use_default_pane: bool) {
        self.use_default_pane = use_default_pane;
    }

    pub fn add_pane_manager(&mut self, name: &str, manager: PaneManager) {
        self.panes.insert(name.to_string(), manager);
    }

    pub fn toggle_full_screen(&mut self) {
        self.full_screen = !self.full_screen;
    }

    pub fn set_mouse_move(&mut self, x: u16, y: u16) {
        if self.full_screen {
            // Ignore mouse move in full screen mode.
            return;
        }
        self.pending_mouse_move = Some(Point::new(x, y));
    }

    pub fn get_current_pane(&self) -> Result<&PaneManager> {
        self.panes.get(&self.current_pane).ok_or(eyre::eyre!("No current pane"))
    }

    pub fn get_current_pane_mut(&mut self) -> Result<&mut PaneManager> {
        self.panes.get_mut(&self.current_pane).ok_or(eyre::eyre!("No current pane"))
    }

    pub fn enter_terminal(&mut self) -> Result<()> {
        ensure!(!self.full_screen, "Cannot enter terminal in full screen mode");
        self.get_current_pane_mut()?.force_goto_by_view(PaneView::Terminal)?;

        Ok(())
    }

    pub fn set_pane(&mut self, name: &str) -> Result<()> {
        if self.panes.contains_key(name) {
            self.current_pane = name.to_string();
            Ok(())
        } else {
            Err(eyre::eyre!("No such pane"))
        }
    }

    pub fn set_large_screen(&mut self) {
        self.current_pane = LARGE_SCREEN_STR.to_string();
    }

    pub fn set_small_screen(&mut self) {
        self.current_pane = SMALL_SCREEN_STR.to_string();
    }

    pub fn focus_up(&mut self) -> Result<()> {
        self.get_current_pane_mut()?.focus_up()
    }

    pub fn focus_down(&mut self) -> Result<()> {
        self.get_current_pane_mut()?.focus_down()
    }

    pub fn focus_left(&mut self) -> Result<()> {
        self.get_current_pane_mut()?.focus_left()
    }

    pub fn focus_right(&mut self) -> Result<()> {
        self.get_current_pane_mut()?.focus_right()
    }

    pub fn get_flattened_layout(&self, app: Rect) -> Result<Vec<PaneFlattened>> {
        if self.full_screen {
            let pane = self.get_current_pane()?.get_focused_pane()?;
            Ok(vec![PaneFlattened {
                view: pane.get_current_view(),
                id: pane.id,
                focused: true,
                rect: app,
            }])
        } else {
            Ok(self.get_current_pane()?.get_flattened_layout(app)?)
        }
    }
}
