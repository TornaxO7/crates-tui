use serde::{Deserialize, Serialize};
use strum::Display;

use crate::app::Mode;

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    KeyRefresh,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    Init,
    Refresh,
    ShowErrorPopup(String),
    ShowInfoPopup(String),
    ClosePopup,
    Help,
    GetCrates,
    SwitchMode(Mode),
    HandleFilterPromptChange,
    IncrementPage,
    DecrementPage,
    ToggleSortBy { reload: bool, forward: bool },
    ScrollBottom,
    ScrollTop,
    ScrollDown,
    ScrollUp,
    ScrollCrateInfoDown,
    ScrollCrateInfoUp,
    SubmitSearch,
    UpdateCurrentSelectionCrateInfo,
    ReloadData,
    ToggleShowHelp,
    ToggleShowCrateInfo,
    StoreTotalNumberOfCrates(u64),
    ClearTaskDetailsHandle(String),
    OpenUrlInBrowser,
    ShowFullCrateInfo,
}
