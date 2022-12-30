use taffy::prelude::*;
use taffy::style::{
    AlignContent, AlignItems, AlignSelf, Dimension, Display, FlexDirection, FlexWrap,
    JustifyContent, LengthPercentage, LengthPercentageAuto, MinTrackSizingFunction, Position,
    TrackSizingFunction,
};

use crate::{
    align_content_from_enum, align_items_from_enum, align_self_from_enum, display_from_enum,
    flex_direction_from_enum, flex_wrap_from_enum, grid_auto_flow_from_enum,
    justify_content_from_enum, position_from_enum, Rect, Size,
};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Style {
    pub(crate) style: taffy::style::Style,
}

const fn dimension_with_auto(t: i32, v: f32) -> LengthPercentageAuto {
    match t {
        0 => LengthPercentageAuto::Points(v),
        1 => LengthPercentageAuto::Percent(v),
        2 => LengthPercentageAuto::Auto,
        _ => panic!(),
    }
}

const fn dimension(t: i32, v: f32) -> LengthPercentage {
    match t {
        0 => LengthPercentage::Points(v),
        1 => LengthPercentage::Percent(v),
        _ => panic!(),
    }
}

pub fn min_max_from_values(
    min_type: i32,
    min_value: f32,
    max_type: i32,
    max_value: f32,
) -> NonRepeatedTrackSizingFunction {
    NonRepeatedTrackSizingFunction {
        min: match min_type {
            0 => MinTrackSizingFunction::AUTO,
            1 => MinTrackSizingFunction::MIN_CONTENT,
            2 => MinTrackSizingFunction::MAX_CONTENT,
            3 => MinTrackSizingFunction::from_percent(min_value),
            4 => MinTrackSizingFunction::from_points(min_value),
            _ => panic!(),
        },
        max: match max_type {
            0 => MaxTrackSizingFunction::AUTO,
            1 => MaxTrackSizingFunction::MIN_CONTENT,
            2 => MaxTrackSizingFunction::MAX_CONTENT,
            3 => MaxTrackSizingFunction::from_percent(max_value),
            4 => MaxTrackSizingFunction::from_points(max_value),
            _ => panic!(),
        },
    }
}

fn to_grid_template(
    single: Option<&[(i32, f32, i32, f32)]>,
    auto_repeat: Option<&[(i32, &[(i32, f32, i32, f32)])]>,
) -> Vec<TrackSizingFunction> {
    if let Some(single) = single {
        return single
            .iter()
            .map(|(min_type, min_value, max_type, max_value)| {
                TrackSizingFunction::Single(min_max_from_values(
                    *min_type, *min_value, *max_type, *max_value,
                ))
            })
            .collect();
    } else if let Some(auto_repeat) = auto_repeat {
        return auto_repeat
            .iter()
            .map(|(track_repetition, values)| {
                TrackSizingFunction::AutoRepeat(
                    match track_repetition {
                        0 => GridTrackRepetition::AutoFill,
                        1 => GridTrackRepetition::AutoFit,
                        _ => panic!(),
                    },
                    values
                        .iter()
                        .map(|(min_type, min_value, max_type, max_value)| {
                            min_max_from_values(*min_type, *min_value, *max_type, *max_value)
                        })
                        .collect(),
                )
            })
            .collect();
    }
    vec![]
}

fn grid_placement(t: i32, v: i16) -> GridPlacement {
    match t {
        0 => GridPlacement::Auto,
        1 => GridPlacement::Line(v),
        2 => GridPlacement::Span(v.try_into().unwrap()),
        _ => panic!(),
    }
}

impl Style {
    pub fn from_taffy(style: taffy::style::Style) -> Self {
        Self { style }
    }

    pub fn into_raw(self) -> *mut Style {
        Box::into_raw(Box::new(self))
    }

    /* Flexbox */

    // Container
    pub fn flex_direction(&self) -> FlexDirection {
        self.style.flex_direction
    }

    pub fn set_flex_direction(&mut self, flex_direction: FlexDirection) {
        self.style.flex_direction = flex_direction;
    }

    pub fn flex_wrap(&self) -> FlexWrap {
        self.style.flex_wrap
    }

    pub fn set_flex_wrap(&mut self, wrap: FlexWrap) {
        self.style.flex_wrap = wrap;
    }

    pub fn justify_content(&self) -> Option<JustifyContent> {
        self.style.justify_content
    }

    pub fn set_justify_content(&mut self, justify: Option<JustifyContent>) {
        self.style.justify_content = justify
    }

    pub fn align_items(&self) -> Option<AlignItems> {
        self.style.align_items
    }

    pub fn set_align_items(&mut self, align: Option<AlignItems>) {
        self.style.align_items = align
    }

    pub fn align_content(&self) -> Option<AlignContent> {
        self.style.align_content
    }

    pub fn set_align_content(&mut self, align: Option<AlignContent>) {
        self.style.align_content = align
    }

    // Items

    pub fn flex_grow(&self) -> f32 {
        self.style.flex_grow
    }

    pub fn set_flex_grow(&mut self, grow: f32) {
        self.style.flex_grow = grow;
    }

    pub fn flex_shrink(&self) -> f32 {
        self.style.flex_shrink
    }

    pub fn set_flex_shrink(&mut self, shrink: f32) {
        self.style.flex_shrink = shrink;
    }

    pub fn flex_basis(&self) -> Dimension {
        self.style.flex_basis
    }

    pub fn set_flex_basis(&mut self, basis: Dimension) {
        self.style.flex_basis = basis;
    }

    pub fn align_self(&self) -> Option<AlignSelf> {
        self.style.align_self
    }

    pub fn set_align_self(&mut self, align: Option<AlignSelf>) {
        self.style.align_self = align
    }

    /* Flexbox */

    /* Grid */

    pub fn gap(&self) -> Size<LengthPercentage> {
        Size::from_taffy(self.style.gap)
    }

    pub fn set_gap(&mut self, size: Size<LengthPercentage>) {
        self.style.gap = size.size;
    }

    pub fn set_row_gap(&mut self, row: LengthPercentage) {
        self.style.gap.width = row;
    }

    pub fn get_row_gap(&self) -> LengthPercentage {
        self.style.gap.width
    }

    pub fn set_column_gap(&mut self, column: LengthPercentage) {
        self.style.gap.height = column;
    }

    pub fn get_column_gap(&self) -> LengthPercentage {
        self.style.gap.height
    }

    pub fn get_grid_auto_columns(&self) -> &[NonRepeatedTrackSizingFunction] {
        self.style.grid_auto_columns.as_slice()
    }

    pub fn set_grid_auto_columns(&mut self, columns: &[NonRepeatedTrackSizingFunction]) {
        self.style.grid_auto_columns = columns.to_vec();
    }

    pub fn get_grid_auto_flow(&self) -> GridAutoFlow {
        self.style.grid_auto_flow
    }

    pub fn set_grid_auto_flow(&mut self, flow: GridAutoFlow) {
        self.style.grid_auto_flow = flow
    }

    pub fn get_grid_column(&self) -> Line<GridPlacement> {
        self.style.grid_column
    }

    pub fn get_grid_column_start(&self) -> GridPlacement {
        self.style.grid_column.start
    }

    pub fn get_grid_column_end(&self) -> GridPlacement {
        self.style.grid_column.end
    }

    pub fn set_grid_column(&mut self, column: Line<GridPlacement>) {
        self.style.grid_column = column
    }

    pub fn set_grid_column_start(&mut self, start: GridPlacement) {
        self.style.grid_column.start = start
    }

    pub fn set_grid_column_end(&mut self, end: GridPlacement) {
        self.style.grid_column.end = end
    }

    pub fn get_grid_row(&self) -> Line<GridPlacement> {
        self.style.grid_row
    }

    pub fn get_grid_row_start(&self) -> GridPlacement {
        self.style.grid_row.start
    }

    pub fn get_grid_row_end(&self) -> GridPlacement {
        self.style.grid_row.end
    }

    pub fn set_grid_row(&mut self, row: Line<GridPlacement>) {
        self.style.grid_row = row
    }

    pub fn set_grid_row_start(&mut self, start: GridPlacement) {
        self.style.grid_row.start = start
    }

    pub fn set_grid_row_end(&mut self, end: GridPlacement) {
        self.style.grid_row.end = end
    }

    pub fn set_grid_template_rows(&mut self, rows: &[TrackSizingFunction]) {
        self.style.grid_template_rows = rows.to_vec();
    }

    pub fn get_grid_template_rows(&self) -> &[TrackSizingFunction] {
        self.style.grid_template_rows.as_slice()
    }

    pub fn set_grid_template_columns(&mut self, columns: &[TrackSizingFunction]) {
        self.style.grid_template_columns = columns.to_vec();
    }

    pub fn get_grid_template_columns(&self) -> &[TrackSizingFunction] {
        self.style.grid_template_columns.as_slice()
    }

    /* Grid */

    /* Core */

    pub fn display(&self) -> Display {
        self.style.display
    }

    pub fn set_display(&mut self, display: Display) {
        self.style.display = display;
    }

    pub fn position(&self) -> Position {
        self.style.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.style.position = position;
    }

    pub fn inset(&self) -> Rect<LengthPercentageAuto> {
        Rect::from_taffy(self.style.inset)
    }

    pub fn set_inset(&mut self, inset: Rect<LengthPercentageAuto>) {
        self.style.inset = inset.rect
    }

    pub fn set_inset_lrtb(
        &mut self,
        left: LengthPercentageAuto,
        right: LengthPercentageAuto,
        top: LengthPercentageAuto,
        bottom: LengthPercentageAuto,
    ) {
        self.style.inset.left = left;
        self.style.inset.right = right;
        self.style.inset.top = top;
        self.style.inset.bottom = bottom;
    }

    pub fn set_inset_left(&mut self, left: LengthPercentageAuto) {
        self.style.inset.left = left;
    }

    pub fn get_inset_left(&self) -> LengthPercentageAuto {
        self.style.inset.left
    }

    pub fn set_inset_right(&mut self, right: LengthPercentageAuto) {
        self.style.inset.right = right;
    }

    pub fn get_inset_right(&self) -> LengthPercentageAuto {
        self.style.inset.right
    }

    pub fn set_inset_top(&mut self, top: LengthPercentageAuto) {
        self.style.inset.top = top;
    }

    pub fn get_inset_top(&self) -> LengthPercentageAuto {
        self.style.inset.top
    }

    pub fn set_inset_bottom(&mut self, bottom: LengthPercentageAuto) {
        self.style.inset.bottom = bottom;
    }

    pub fn get_inset_bottom(&self) -> LengthPercentageAuto {
        self.style.inset.bottom
    }

    pub fn margin(&self) -> Rect<LengthPercentageAuto> {
        Rect::from_taffy(self.style.margin)
    }

    pub fn set_margin(&mut self, margin: Rect<LengthPercentageAuto>) {
        self.style.margin = margin.rect;
    }

    pub fn set_margin_lrtb(
        &mut self,
        left: LengthPercentageAuto,
        right: LengthPercentageAuto,
        top: LengthPercentageAuto,
        bottom: LengthPercentageAuto,
    ) {
        self.style.margin.left = left;
        self.style.margin.right = right;
        self.style.margin.top = top;
        self.style.margin.bottom = bottom;
    }

    pub fn set_margin_left(&mut self, left: LengthPercentageAuto) {
        self.style.margin.left = left;
    }

    pub fn get_margin_left(&self) -> LengthPercentageAuto {
        self.style.margin.left
    }

    pub fn set_margin_right(&mut self, right: LengthPercentageAuto) {
        self.style.margin.right = right;
    }

    pub fn get_margin_right(&self) -> LengthPercentageAuto {
        self.style.margin.right
    }

    pub fn set_margin_top(&mut self, top: LengthPercentageAuto) {
        self.style.margin.top = top;
    }

    pub fn get_margin_top(&self) -> LengthPercentageAuto {
        self.style.margin.top
    }

    pub fn set_margin_bottom(&mut self, bottom: LengthPercentageAuto) {
        self.style.margin.bottom = bottom;
    }

    pub fn get_margin_bottom(&self) -> LengthPercentageAuto {
        self.style.margin.bottom
    }

    pub fn padding(&self) -> Rect<LengthPercentage> {
        Rect::from_taffy(self.style.padding)
    }

    pub fn set_padding(&mut self, padding: Rect<LengthPercentage>) {
        self.style.padding = padding.rect;
    }

    pub fn set_padding_lrtb(
        &mut self,
        left: LengthPercentage,
        right: LengthPercentage,
        top: LengthPercentage,
        bottom: LengthPercentage,
    ) {
        self.style.padding.left = left;
        self.style.padding.right = right;
        self.style.padding.top = top;
        self.style.padding.bottom = bottom;
    }

    pub fn set_padding_left(&mut self, left: LengthPercentage) {
        self.style.padding.left = left;
    }

    pub fn get_padding_left(&self) -> LengthPercentage {
        self.style.padding.left
    }

    pub fn set_padding_right(&mut self, right: LengthPercentage) {
        self.style.padding.right = right;
    }

    pub fn get_padding_right(&self) -> LengthPercentage {
        self.style.padding.right
    }

    pub fn set_padding_top(&mut self, top: LengthPercentage) {
        self.style.padding.top = top;
    }

    pub fn get_padding_top(&self) -> LengthPercentage {
        self.style.padding.top
    }

    pub fn set_padding_bottom(&mut self, bottom: LengthPercentage) {
        self.style.padding.bottom = bottom;
    }

    pub fn get_padding_bottom(&self) -> LengthPercentage {
        self.style.padding.bottom
    }

    pub fn border(&self) -> Rect<LengthPercentage> {
        Rect::from_taffy(self.style.border)
    }

    pub fn set_border(&mut self, border: Rect<LengthPercentage>) {
        self.style.border = border.rect;
    }

    pub fn set_border_lrtb(
        &mut self,
        left: LengthPercentage,
        right: LengthPercentage,
        top: LengthPercentage,
        bottom: LengthPercentage,
    ) {
        self.style.border.left = left;
        self.style.border.right = right;
        self.style.border.top = top;
        self.style.border.bottom = bottom;
    }

    pub fn set_border_left(&mut self, left: LengthPercentage) {
        self.style.border.left = left;
    }

    pub fn get_border_left(&self) -> LengthPercentage {
        self.style.border.left
    }

    pub fn set_border_right(&mut self, right: LengthPercentage) {
        self.style.border.right = right;
    }

    pub fn get_border_right(&self) -> LengthPercentage {
        self.style.border.right
    }

    pub fn set_border_top(&mut self, top: LengthPercentage) {
        self.style.border.top = top;
    }

    pub fn get_border_top(&self) -> LengthPercentage {
        self.style.border.top
    }

    pub fn set_border_bottom(&mut self, bottom: LengthPercentage) {
        self.style.border.bottom = bottom;
    }

    pub fn get_border_bottom(&self) -> LengthPercentage {
        self.style.border.bottom
    }

    pub fn size(&self) -> Size<Dimension> {
        Size::from_taffy(self.style.size)
    }

    pub fn set_size(&mut self, size: Size<Dimension>) {
        self.style.size = size.size
    }

    pub fn set_size_width(&mut self, width: Dimension) {
        self.style.size.width = width;
    }

    pub fn get_size_width(&self) -> Dimension {
        self.style.size.width
    }

    pub fn set_size_height(&mut self, height: Dimension) {
        self.style.size.height = height;
    }

    pub fn get_size_height(&self) -> Dimension {
        self.style.size.height
    }

    pub fn min_size(&self) -> Size<Dimension> {
        Size::from_taffy(self.style.min_size)
    }

    pub fn set_min_size(&mut self, min: Size<Dimension>) {
        self.style.min_size = min.size
    }

    pub fn set_min_size_width(&mut self, width: Dimension) {
        self.style.min_size.width = width;
    }

    pub fn get_min_size_width(&self) -> Dimension {
        self.style.min_size.width
    }

    pub fn set_min_size_height(&mut self, height: Dimension) {
        self.style.min_size.height = height;
    }

    pub fn get_min_size_height(&self) -> Dimension {
        self.style.min_size.height
    }

    pub fn max_size(&self) -> Size<Dimension> {
        Size::from_taffy(self.style.max_size)
    }

    pub fn set_max_size(&mut self, size: Size<Dimension>) {
        self.style.max_size = size.size;
    }

    pub fn set_max_size_width(&mut self, width: Dimension) {
        self.style.max_size.width = width;
    }

    pub fn get_max_size_width(&self) -> Dimension {
        self.style.max_size.width
    }

    pub fn set_max_size_height(&mut self, height: Dimension) {
        self.style.max_size.height = height;
    }

    pub fn get_max_size_height(&self) -> Dimension {
        self.style.max_size.height
    }

    pub fn aspect_ratio(&self) -> Option<f32> {
        self.style.aspect_ratio
    }

    pub fn set_aspect_ratio(&mut self, ratio: Option<f32>) {
        match ratio {
            None => {
                self.style.aspect_ratio = None;
            }
            Some(value) => {
                self.style.aspect_ratio = if f32::is_nan(value) {
                    None
                } else {
                    Some(value)
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_ffi(
        display: i32,
        position: i32,
        direction: i32,
        flex_direction: i32,
        flex_wrap: i32,
        overflow: i32,
        align_items: i32,
        align_self: i32,
        align_content: i32,
        justify_content: i32,
        inset_left_type: i32,
        inset_left_value: f32,
        inset_right_type: i32,
        inset_right_value: f32,
        inset_top_type: i32,
        inset_top_value: f32,
        inset_bottom_type: i32,
        inset_bottom_value: f32,
        margin_left_type: i32,
        margin_left_value: f32,
        margin_right_type: i32,
        margin_right_value: f32,
        margin_top_type: i32,
        margin_top_value: f32,
        margin_bottom_type: i32,
        margin_bottom_value: f32,
        padding_left_type: i32,
        padding_left_value: f32,
        padding_right_type: i32,
        padding_right_value: f32,
        padding_top_type: i32,
        padding_top_value: f32,
        padding_bottom_type: i32,
        padding_bottom_value: f32,
        border_left_type: i32,
        border_left_value: f32,
        border_right_type: i32,
        border_right_value: f32,
        border_top_type: i32,
        border_top_value: f32,
        border_bottom_type: i32,
        border_bottom_value: f32,
        flex_grow: f32,
        flex_shrink: f32,
        flex_basis_type: i32,
        flex_basis_value: f32,
        width_type: i32,
        width_value: f32,
        height_type: i32,
        height_value: f32,
        min_width_type: i32,
        min_width_value: f32,
        min_height_type: i32,
        min_height_value: f32,
        max_width_type: i32,
        max_width_value: f32,
        max_height_type: i32,
        max_height_value: f32,
        gap_row_type: i32,
        gap_row_value: f32,
        gap_column_type: i32,
        gap_column_value: f32,
        aspect_ratio: f32,
        grid_auto_rows: Vec<NonRepeatedTrackSizingFunction>,
        grid_auto_columns: Vec<NonRepeatedTrackSizingFunction>,
        grid_auto_flow: i32,
        grid_column_start_type: i32,
        grid_column_start_value: i16,
        grid_column_end_type: i32,
        grid_column_end_value: i16,
        grid_row_start_type: i32,
        grid_row_start_value: i16,
        grid_row_end_type: i32,
        grid_row_end_value: i16,
        grid_template_rows: Vec<TrackSizingFunction>,
        grid_template_columns: Vec<TrackSizingFunction>,
    ) -> Self {
        Style::from_taffy(taffy::style::Style {
            display: match display {
                0 => Display::Flex,
                1 => Display::None,
                _ => panic!(),
            },

            position: match position {
                0 => Position::Relative,
                1 => Position::Absolute,
                _ => panic!(),
            },

            /* direction: match direction {
                0 => Direction::Inherit,
                1 => Direction::LTR,
                2 => Direction::RTL,
                _ => panic!(),
            }, */
            flex_direction: match flex_direction {
                0 => FlexDirection::Row,
                1 => FlexDirection::Column,
                2 => FlexDirection::RowReverse,
                3 => FlexDirection::ColumnReverse,
                _ => panic!(),
            },

            flex_wrap: match flex_wrap {
                0 => FlexWrap::NoWrap,
                1 => FlexWrap::Wrap,
                2 => FlexWrap::WrapReverse,
                _ => panic!(),
            },

            /*
            overflow: match overflow {
                0 => Overflow::Visible,
                1 => Overflow::Hidden,
                2 => Overflow::Scroll,
                _ => panic!(),
            },
            */
            align_items: match align_items {
                -1 => None,
                0 => Some(AlignItems::Start),
                1 => Some(AlignItems::End),
                2 => Some(AlignItems::Center),
                3 => Some(AlignItems::Baseline),
                4 => Some(AlignItems::Stretch),
                _ => panic!(),
            },

            align_self: match align_self {
                -1 => None,
                0 => Some(AlignSelf::Start),
                1 => Some(AlignSelf::End),
                2 => Some(AlignSelf::Center),
                3 => Some(AlignSelf::Baseline),
                4 => Some(AlignSelf::Stretch),
                _ => panic!(),
            },

            justify_items: match align_items {
                -1 => None,
                0 => Some(AlignItems::Start),
                1 => Some(AlignItems::End),
                2 => Some(AlignItems::Center),
                3 => Some(AlignItems::Baseline),
                4 => Some(AlignItems::Stretch),
                _ => panic!(),
            },
            justify_self: match align_self {
                -1 => None,
                0 => Some(AlignSelf::Start),
                1 => Some(AlignSelf::End),
                2 => Some(AlignSelf::Center),
                3 => Some(AlignSelf::Baseline),
                4 => Some(AlignSelf::Stretch),
                _ => panic!(),
            },
            align_content: match align_content {
                -1 => None,
                0 => Some(AlignContent::Start),
                1 => Some(AlignContent::End),
                2 => Some(AlignContent::Center),
                3 => Some(AlignContent::Stretch),
                4 => Some(AlignContent::SpaceBetween),
                5 => Some(AlignContent::SpaceAround),
                6 => Some(AlignContent::SpaceEvenly),
                7 => Some(AlignContent::Stretch),
                _ => panic!(),
            },

            justify_content: match justify_content {
                -1 => None,
                0 => Some(JustifyContent::Start),
                1 => Some(JustifyContent::End),
                2 => Some(JustifyContent::Center),
                3 => Some(JustifyContent::SpaceBetween),
                4 => Some(JustifyContent::SpaceAround),
                5 => Some(JustifyContent::SpaceEvenly),
                _ => panic!(),
            },

            inset: taffy::geometry::Rect {
                left: dimension_with_auto(inset_left_type, inset_left_value),
                top: dimension_with_auto(inset_top_type, inset_top_value),
                bottom: dimension_with_auto(inset_bottom_type, inset_bottom_value),
                right: dimension_with_auto(inset_right_type, inset_right_value),
            },

            margin: taffy::geometry::Rect {
                left: dimension_with_auto(margin_left_type, margin_left_value),
                right: dimension_with_auto(margin_right_type, margin_right_value),
                top: dimension_with_auto(margin_top_type, margin_top_value),
                bottom: dimension_with_auto(margin_bottom_type, margin_bottom_value),
            },

            padding: taffy::geometry::Rect {
                left: dimension(padding_left_type, padding_left_value),
                right: dimension(padding_right_type, padding_right_value),
                top: dimension(padding_top_type, padding_top_value),
                bottom: dimension(padding_bottom_type, padding_bottom_value),
            },

            border: taffy::geometry::Rect {
                left: dimension(border_left_type, border_left_value),
                right: dimension(border_right_type, border_right_value),
                top: dimension(border_top_type, border_top_value),
                bottom: dimension(border_bottom_type, border_bottom_value),
            },

            gap: taffy::geometry::Size {
                width: dimension(gap_row_type, gap_row_value),
                height: dimension(gap_column_type, gap_column_value),
            },

            flex_grow,

            flex_shrink,

            grid_template_rows,
            grid_template_columns,
            grid_auto_rows,

            grid_auto_columns,

            grid_auto_flow: grid_auto_flow_from_enum(grid_auto_flow).unwrap_or_else(|| panic!()),
            grid_row: Line {
                start: grid_placement(grid_row_start_type, grid_row_start_value),
                end: grid_placement(grid_row_end_type, grid_row_end_value),
            },
            flex_basis: dimension(flex_basis_type, flex_basis_value).into(),

            size: taffy::geometry::Size {
                width: dimension(width_type, width_value).into(),
                height: dimension(height_type, height_value).into(),
            },

            min_size: taffy::geometry::Size {
                width: dimension(min_width_type, min_width_value).into(),
                height: dimension(min_height_type, min_height_value).into(),
            },

            max_size: taffy::geometry::Size {
                width: dimension(max_width_type, max_width_value).into(),
                height: dimension(max_height_type, max_height_value).into(),
            },

            aspect_ratio: if f32::is_nan(aspect_ratio) {
                None
            } else {
                Some(aspect_ratio)
            },
            grid_column: Line {
                start: grid_placement(grid_column_start_type, grid_column_start_value),
                end: grid_placement(grid_column_end_type, grid_column_end_value),
            },
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_from_ffi(
        style: &mut Style,
        display: i32,
        position: i32,
        _direction: i32,
        flex_direction: i32,
        flex_wrap: i32,
        _overflow: i32,
        align_items: i32,
        align_self: i32,
        align_content: i32,
        justify_content: i32,
        inset_left_type: i32,
        inset_left_value: f32,
        inset_right_type: i32,
        inset_right_value: f32,
        inset_top_type: i32,
        inset_top_value: f32,
        inset_bottom_type: i32,
        inset_bottom_value: f32,
        margin_left_type: i32,
        margin_left_value: f32,
        margin_right_type: i32,
        margin_right_value: f32,
        margin_top_type: i32,
        margin_top_value: f32,
        margin_bottom_type: i32,
        margin_bottom_value: f32,
        padding_left_type: i32,
        padding_left_value: f32,
        padding_right_type: i32,
        padding_right_value: f32,
        padding_top_type: i32,
        padding_top_value: f32,
        padding_bottom_type: i32,
        padding_bottom_value: f32,
        border_left_type: i32,
        border_left_value: f32,
        border_right_type: i32,
        border_right_value: f32,
        border_top_type: i32,
        border_top_value: f32,
        border_bottom_type: i32,
        border_bottom_value: f32,
        flex_grow: f32,
        flex_shrink: f32,
        flex_basis_type: i32,
        flex_basis_value: f32,
        width_type: i32,
        width_value: f32,
        height_type: i32,
        height_value: f32,
        min_width_type: i32,
        min_width_value: f32,
        min_height_type: i32,
        min_height_value: f32,
        max_width_type: i32,
        max_width_value: f32,
        max_height_type: i32,
        max_height_value: f32,
        gap_row_type: i32,
        gap_row_value: f32,
        gap_column_type: i32,
        gap_column_value: f32,
        aspect_ratio: f32,
        grid_auto_rows: Vec<NonRepeatedTrackSizingFunction>,
        grid_auto_columns: Vec<NonRepeatedTrackSizingFunction>,
        grid_auto_flow: i32,
        grid_column_start_type: i32,
        grid_column_start_value: i16,
        grid_column_end_type: i32,
        grid_column_end_value: i16,
        grid_row_start_type: i32,
        grid_row_start_value: i16,
        grid_row_end_type: i32,
        grid_row_end_value: i16,
        grid_template_rows: Vec<TrackSizingFunction>,
        grid_template_columns: Vec<TrackSizingFunction>,
    ) {
        style.style.display = display_from_enum(display).unwrap_or_else(|| panic!());

        style.style.position = position_from_enum(position).unwrap_or_else(|| panic!());

        /* direction: match direction {
            0 => Direction::Inherit,
            1 => Direction::LTR,
            2 => Direction::RTL,
            _ => panic!(),
        }, */

        style.style.flex_direction =
            flex_direction_from_enum(flex_direction).unwrap_or_else(|| panic!());

        style.style.flex_wrap = flex_wrap_from_enum(flex_wrap).unwrap_or_else(|| panic!());

        /*
        overflow: match overflow {
            0 => Overflow::Visible,
            1 => Overflow::Hidden,
            2 => Overflow::Scroll,
            _ => panic!(),
        },
        */

        style.style.align_items =
            Some(align_items_from_enum(align_items).unwrap_or_else(|| panic!()));

        style.style.align_self = Some(align_self_from_enum(align_self).unwrap_or_else(|| panic!()));

        style.style.align_content =
            Some(align_content_from_enum(align_content).unwrap_or_else(|| panic!()));

        style.style.justify_content =
            Some(justify_content_from_enum(justify_content).unwrap_or_else(|| panic!()));

        style.style.inset = taffy::geometry::Rect {
            left: dimension_with_auto(inset_left_type, inset_left_value),
            top: dimension_with_auto(inset_top_type, inset_top_value),
            bottom: dimension_with_auto(inset_bottom_type, inset_bottom_value),
            right: dimension_with_auto(inset_right_type, inset_right_value),
        };

        style.style.margin = taffy::geometry::Rect {
            left: dimension_with_auto(margin_left_type, margin_left_value),
            right: dimension_with_auto(margin_right_type, margin_right_value),
            top: dimension_with_auto(margin_top_type, margin_top_value),
            bottom: dimension_with_auto(margin_bottom_type, margin_bottom_value),
        };

        style.style.padding = taffy::geometry::Rect {
            left: dimension(padding_left_type, padding_left_value),
            right: dimension(padding_right_type, padding_right_value),
            top: dimension(padding_top_type, padding_top_value),
            bottom: dimension(padding_bottom_type, padding_bottom_value),
        };

        style.style.border = taffy::geometry::Rect {
            left: dimension(border_left_type, border_left_value),
            right: dimension(border_right_type, border_right_value),
            top: dimension(border_top_type, border_top_value),
            bottom: dimension(border_bottom_type, border_bottom_value),
        };

        style.style.gap = taffy::geometry::Size {
            width: dimension(gap_row_type, gap_row_value),
            height: dimension(gap_column_type, gap_column_value),
        };
        style.style.flex_grow = flex_grow;
        style.style.flex_shrink = flex_shrink;

        style.style.flex_basis = dimension(flex_basis_type, flex_basis_value).into();

        style.style.size = taffy::geometry::Size {
            width: dimension(width_type, width_value).into(),
            height: dimension(height_type, height_value).into(),
        };

        style.style.min_size = taffy::geometry::Size {
            width: dimension(min_width_type, min_width_value).into(),
            height: dimension(min_height_type, min_height_value).into(),
        };

        style.style.max_size = taffy::geometry::Size {
            width: dimension(max_width_type, max_width_value).into(),
            height: dimension(max_height_type, max_height_value).into(),
        };

        style.style.aspect_ratio = if f32::is_nan(aspect_ratio) {
            None
        } else {
            Some(aspect_ratio)
        };

        style.style.grid_template_rows = grid_template_rows;

        style.style.grid_template_columns = grid_template_columns;

        style.style.grid_auto_rows = grid_auto_rows;

        style.style.grid_auto_columns = grid_auto_columns;

        style.style.grid_auto_flow =
            grid_auto_flow_from_enum(grid_auto_flow).unwrap_or_else(|| panic!());

        style.style.grid_row = Line {
            start: grid_placement(grid_row_start_type, grid_row_start_value),
            end: grid_placement(grid_row_end_type, grid_row_end_value),
        };

        style.style.grid_column = Line {
            start: grid_placement(grid_column_start_type, grid_column_start_value),
            end: grid_placement(grid_column_end_type, grid_column_end_value),
        };
    }
}
