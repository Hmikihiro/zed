use crate::TerminalView;
use gpui::{
    elements::*, AppContext, Entity, ModelHandle, Subscription, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use project::Project;
use settings::{Settings, WorkingDirectory};
use util::ResultExt;
use workspace::{dock::Panel, pane, DraggedItem, Pane, Workspace};

pub enum Event {
    Close,
}

pub struct TerminalPanel {
    project: ModelHandle<Project>,
    pane: ViewHandle<Pane>,
    workspace: WeakViewHandle<Workspace>,
    _subscription: Subscription,
}

impl TerminalPanel {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        let pane = cx.add_view(|cx| {
            let window_id = cx.window_id();
            let mut pane = Pane::new(
                workspace.weak_handle(),
                workspace.app_state().background_actions,
                cx,
            );
            pane.on_can_drop(move |drag_and_drop, cx| {
                drag_and_drop
                    .currently_dragged::<DraggedItem>(window_id)
                    .map_or(false, |(_, item)| {
                        item.handle.act_as::<TerminalView>(cx).is_some()
                    })
            });
            pane
        });
        let subscription = cx.subscribe(&pane, Self::handle_pane_event);
        Self {
            project: workspace.project().clone(),
            pane,
            workspace: workspace.weak_handle(),
            _subscription: subscription,
        }
    }

    fn handle_pane_event(
        &mut self,
        _pane: ViewHandle<Pane>,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            pane::Event::Remove => cx.emit(Event::Close),
            _ => {}
        }
    }
}

impl Entity for TerminalPanel {
    type Event = Event;
}

impl View for TerminalPanel {
    fn ui_name() -> &'static str {
        "TerminalPanel"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> gpui::AnyElement<Self> {
        ChildView::new(&self.pane, cx).into_any()
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if self.pane.read(cx).items_len() == 0 {
            if let Some(workspace) = self.workspace.upgrade(cx) {
                let working_directory_strategy = cx
                    .global::<Settings>()
                    .terminal_overrides
                    .working_directory
                    .clone()
                    .unwrap_or(WorkingDirectory::CurrentProjectDirectory);
                let working_directory = crate::get_working_directory(
                    workspace.read(cx),
                    cx,
                    working_directory_strategy,
                );
                let window_id = cx.window_id();
                if let Some(terminal) = self.project.update(cx, |project, cx| {
                    project
                        .create_terminal(working_directory, window_id, cx)
                        .log_err()
                }) {
                    workspace.update(cx, |workspace, cx| {
                        let terminal = Box::new(cx.add_view(|cx| {
                            TerminalView::new(terminal, workspace.database_id(), cx)
                        }));
                        Pane::add_item(workspace, &self.pane, terminal, true, true, None, cx);
                    });
                }
            }
        }
    }
}

impl Panel for TerminalPanel {
    fn should_close_on_event(&self, event: &Event, _: &AppContext) -> bool {
        matches!(event, Event::Close)
    }
}
