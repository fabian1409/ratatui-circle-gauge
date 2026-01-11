use std::f64::consts::PI;

use ratatui_core::{
    buffer::Buffer, layout::Rect, style::Style, symbols::braille::BRAILLE, text::Span,
    widgets::Widget,
};

const TWO_PI: f64 = 2.0 * PI;

pub struct CircleGauge<'a> {
    ratio: f64, // 0.0..=1.0
    fill_style: Style,
    empty_style: Option<Style>,
    start_angle: f64, // in radians
    stroke: f64,
    label: Option<Span<'a>>,
}

impl Default for CircleGauge<'_> {
    fn default() -> Self {
        Self {
            ratio: 0.0,
            fill_style: Style::default(),
            empty_style: None,
            start_angle: PI / 2.0,
            stroke: 2.0,
            label: None,
        }
    }
}

impl<'a> CircleGauge<'a> {
    pub fn ratio(mut self, ratio: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&ratio),
            "Ratio should be between 0 and 1 inclusively."
        );
        self.ratio = ratio;
        self
    }

    pub fn percent(mut self, percent: u16) -> Self {
        assert!(
            percent <= 100,
            "Percentage should be between 0 and 100 inclusively."
        );
        self.ratio = f64::from(percent) / 100.0;
        self
    }

    pub fn angle(mut self, angle: f64) -> Self {
        assert!(
            (0.0..=TWO_PI).contains(&angle),
            "Angle should be between 0 and 2π inclusively."
        );
        self.ratio = angle / TWO_PI;
        self
    }

    pub fn start_angle(mut self, angle: f64) -> Self {
        assert!(
            (0.0..=TWO_PI).contains(&angle),
            "Start angle should be between 0 and 2π inclusively."
        );
        self.start_angle = angle;
        self
    }

    pub fn fill_style(mut self, style: Style) -> Self {
        self.fill_style = style;
        self
    }

    pub fn empty_style(mut self, style: Style) -> Self {
        self.empty_style = Some(style);
        self
    }

    pub fn stroke(mut self, stroke: f64) -> Self {
        self.stroke = stroke;
        self
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<Span<'a>>,
    {
        self.label = Some(label.into());
        self
    }
}

impl Widget for CircleGauge<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let end_angle = self.ratio * TWO_PI;

        let px_w = area.width as usize * 2;
        let px_h = area.height as usize * 4;

        let cx = px_w as f64 / 2.0;
        let cy = px_h as f64 / 2.0;

        let radius = cx.min(cy);
        let stroke = self.stroke.clamp(0.0, radius);

        for cell_y in 0..area.height {
            for cell_x in 0..area.width {
                let cell = &mut buf[(area.x + cell_x, area.y + cell_y)];

                let mut bits = 0u8;
                let mut fill = false;

                for dy in 0..4 {
                    for dx in 0..2 {
                        let px = cell_x as usize * 2 + dx;
                        let py = cell_y as usize * 4 + dy;

                        let fx = px as f64 + 0.5 - cx;
                        let fy = cy - (py as f64 + 0.5);

                        let dist = (fx * fx + fy * fy).sqrt();

                        if dist <= radius && dist >= radius - stroke {
                            let bit_index = dy * 2 + dx;
                            bits |= 1 << bit_index;

                            let angle = (self.start_angle - fy.atan2(fx)).rem_euclid(TWO_PI);

                            if angle <= end_angle {
                                fill = true;
                            }
                        }
                    }
                }

                if bits != 0 {
                    let ch = BRAILLE[bits as usize];
                    if fill {
                        cell.set_char(ch).set_style(self.fill_style);
                    } else if let Some(empty_style) = self.empty_style {
                        cell.set_char(ch).set_style(empty_style);
                    }
                }
            }
        }

        let default_label = Span::raw(format!("{}%", f64::round(self.ratio * 100.0)));
        let label = self.label.unwrap_or(default_label);
        let label_width = label.width() as u16;
        let x = area.x + (area.width.saturating_sub(label_width)) / 2;
        let y = area.y + area.height / 2;
        buf.set_span(x, y, &label, area.width.min(label_width));
    }
}
