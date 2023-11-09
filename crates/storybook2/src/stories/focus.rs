use gpui::{
    actions, div, Div, FocusEnabled, Focusable, KeyBinding, ParentElement, Render,
    StatefulInteractivity, StatelessInteractive, Styled, View, VisualContext, WindowContext,
};
use theme2::ActiveTheme;

actions!(ActionA, ActionB, ActionC);

pub struct FocusStory {}

impl FocusStory {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.bind_keys([
            KeyBinding::new("cmd-a", ActionA, Some("parent")),
            KeyBinding::new("cmd-a", ActionB, Some("child-1")),
            KeyBinding::new("cmd-c", ActionC, None),
        ]);

        cx.build_view(move |cx| Self {})
    }
}

impl Render for FocusStory {
    type Element = Div<Self, StatefulInteractivity<Self>, FocusEnabled<Self>>;

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> Self::Element {
        let theme = cx.theme();
        let color_1 = theme.status().created;
        let color_2 = theme.status().modified;
        let color_3 = theme.status().deleted;
        let color_4 = theme.status().conflict;
        let color_5 = theme.status().ignored;
        let color_6 = theme.status().renamed;
        let child_1 = cx.focus_handle();
        let child_2 = cx.focus_handle();

        div()
            .id("parent")
            .focusable()
            .context("parent")
            .on_action(|_, action: &ActionA, cx| {
                println!("Action A dispatched on parent during");
            })
            .on_action(|_, action: &ActionB, cx| {
                println!("Action B dispatched on parent during");
            })
            .on_focus(|_, _, _| println!("Parent focused"))
            .on_blur(|_, _, _| println!("Parent blurred"))
            .on_focus_in(|_, _, _| println!("Parent focus_in"))
            .on_focus_out(|_, _, _| println!("Parent focus_out"))
            .on_key_down(|_, event, phase, _| println!("Key down on parent {:?}", event))
            .on_key_up(|_, event, phase, _| println!("Key up on parent {:?}", event))
            .size_full()
            .bg(color_1)
            .focus(|style| style.bg(color_2))
            .focus_in(|style| style.bg(color_3))
            .child(
                div()
                    .track_focus(&child_1)
                    .context("child-1")
                    .on_action(|_, action: &ActionB, cx| {
                        println!("Action B dispatched on child 1 during");
                    })
                    .w_full()
                    .h_6()
                    .bg(color_4)
                    .focus(|style| style.bg(color_5))
                    .in_focus(|style| style.bg(color_6))
                    .on_focus(|_, _, _| println!("Child 1 focused"))
                    .on_blur(|_, _, _| println!("Child 1 blurred"))
                    .on_focus_in(|_, _, _| println!("Child 1 focus_in"))
                    .on_focus_out(|_, _, _| println!("Child 1 focus_out"))
                    .on_key_down(|_, event, phase, _| println!("Key down on child 1 {:?}", event))
                    .on_key_up(|_, event, phase, _| println!("Key up on child 1 {:?}", event))
                    .child("Child 1"),
            )
            .child(
                div()
                    .track_focus(&child_2)
                    .context("child-2")
                    .on_action(|_, action: &ActionC, cx| {
                        println!("Action C dispatched on child 2 during");
                    })
                    .w_full()
                    .h_6()
                    .bg(color_4)
                    .on_focus(|_, _, _| println!("Child 2 focused"))
                    .on_blur(|_, _, _| println!("Child 2 blurred"))
                    .on_focus_in(|_, _, _| println!("Child 2 focus_in"))
                    .on_focus_out(|_, _, _| println!("Child 2 focus_out"))
                    .on_key_down(|_, event, phase, _| println!("Key down on child 2 {:?}", event))
                    .on_key_up(|_, event, phase, _| println!("Key up on child 2 {:?}", event))
                    .child("Child 2"),
            )
    }
}