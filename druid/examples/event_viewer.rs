// Copyright 2020 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use druid::widget::{
    prelude::*, Container, Controller, Flex, Label, List, Painter, Scroll, SizedBox, TextBox,
};
use druid::{
    AppLauncher, Code, Color, Data, Env, KbKey, KeyEvent, Lens, LocalizedString, Location,
    Modifiers, MouseButton, MouseEvent, Point, Widget, WidgetExt, WindowDesc,
};
use std::sync::Arc;

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Hello World!");
const INACTIVE_AREA_COLOR: Color = Color::grey8(0x55);
const HOVER_AREA_COLOR: Color = Color::grey8(0xAA);
const ACTIVE_AREA_COLOR: Color = Color::grey8(0xCC);

#[derive(Clone, Data, Lens)]
struct AppState {
    text_input: String,
    events: Arc<Vec<EventL>>,
}

pub fn main() {
    //describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    //create the initial app state
    let initial_state = AppState {
        text_input: String::new(),
        events: Arc::new(Vec::new()),
    };

    //start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn interactive_area() -> impl Widget<AppState> {
    Flex::row()
        .with_flex_child(
            TextBox::multiline()
                .lens(AppState::text_input)
                .controller(EventLogger::new(
                    |event| matches!(event, Event::KeyDown(_) | Event::KeyUp(_)),
                ))
                .expand()
                .padding(10.0),
            1.0,
        )
        .with_flex_child(
            Container::new(SizedBox::empty())
                .background(Painter::new(|ctx, _, _env| {
                    let bg_color = if ctx.is_active() {
                        ACTIVE_AREA_COLOR
                    } else if ctx.is_hot() {
                        HOVER_AREA_COLOR
                    } else {
                        INACTIVE_AREA_COLOR
                    };
                    let rect = ctx.size().to_rect();
                    ctx.fill(rect, &bg_color);
                }))
                .rounded(5.0)
                .border(Color::grey8(0xCC), 1.0)
                .lens(AppState::text_input)
                .controller(EventLogger::new(
                    |event| matches!(event, Event::MouseDown(_) | Event::MouseUp(_)),
                ))
                .expand()
                .padding(10.0),
            1.0,
        )
}

fn event_list() -> impl Widget<AppState> {
    Scroll::new(List::new(make_list_item).lens(AppState::events))
        .vertical()
        .expand_width()
}

fn make_list_item() -> Box<dyn Widget<EventL>> {
    Box::new(
        Flex::row()
            .with_child(
                Label::dynamic(|d: &EventL, _| d.name())
                    .with_text_size(12.0)
                    .fix_width(80.0),
            )
            .with_child(
                Label::dynamic(|d: &EventL, _| {
                    if let Some(pt) = d.mouse_pos() {
                        format!("{:.2}", pt)
                    } else {
                        "".to_string()
                    }
                })
                .with_text_size(12.0)
                .fix_width(80.0),
            )
            .with_child(
                Label::dynamic(|d: &EventL, _| {
                    if let Some(b) = d.mouse_button() {
                        format!("{:?}", b)
                    } else {
                        "".to_string()
                    }
                })
                .with_text_size(12.0)
                .fix_width(60.0),
            )
            .with_child(
                Label::dynamic(|d: &EventL, _| {
                    if let Some(b) = d.click_count() {
                        format!("{}", b)
                    } else {
                        "".to_string()
                    }
                })
                .with_text_size(12.0)
                .fix_width(16.0),
            )
            .with_child(
                Label::dynamic(|d: &EventL, _| {
                    if let Some(b) = d.key() {
                        format!("{}", b)
                    } else {
                        "".to_string()
                    }
                })
                .with_text_size(12.0)
                .fix_width(40.0),
            )
            .with_child(
                Label::dynamic(|d: &EventL, _| {
                    if let Some(b) = d.code() {
                        format!("{}", b)
                    } else {
                        "".to_string()
                    }
                })
                .with_text_size(12.0)
                .fix_width(26.0),
            ),
    )
}

fn build_root_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_flex_child(interactive_area().padding(10.0), 1.0)
        .with_flex_child(event_list().padding(10.0), 1.0)
}

#[derive(Debug, Clone)]
enum LoggedEvent {
    KeyDown(KeyEvent),
    KeyUp(KeyEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
}

#[derive(Debug, Clone, Copy, Data, PartialEq)]
enum EventT {
    KeyDown,
    KeyUp,
    MouseDown,
    MouseUp,
}

#[derive(Debug, Clone, Data)]
struct EventL {
    typ: EventT,
    #[data(ignore)]
    mouse: Option<MouseEvent>,
    #[data(ignore)]
    key: Option<KeyEvent>,
}

impl EventL {
    fn try_from_event(event: &Event) -> Option<Self> {
        let to_log = match event {
            Event::MouseUp(mouse) => Some((EventT::MouseUp, Some(mouse.clone()), None)),
            Event::MouseDown(mouse) => Some((EventT::MouseDown, Some(mouse.clone()), None)),
            Event::KeyUp(mouse) => Some((EventT::KeyUp, None, Some(mouse.clone()))),
            Event::KeyDown(mouse) => Some((EventT::KeyDown, None, Some(mouse.clone()))),
            _ => None,
        };

        to_log.map(|(typ, mouse, key)| EventL { typ, mouse, key })
    }

    fn name(&self) -> String {
        match self.typ {
            EventT::KeyDown => "KeyDown",
            EventT::KeyUp => "KeyUp",
            EventT::MouseDown => "MouseDown",
            EventT::MouseUp => "MouseUp",
        }
        .to_string()
    }

    fn mouse_pos(&self) -> Option<Point> {
        self.mouse.as_ref().map(|m| m.pos)
    }

    fn mouse_button(&self) -> Option<MouseButton> {
        self.mouse.as_ref().map(|m| m.button)
    }

    fn click_count(&self) -> Option<u8> {
        self.mouse.as_ref().map(|m| m.count)
    }

    fn key(&self) -> Option<KbKey> {
        self.key.as_ref().map(|k| k.key.clone())
    }

    fn code(&self) -> Option<Code> {
        self.key.as_ref().map(|k| k.code)
    }

    fn location(&self) -> Option<Location> {
        self.key.as_ref().map(|k| k.location)
    }

    fn repeat(&self) -> Option<bool> {
        self.key.as_ref().map(|k| k.repeat)
    }
}

struct EventLogger {
    filter: Box<dyn Fn(&Event) -> bool>,
}

impl EventLogger {
    fn new(f: impl Fn(&Event) -> bool + 'static) -> Self {
        EventLogger {
            filter: Box::new(f),
        }
    }
}

impl<W: Widget<AppState>> Controller<AppState, W> for EventLogger {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        if (self.filter)(event) {
            data.log_event(event);
        }
        child.event(ctx, event, data, env)
    }
}

impl AppState {
    fn log_event(&mut self, event: &Event) {
        if let Some(to_log) = EventL::try_from_event(event) {
            Arc::make_mut(&mut self.events).push(to_log);
        }
    }
}
