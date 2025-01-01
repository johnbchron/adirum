use radix_colors_rs::*;

macro_rules! radix_to_ratatui_const {
  ($radix_color:ident, $ratatui_color:ident) => {
    pub const $ratatui_color: ratatui::style::Color =
      ratatui::style::Color::Rgb(
        $radix_color.r,
        $radix_color.g,
        $radix_color.b,
      );
  };
}

radix_to_ratatui_const!(SANDDARK_SAND1, BASE_1_RATATUI);
radix_to_ratatui_const!(SANDDARK_SAND2, BASE_2_RATATUI);
radix_to_ratatui_const!(SANDDARK_SAND3, BASE_3_RATATUI);
radix_to_ratatui_const!(SANDDARK_SAND4, BASE_4_RATATUI);
radix_to_ratatui_const!(SANDDARK_SAND5, BASE_5_RATATUI);
radix_to_ratatui_const!(SANDDARK_SAND6, BASE_6_RATATUI);
radix_to_ratatui_const!(SANDDARK_SAND7, BASE_7_RATATUI);
radix_to_ratatui_const!(SANDDARK_SAND8, BASE_8_RATATUI);
radix_to_ratatui_const!(SANDDARK_SAND9, BASE_9_RATATUI);
radix_to_ratatui_const!(SANDDARK_SAND10, BASE_10_RATATUI);
radix_to_ratatui_const!(SANDDARK_SAND11, BASE_11_RATATUI);
radix_to_ratatui_const!(SANDDARK_SAND12, BASE_12_RATATUI);

radix_to_ratatui_const!(AMBERDARK_AMBER1, PRIMARY_1_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER2, PRIMARY_2_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER3, PRIMARY_3_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER4, PRIMARY_4_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER5, PRIMARY_5_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER6, PRIMARY_6_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER7, PRIMARY_7_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER8, PRIMARY_8_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER9, PRIMARY_9_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER10, PRIMARY_10_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER11, PRIMARY_11_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER12, PRIMARY_12_RATATUI);

pub use BASE_1_RATATUI as BASE_COLOR_RATATUI;
pub use BASE_2_RATATUI as BACKGROUND_COLOR_RATATUI;
pub use BASE_7_RATATUI as NORMAL_BORDER_COLOR_RATATUI;
pub use BASE_11_RATATUI as DIM_TEXT_COLOR_RATATUI;
pub use BASE_12_RATATUI as NORMAL_TEXT_COLOR_RATATUI;
pub use PRIMARY_7_RATATUI as ACTIVE_BORDER_COLOR_RATATUI;
pub use PRIMARY_11_RATATUI as PUNCHY_TEXT_COLOR_RATATUI;
pub use PRIMARY_11_RATATUI as TITLE_COLOR_RATATUI;
