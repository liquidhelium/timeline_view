use egui::{Id,Sense,Layout};
mod style;
pub use style::Style;

pub struct TrackView<'a, T> {
    track_datas: &'a mut Vec<T>,
    id_source: Option<Id>,
    header_width: Option<f32>,
    spliter_draggable: bool,
    style: style::Style,
}

impl<'a, T> TrackView<'a, T> {
    pub fn new(tracks: &'a mut Vec<T>) -> Self {
        Self {
            track_datas: tracks,
            id_source: None,
            header_width: None,
            style: Default::default(),
            spliter_draggable: true,
        }
    }
    pub fn id_source(mut self, id_source: impl std::hash::Hash) -> Self {
        self.id_source = Some(Id::new(id_source));
        self
    }
    pub fn header_width(mut self, header_width: f32) -> Self {
        self.header_width = Some(header_width);
        self
    }
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    pub fn spliter_draggable(mut self, value: bool) -> Self {
        self.spliter_draggable = value;
        self
    }

    pub fn show(
        self,
        ui: &mut egui::Ui,
        on_header: impl Fn(&mut egui::Ui, &mut T),
        on_tracks: impl Fn(&mut egui::Ui, &mut T),
    ) {
        let id_source = self.id_source.unwrap_or("timeline".into());
        let TrackViewState {
            mut track_area_state,
            mut header_width,
            drag_current_x,
        } = ui
            .ctx()
            .data()
            .get_temp(id_source)
            .unwrap_or(TrackViewState {
                header_width: self.style.default_header_width,
                ..Default::default()
            });
        let view_height = ui.available_height();
        ui.horizontal(|ui| {
            let header_size = [header_width, view_height];
            let y_offset = track_area_state.offset.y;
            let header_rect = ui.allocate_space(header_size.into()).1;
            let mut header_ui = ui.child_ui(
                header_rect.translate([0.0, -y_offset].into()),
                Layout::top_down(egui::Align::Min),
            );
            header_ui.set_clip_rect(header_rect);
            let spliter = ui
                .allocate_response(
                    [self.style.spliter_width, view_height].into(),
                    Sense::click_and_drag(),
                )
                .on_hover_cursor(egui::CursorIcon::ResizeColumn);
            ui.painter()
                .rect_filled(spliter.rect, 0.0, self.style.spliter_color);
            if spliter.dragged() && self.spliter_draggable {
                header_width += spliter.drag_delta().x;
            }
            track_area_state = egui::ScrollArea::new([true, true])
                .auto_shrink([false, false])
                .hscroll(true)
                .vscroll(true)
                .id_source(id_source.with("tracks"))
                .show(ui, |ui| {
                    let width = ui.available_width();
                    ui.vertical(|ui| {
                        for (i, data) in self.track_datas.iter_mut().enumerate() {
                            ui.push_id(i, |ui| {
                                let height = header_ui
                                    .push_id(i, |ui| {
                                        Self::warp_header(
                                            &self.style,
                                            header_width,
                                            ui,
                                            data,
                                            &on_header,
                                        )
                                    })
                                    .inner;
                                Self::wrap_track(&self.style, height,width, ui, data, &on_tracks);
                            });
                        }
                    });
                })
                .state;
            ui.ctx().data().insert_temp(
                id_source,
                TrackViewState {
                    track_area_state,
                    drag_current_x,
                    header_width,
                },
            )
        });
    }
    fn warp_header(
        style: &Style,
        width: f32,
        ui: &mut egui::Ui,
        data: &mut T,
        func: &impl Fn(&mut egui::Ui, &mut T),
    ) -> f32 {
        let mut frame = egui::Frame::none()
            .fill(style.default_header_bgcolor)
            .stroke(style.default_header_stoke)
            .inner_margin(style.margin_between_tracks)
            .begin(ui);
        let ret = frame
            .content_ui
            .allocate_ui([width, style.default_track_height].into(), |ui| {
                func(ui, data);
                ui.allocate_space(
                    [
                        ui.available_size().x - 2.0 * style.margin_between_tracks,
                        ui.available_size().y,
                    ]
                    .into(),
                );
                ui.min_rect().height()
            })
            .inner;
        frame.end(ui);
        ret
    }
    fn wrap_track(
        style: &Style,
        height: f32,
        width: f32,
        ui: &mut egui::Ui,
        data: &mut T,
        func: &impl Fn(&mut egui::Ui, &mut T),
    ) ->f32{
        let mut frame = egui::Frame::none()
            .fill(style.default_track_bgcolor)
            .stroke(style.default_track_stoke)
            .inner_margin(style.margin_between_tracks)
            .begin(ui);
        let ret = frame.content_ui.allocate_ui_with_layout(
            [width, height].into(),
            Layout::left_to_right(egui::Align::Center),
            |ui| {
                func(ui, data);
                ui.allocate_space(ui.available_size());
                ui.min_rect().height()
            },
        ).inner;
        frame.end(ui);
        ret
    }
}

#[derive(Clone, Copy, Default)]
pub struct TrackViewState {
    track_area_state: egui::scroll_area::State,
    drag_current_x: f32,
    header_width: f32,
}