use bevy::diagnostic::DiagnosticPath;

pub const SHAPE_BUFFER_COUNT_DIAG_PATH: DiagnosticPath =
  DiagnosticPath::const_new("render/shape_buffer_count");
pub const DRAWN_CELL_COUNT_DIAG_PATH: DiagnosticPath =
  DiagnosticPath::const_new("render/drawn_cell_count");
