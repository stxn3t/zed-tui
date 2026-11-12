use gpui::{AnyElement, FontWeight};

use crate::prelude::*;

/// Height of the row that holds a [`TuiFrame`]'s title. The frame's top line runs through
/// the middle of this row, so it must be even for the line to land on a whole pixel.
pub const TUI_FRAME_TITLE_HEIGHT: Pixels = px(16.);
const TITLE_LINE_OFFSET: Pixels = px(8.);
const LEAD_IN_WIDTH: Pixels = px(8.);

/// A box with a 1px border whose title interrupts the top border, in the style of
/// terminal UIs: `┌─ title ───── end title ─┐`.
#[derive(IntoElement)]
pub struct TuiFrame {
    title: SharedString,
    end_title: Option<SharedString>,
    focused: bool,
    fill: bool,
    content: Option<AnyElement>,
}

impl TuiFrame {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            end_title: None,
            focused: false,
            fill: true,
            content: None,
        }
    }

    /// Text drawn at the right end of the top border, such as a match count.
    pub fn end_title(mut self, end_title: impl Into<Option<SharedString>>) -> Self {
        self.end_title = end_title.into();
        self
    }

    /// Draws the frame and its title in the accent color.
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Sizes the frame to its content instead of filling its parent, for frames inside
    /// content-sized containers such as modals.
    pub fn fit_content(mut self) -> Self {
        self.fill = false;
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

impl RenderOnce for TuiFrame {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = cx.theme().colors();
        let frame_color = if self.focused {
            colors.panel_focused_border
        } else {
            colors.border
        };
        let label_color = if self.focused {
            Color::Accent
        } else {
            Color::Muted
        };
        let line = |width: Option<Pixels>| {
            div()
                .h(px(1.))
                .mt(TITLE_LINE_OFFSET)
                .bg(frame_color)
                .map(|this| match width {
                    Some(width) => this.w(width).flex_none(),
                    None => this.flex_1(),
                })
        };
        let title_label = |text: SharedString, bold: bool| {
            h_flex().flex_none().px_1().child(
                Label::new(text)
                    .size(LabelSize::Small)
                    .color(label_color)
                    .when(bold, |label| label.weight(FontWeight::BOLD)),
            )
        };

        // The body has no top border; the title row draws the top edge as line segments
        // so the line stops on either side of the title instead of running through it.
        let title_row = h_flex()
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .h(TUI_FRAME_TITLE_HEIGHT)
            .items_start()
            .child(line(Some(LEAD_IN_WIDTH)))
            .child(title_label(self.title, self.focused).h(TUI_FRAME_TITLE_HEIGHT))
            .child(line(None))
            .when_some(self.end_title, |this, end_title| {
                this.child(title_label(end_title, false).h(TUI_FRAME_TITLE_HEIGHT))
                    .child(line(Some(LEAD_IN_WIDTH)))
            });

        div()
            .relative()
            .map(|this| {
                if self.fill {
                    this.size_full()
                } else {
                    this.w_full()
                }
            })
            .pt(TITLE_LINE_OFFSET)
            .child(
                v_flex()
                    .map(|this| {
                        if self.fill {
                            this.size_full()
                        } else {
                            this.w_full()
                        }
                    })
                    .min_h_0()
                    .pt(TUI_FRAME_TITLE_HEIGHT - TITLE_LINE_OFFSET)
                    .border_l_1()
                    .border_r_1()
                    .border_b_1()
                    .border_color(frame_color)
                    .overflow_hidden()
                    .children(self.content),
            )
            .child(title_row)
    }
}
