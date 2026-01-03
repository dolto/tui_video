use ratatui::text::Line;
use ratatui::style::{Color, Style};

const TABLE: &[u8] = b" .:-=+*#%@";

pub fn rgb_to_colored_ascii(
    rgb: &[u8],
    width: usize,
    height: usize,
    out: &mut Vec<Line>,
) {
    out.clear();

    let mut y = 0;
    while y + 1 < height {
        let mut spans = Vec::with_capacity(width);

        for x in 0..width {
            let i1 = (y * width + x) * 3;
            let i2 = ((y + 1) * width + x) * 3;

            let r = ((rgb[i1] as u16 + rgb[i2] as u16) / 2) as u8;
            let g = ((rgb[i1 + 1] as u16 + rgb[i2 + 1] as u16) / 2) as u8;
            let b = ((rgb[i1 + 2] as u16 + rgb[i2 + 2] as u16) / 2) as u8;

            let luma =
                (0.2126 * r as f32 +
                    0.7152 * g as f32 +
                    0.0722 * b as f32) as usize;

            let idx = luma * (TABLE.len() - 1) / 255;
            let ch = TABLE[idx] as char;

            spans.push(
                ratatui::text::Span::styled(
                    ch.to_string(),
                    Style::default().fg(Color::Rgb(r, g, b)),
                )
            );
        }

        out.push(Line::from(spans));
        y += 2;
    }
}
