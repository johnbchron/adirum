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

radix_to_ratatui_const!(AMBERDARK_AMBER1, COLOR_1_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER2, COLOR_2_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER3, COLOR_3_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER4, COLOR_4_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER5, COLOR_5_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER6, COLOR_6_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER7, COLOR_7_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER8, COLOR_8_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER9, COLOR_9_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER10, COLOR_10_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER11, COLOR_11_RATATUI);
radix_to_ratatui_const!(AMBERDARK_AMBER12, COLOR_12_RATATUI);

// radix_to_ratatui_const!(ORANGEDARK_ORANGE1, COLOR_1_RATATUI);
// radix_to_ratatui_const!(ORANGEDARK_ORANGE2, COLOR_2_RATATUI);
// radix_to_ratatui_const!(ORANGEDARK_ORANGE3, COLOR_3_RATATUI);
// radix_to_ratatui_const!(ORANGEDARK_ORANGE4, COLOR_4_RATATUI);
// radix_to_ratatui_const!(ORANGEDARK_ORANGE5, COLOR_5_RATATUI);
// radix_to_ratatui_const!(ORANGEDARK_ORANGE6, COLOR_6_RATATUI);
// radix_to_ratatui_const!(ORANGEDARK_ORANGE7, COLOR_7_RATATUI);
// radix_to_ratatui_const!(ORANGEDARK_ORANGE8, COLOR_8_RATATUI);
// radix_to_ratatui_const!(ORANGEDARK_ORANGE9, COLOR_9_RATATUI);
// radix_to_ratatui_const!(ORANGEDARK_ORANGE10, COLOR_10_RATATUI);
// radix_to_ratatui_const!(ORANGEDARK_ORANGE11, COLOR_11_RATATUI);
// radix_to_ratatui_const!(ORANGEDARK_ORANGE12, COLOR_12_RATATUI);

pub use COLOR_1_RATATUI as BASE_COLOR_RATATUI;
pub use COLOR_2_RATATUI as BACKGROUND_COLOR_RATATUI;
pub use COLOR_7_RATATUI as BORDER_COLOR_RATATUI;
pub use COLOR_9_RATATUI as DIM_TEXT_COLOR_RATATUI;
pub use COLOR_10_RATATUI as TEXT_COLOR_RATATUI;
pub use COLOR_12_RATATUI as TITLE_COLOR_RATATUI;
