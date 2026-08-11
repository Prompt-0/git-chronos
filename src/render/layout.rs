#[derive(Debug, Clone)]
pub struct RectLayout {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl RectLayout {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppLayout {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub header_rect: RectLayout,
    pub commit_card_rect: RectLayout,
    pub node_graph_rect: RectLayout,
    pub diff_sparks_rect: RectLayout,
    pub contributors_rect: RectLayout,
    pub footer_rect: RectLayout,
}

impl AppLayout {
    pub fn calculate(width: u32, height: u32) -> Self {
        let margin = 20i32;
        let header_height = 65u32;
        let footer_height = 30u32;

        let right_panel_width = ((width as f32) * 0.28) as u32;
        let left_panel_width = width - (margin as u32) * 3 - right_panel_width;

        let content_y = (margin as u32) + header_height + 15;
        let content_height = height - content_y - footer_height - (margin as u32);

        let commit_card_height = 170u32;
        let node_graph_height = 160u32;
        let diff_sparks_height = content_height - commit_card_height - node_graph_height - 30;

        let header_rect =
            RectLayout::new(margin, margin, width - (margin as u32) * 2, header_height);

        let commit_card_rect = RectLayout::new(
            margin,
            content_y as i32,
            left_panel_width,
            commit_card_height,
        );

        let node_graph_rect = RectLayout::new(
            margin,
            (content_y + commit_card_height + 15) as i32,
            left_panel_width,
            node_graph_height,
        );

        let diff_sparks_rect = RectLayout::new(
            margin,
            (content_y + commit_card_height + 15 + node_graph_height + 15) as i32,
            left_panel_width,
            diff_sparks_height,
        );

        let contributors_rect = RectLayout::new(
            margin + left_panel_width as i32 + margin,
            content_y as i32,
            right_panel_width,
            content_height,
        );

        let footer_rect = RectLayout::new(
            margin,
            (height - footer_height - 10) as i32,
            width - (margin as u32) * 2,
            footer_height,
        );

        Self {
            canvas_width: width,
            canvas_height: height,
            header_rect,
            commit_card_rect,
            node_graph_rect,
            diff_sparks_rect,
            contributors_rect,
            footer_rect,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_layout_calculation() {
        let layout = AppLayout::calculate(1280, 720);
        assert_eq!(layout.canvas_width, 1280);
        assert_eq!(layout.canvas_height, 720);
        assert!(layout.commit_card_rect.width > 0);
        assert!(layout.contributors_rect.width > 0);
    }
}
