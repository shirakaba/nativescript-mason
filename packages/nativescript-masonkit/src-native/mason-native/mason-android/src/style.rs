use jni::objects::{JObject, JObjectArray};
use jni::signature::ReturnType;
use jni::sys::{jfloat, jfloatArray, jint, jlong, jobjectArray, jshort};
use jni::JNIEnv;

use mason_core::style::{dimension_with_auto, min_max_from_values, Style};
use mason_core::{
    align_content_from_enum, display_from_enum, Dimension, GridTrackRepetition,
    LengthPercentageAuto, NonRepeatedTrackSizingFunction, TrackSizingFunction,
};

use crate::TRACK_SIZING_FUNCTION;

#[no_mangle]
pub extern "system" fn Java_org_nativescript_mason_masonkit_Style_nativeDestroy(
    _: JNIEnv,
    _: JObject,
    style: jlong,
) {
    if style == 0 {
        return;
    }
    unsafe {
        let _ = Box::from_raw(style as *mut Style);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_mason_masonkit_Style_nativeInit(
    _: JNIEnv,
    _: JObject,
) -> jlong {
    let mut style: Style = Style::default();
    style.set_align_content(align_content_from_enum(0));
    style.set_flex_basis(Dimension::Auto);
    style.set_flex_grow(0.0);
    style.set_flex_shrink(0.0);
    style.into_raw() as jlong
}

pub(crate) const JAVA_INT_TYPE: ReturnType = ReturnType::Primitive(jni::signature::Primitive::Int);
pub(crate) const JAVA_SHORT_TYPE: ReturnType =
    ReturnType::Primitive(jni::signature::Primitive::Short);
pub(crate) const JAVA_FLOAT_TYPE: ReturnType =
    ReturnType::Primitive(jni::signature::Primitive::Float);
pub(crate) const JAVA_BOOLEAN_TYPE: ReturnType =
    ReturnType::Primitive(jni::signature::Primitive::Boolean);
pub(crate) const JAVA_OBJECT_TYPE: ReturnType = ReturnType::Object;
pub(crate) const JAVA_ARRAY_TYPE: ReturnType = ReturnType::Array;

pub(crate) fn to_vec_non_repeated_track_sizing_function(
    env: &mut JNIEnv,
    array: jobjectArray,
) -> Vec<NonRepeatedTrackSizingFunction> {
    unsafe {
        return if array.is_null() {
            vec![]
        } else {
            let array = JObjectArray::from_raw(array);
            let len = env.get_array_length(&array).unwrap_or_default();
            if len == 0 {
                return vec![];
            }

            let mut ret = Vec::with_capacity(len as usize);

            let min_max = crate::MIN_MAX.get().unwrap();

            for i in 0..len {
                let object = env.get_object_array_element(&array, i).unwrap();
                let min_type = env
                    .call_method_unchecked(&object, min_max.min_type_id, JAVA_INT_TYPE, &[])
                    .unwrap();
                let min_value = env
                    .call_method_unchecked(&object, min_max.min_value_id, JAVA_FLOAT_TYPE, &[])
                    .unwrap();
                let max_type = env
                    .call_method_unchecked(&object, min_max.max_type_id, JAVA_INT_TYPE, &[])
                    .unwrap();
                let max_value = env
                    .call_method_unchecked(&object, min_max.max_value_id, JAVA_FLOAT_TYPE, &[])
                    .unwrap();
                ret.push(min_max_from_values(
                    min_type.i().unwrap(),
                    min_value.f().unwrap(),
                    max_type.i().unwrap(),
                    max_value.f().unwrap(),
                ));
            }
            ret
        };
    }
}

pub(crate) fn to_vec_track_sizing_function(
    env: &mut JNIEnv,
    array: jobjectArray,
) -> Vec<TrackSizingFunction> {
    if !array.is_null() {
        let array = unsafe { JObjectArray::from_raw(array) };
        let len = env.get_array_length(&array).unwrap_or_default();

        if len == 0 {
            return vec![];
        }

        let mut ret = Vec::with_capacity(len as usize);

        let track_sizing = TRACK_SIZING_FUNCTION.get().unwrap();

        let min_max = crate::MIN_MAX.get().unwrap();

        unsafe {
            for i in 0..len {
                let object = env.get_object_array_element(&array, i).unwrap();

                let is_repeating = env
                    .call_method_unchecked(
                        &object,
                        track_sizing.is_repeating,
                        JAVA_BOOLEAN_TYPE,
                        &[],
                    )
                    .unwrap()
                    .z()
                    .unwrap();

                if is_repeating {
                    let auto_repeat_grid_track_repetition = env
                        .call_method_unchecked(
                            &object,
                            track_sizing.auto_repeat_grid_track_repetition_id,
                            JAVA_INT_TYPE,
                            &[],
                        )
                        .unwrap()
                        .i()
                        .unwrap();

                    let auto_repeat_grid_track_repetition_count = env
                        .call_method_unchecked(
                            &object,
                            track_sizing.auto_repeat_grid_track_repetition_count_id,
                            JAVA_SHORT_TYPE,
                            &[],
                        )
                        .unwrap()
                        .s()
                        .unwrap();

                    let auto_repeat_value = env
                        .call_method_unchecked(
                            &object,
                            track_sizing.auto_repeat_value_id,
                            JAVA_ARRAY_TYPE,
                            &[],
                        )
                        .unwrap();

                    let auto_repeat_value_array =
                        JObjectArray::from(auto_repeat_value.l().unwrap());

                    let repeating_len = env
                        .get_array_length(&auto_repeat_value_array)
                        .unwrap_or_default();

                    let mut repeat_ret = Vec::with_capacity(repeating_len as usize);

                    for j in 0..repeating_len {
                        let repeat_object = env
                            .get_object_array_element(&auto_repeat_value_array, j)
                            .unwrap();

                        let min_type = env
                            .call_method_unchecked(
                                &repeat_object,
                                min_max.min_type_id,
                                JAVA_INT_TYPE,
                                &[],
                            )
                            .unwrap();

                        let min_value = env
                            .call_method_unchecked(
                                &repeat_object,
                                min_max.min_value_id,
                                JAVA_FLOAT_TYPE,
                                &[],
                            )
                            .unwrap();

                        let max_type = env
                            .call_method_unchecked(
                                &repeat_object,
                                min_max.max_type_id,
                                JAVA_INT_TYPE,
                                &[],
                            )
                            .unwrap();

                        let max_value = env
                            .call_method_unchecked(
                                repeat_object,
                                min_max.max_value_id,
                                JAVA_FLOAT_TYPE,
                                &[],
                            )
                            .unwrap();

                        repeat_ret.push(min_max_from_values(
                            min_type.i().unwrap(),
                            min_value.f().unwrap(),
                            max_type.i().unwrap(),
                            max_value.f().unwrap(),
                        ));
                    }

                    ret.push(TrackSizingFunction::Repeat(
                        match auto_repeat_grid_track_repetition {
                            0 => GridTrackRepetition::AutoFill,
                            1 => GridTrackRepetition::AutoFit,
                            2 => GridTrackRepetition::Count(
                                auto_repeat_grid_track_repetition_count as u16,
                            ),
                            _ => panic!(),
                        },
                        repeat_ret,
                    ));
                } else {
                    let single_value = env
                        .call_method_unchecked(
                            object,
                            track_sizing.single_value_id,
                            JAVA_OBJECT_TYPE,
                            &[],
                        )
                        .unwrap()
                        .l()
                        .unwrap();

                    let min_type = env
                        .call_method_unchecked(
                            &single_value,
                            min_max.min_type_id,
                            JAVA_INT_TYPE,
                            &[],
                        )
                        .unwrap();

                    let min_value = env
                        .call_method_unchecked(
                            &single_value,
                            min_max.min_value_id,
                            JAVA_FLOAT_TYPE,
                            &[],
                        )
                        .unwrap();

                    let max_type = env
                        .call_method_unchecked(
                            &single_value,
                            min_max.max_type_id,
                            JAVA_INT_TYPE,
                            &[],
                        )
                        .unwrap();

                    let max_value = env
                        .call_method_unchecked(
                            single_value,
                            min_max.max_value_id,
                            JAVA_FLOAT_TYPE,
                            &[],
                        )
                        .unwrap();

                    ret.push(TrackSizingFunction::Single(min_max_from_values(
                        min_type.i().unwrap(),
                        min_value.f().unwrap(),
                        max_type.i().unwrap(),
                        max_value.f().unwrap(),
                    )));
                }
            }
        }

        return ret;
    }

    vec![]
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_mason_masonkit_Style_nativeInitWithValues(
    env: &mut JNIEnv,
    _: JObject,
    display: jint,
    position: jint,
    direction: jint,
    flex_direction: jint,
    flex_wrap: jint,
    overflow: jint,
    align_items: jint,
    align_self: jint,
    align_content: jint,
    justify_items: jint,
    justify_self: jint,
    justify_content: jint,

    inset_left_type: jint,
    inset_left_value: jfloat,
    inset_right_type: jint,
    inset_right_value: jfloat,
    inset_top_type: jint,
    inset_top_value: jfloat,
    inset_bottom_type: jint,
    inset_bottom_value: jfloat,

    margin_left_type: jint,
    margin_left_value: jfloat,
    margin_right_type: jint,
    margin_right_value: jfloat,
    margin_top_type: jint,
    margin_top_value: jfloat,
    margin_bottom_type: jint,
    margin_bottom_value: jfloat,

    padding_left_type: jint,
    padding_left_value: jfloat,
    padding_right_type: jint,
    padding_right_value: jfloat,
    padding_top_type: jint,
    padding_top_value: jfloat,
    padding_bottom_type: jint,
    padding_bottom_value: jfloat,

    border_left_type: jint,
    border_left_value: jfloat,
    border_right_type: jint,
    border_right_value: jfloat,
    border_top_type: jint,
    border_top_value: jfloat,
    border_bottom_type: jint,
    border_bottom_value: jfloat,

    flex_grow: jfloat,
    flex_shrink: jfloat,

    flex_basis_type: jint,
    flex_basis_value: jfloat,

    width_type: jint,
    width_value: jfloat,
    height_type: jint,
    height_value: jfloat,

    min_width_type: jint,
    min_width_value: jfloat,
    min_height_type: jint,
    min_height_value: jfloat,

    max_width_type: jint,
    max_width_value: jfloat,
    max_height_type: jint,
    max_height_value: jfloat,

    gap_row_type: jint,
    gap_row_value: jfloat,
    gap_column_type: jint,
    gap_column_value: jfloat,

    aspect_ratio: jfloat,

    grid_auto_rows: jobjectArray,
    grid_auto_columns: jobjectArray,
    grid_auto_flow: jint,
    grid_column_start_type: jint,
    grid_column_start_value: jshort,
    grid_column_end_type: jint,
    grid_column_end_value: jshort,
    grid_row_start_type: jint,
    grid_row_start_value: jshort,
    grid_row_end_type: jint,
    grid_row_end_value: jshort,
    grid_template_rows: jobjectArray,
    grid_template_columns: jobjectArray,
    overflow_x: jint,
    overflow_y: jint,
    scrollbar_width: jfloat,
) -> jlong {
    let grid_auto_rows = to_vec_non_repeated_track_sizing_function(env, grid_auto_rows);
    let grid_auto_columns = to_vec_non_repeated_track_sizing_function(env, grid_auto_columns);
    let grid_template_rows = to_vec_track_sizing_function(env, grid_template_rows);
    let grid_template_columns = to_vec_track_sizing_function(env, grid_template_columns);

    Box::into_raw(Box::new(Style::from_ffi(
        display,
        position,
        direction,
        flex_direction,
        flex_wrap,
        overflow,
        align_items,
        align_self,
        align_content,
        justify_items,
        justify_self,
        justify_content,
        inset_left_type,
        inset_left_value,
        inset_right_type,
        inset_right_value,
        inset_top_type,
        inset_top_value,
        inset_bottom_type,
        inset_bottom_value,
        margin_left_type,
        margin_left_value,
        margin_right_type,
        margin_right_value,
        margin_top_type,
        margin_top_value,
        margin_bottom_type,
        margin_bottom_value,
        padding_left_type,
        padding_left_value,
        padding_right_type,
        padding_right_value,
        padding_top_type,
        padding_top_value,
        padding_bottom_type,
        padding_bottom_value,
        border_left_type,
        border_left_value,
        border_right_type,
        border_right_value,
        border_top_type,
        border_top_value,
        border_bottom_type,
        border_bottom_value,
        flex_grow,
        flex_shrink,
        flex_basis_type,
        flex_basis_value,
        width_type,
        width_value,
        height_type,
        height_value,
        min_width_type,
        min_width_value,
        min_height_type,
        min_height_value,
        max_width_type,
        max_width_value,
        max_height_type,
        max_height_value,
        gap_row_type,
        gap_row_value,
        gap_column_type,
        gap_column_value,
        aspect_ratio,
        grid_auto_rows,
        grid_auto_columns,
        grid_auto_flow,
        grid_column_start_type,
        grid_column_start_value,
        grid_column_end_type,
        grid_column_end_value,
        grid_row_start_type,
        grid_row_start_value,
        grid_row_end_type,
        grid_row_end_value,
        grid_template_rows,
        grid_template_columns,
        overflow_x,
        overflow_y,
        scrollbar_width,
    ))) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_mason_masonkit_Style_nativeUpdateWithValues(
    env: &mut JNIEnv,
    _: JObject,
    style: jlong,
    display: jint,
    position: jint,
    direction: jint,
    flex_direction: jint,
    flex_wrap: jint,
    overflow: jint,
    align_items: jint,
    align_self: jint,
    align_content: jint,
    justify_items: jint,
    justify_self: jint,
    justify_content: jint,

    inset_left_type: jint,
    inset_left_value: jfloat,
    inset_right_type: jint,
    inset_right_value: jfloat,
    inset_top_type: jint,
    inset_top_value: jfloat,
    inset_bottom_type: jint,
    inset_bottom_value: jfloat,

    margin_left_type: jint,
    margin_left_value: jfloat,
    margin_right_type: jint,
    margin_right_value: jfloat,
    margin_top_type: jint,
    margin_top_value: jfloat,
    margin_bottom_type: jint,
    margin_bottom_value: jfloat,

    padding_left_type: jint,
    padding_left_value: jfloat,
    padding_right_type: jint,
    padding_right_value: jfloat,
    padding_top_type: jint,
    padding_top_value: jfloat,
    padding_bottom_type: jint,
    padding_bottom_value: jfloat,

    border_left_type: jint,
    border_left_value: jfloat,
    border_right_type: jint,
    border_right_value: jfloat,
    border_top_type: jint,
    border_top_value: jfloat,
    border_bottom_type: jint,
    border_bottom_value: jfloat,

    flex_grow: jfloat,
    flex_shrink: jfloat,

    flex_basis_type: jint,
    flex_basis_value: jfloat,

    width_type: jint,
    width_value: jfloat,
    height_type: jint,
    height_value: jfloat,

    min_width_type: jint,
    min_width_value: jfloat,
    min_height_type: jint,
    min_height_value: jfloat,

    max_width_type: jint,
    max_width_value: jfloat,
    max_height_type: jint,
    max_height_value: jfloat,

    gap_row_type: jint,
    gap_row_value: jfloat,
    gap_column_type: jint,
    gap_column_value: jfloat,

    aspect_ratio: jfloat,

    grid_auto_rows: jobjectArray,
    grid_auto_columns: jobjectArray,
    grid_auto_flow: jint,
    grid_column_start_type: jint,
    grid_column_start_value: jshort,
    grid_column_end_type: jint,
    grid_column_end_value: jshort,
    grid_row_start_type: jint,
    grid_row_start_value: jshort,
    grid_row_end_type: jint,
    grid_row_end_value: jshort,
    grid_template_rows: jobjectArray,
    grid_template_columns: jobjectArray,
    overflow_x: jint,
    overflow_y: jint,
    scrollbar_width: jfloat,
) {
    let grid_auto_rows = to_vec_non_repeated_track_sizing_function(env, grid_auto_rows);
    let grid_auto_columns = to_vec_non_repeated_track_sizing_function(env, grid_auto_columns);
    let grid_template_rows = to_vec_track_sizing_function(env, grid_template_rows);
    let grid_template_columns = to_vec_track_sizing_function(env, grid_template_columns);

    unsafe {
        let mut style = Box::from_raw(style as *mut Style);
        Style::update_from_ffi(
            &mut style,
            display,
            position,
            direction,
            flex_direction,
            flex_wrap,
            overflow,
            align_items,
            align_self,
            align_content,
            justify_items,
            justify_self,
            justify_content,
            inset_left_type,
            inset_left_value,
            inset_right_type,
            inset_right_value,
            inset_top_type,
            inset_top_value,
            inset_bottom_type,
            inset_bottom_value,
            margin_left_type,
            margin_left_value,
            margin_right_type,
            margin_right_value,
            margin_top_type,
            margin_top_value,
            margin_bottom_type,
            margin_bottom_value,
            padding_left_type,
            padding_left_value,
            padding_right_type,
            padding_right_value,
            padding_top_type,
            padding_top_value,
            padding_bottom_type,
            padding_bottom_value,
            border_left_type,
            border_left_value,
            border_right_type,
            border_right_value,
            border_top_type,
            border_top_value,
            border_bottom_type,
            border_bottom_value,
            flex_grow,
            flex_shrink,
            flex_basis_type,
            flex_basis_value,
            width_type,
            width_value,
            height_type,
            height_value,
            min_width_type,
            min_width_value,
            min_height_type,
            min_height_value,
            max_width_type,
            max_width_value,
            max_height_type,
            max_height_value,
            gap_row_type,
            gap_row_value,
            gap_column_type,
            gap_column_value,
            aspect_ratio,
            grid_auto_rows,
            grid_auto_columns,
            grid_auto_flow,
            grid_column_start_type,
            grid_column_start_value,
            grid_column_end_type,
            grid_column_end_value,
            grid_row_start_type,
            grid_row_start_value,
            grid_row_end_type,
            grid_row_end_value,
            grid_template_rows,
            grid_template_columns,
            overflow_x,
            overflow_y,
            scrollbar_width,
        );
        Box::leak(style);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_mason_masonkit_Style_nativeGetSize(
    env: JNIEnv,
    _: JObject,
    style: jlong,
) -> jfloatArray {
    if style == 0 {
        return env.new_float_array(0_i32).unwrap().into_raw();
    }
    unsafe {
        let style: Box<Style> = Box::from_raw(style as *mut Style);

        let size_width = style.get_size_width().into();
        let size_height = style.get_size_height().into();

        let mut output = vec![0.0, 0.0, 0.0, 0.0];

        match size_width {
            Dimension::Auto => {
                output[0] = 0.0;
                output[1] = 0.0;
            }
            Dimension::Length(points) => {
                output[0] = 1.0;
                output[1] = points;
            }
            Dimension::Percent(percent) => {
                output[0] = 2.0;
                output[1] = percent;
            }
        }

        match size_height {
            Dimension::Auto => {
                output[2] = 0.0;
                output[3] = 0.0;
            }
            Dimension::Length(points) => {
                output[2] = 1.0;
                output[3] = points;
            }
            Dimension::Percent(percent) => {
                output[2] = 2.0;
                output[3] = percent;
            }
        }

        let result = env.new_float_array(output.len() as i32).unwrap();
        env.set_float_array_region(&result, 0, &output).unwrap();

        Box::leak(style);

        result.into_raw()
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_mason_masonkit_Style_nativeGetMargins(
    env: JNIEnv,
    _: JObject,
    style: jlong,
) -> jfloatArray {
    if style == 0 {
        return env.new_float_array(0_i32).unwrap().into_raw();
    }
    unsafe {
        let style: Box<Style> = Box::from_raw(style as *mut Style);

        let top = style.get_margin_top();
        let left = style.get_margin_left();
        let bottom = style.get_margin_bottom();
        let right = style.get_margin_right();

        let mut output = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

        match top {
            LengthPercentageAuto::Auto => {
                output[0] = 0.0;
                output[1] = 0.0;
            }
            LengthPercentageAuto::Length(points) => {
                output[0] = 1.0;
                output[1] = points;
            }
            LengthPercentageAuto::Percent(percent) => {
                output[0] = 2.0;
                output[1] = percent;
            }
        }

        match left {
            LengthPercentageAuto::Auto => {
                output[2] = 0.0;
                output[3] = 0.0;
            }
            LengthPercentageAuto::Length(points) => {
                output[2] = 1.0;
                output[3] = points;
            }
            LengthPercentageAuto::Percent(percent) => {
                output[2] = 2.0;
                output[3] = percent;
            }
        }

        match bottom {
            LengthPercentageAuto::Auto => {
                output[4] = 0.0;
                output[5] = 0.0;
            }
            LengthPercentageAuto::Length(points) => {
                output[4] = 1.0;
                output[5] = points;
            }
            LengthPercentageAuto::Percent(percent) => {
                output[4] = 2.0;
                output[5] = percent;
            }
        }

        match right {
            LengthPercentageAuto::Auto => {
                output[6] = 0.0;
                output[7] = 0.0;
            }
            LengthPercentageAuto::Length(points) => {
                output[6] = 1.0;
                output[7] = points;
            }
            LengthPercentageAuto::Percent(percent) => {
                output[6] = 2.0;
                output[7] = percent;
            }
        }

        let result = env.new_float_array(output.len() as i32).unwrap();
        env.set_float_array_region(&result, 0, &output).unwrap();

        Box::leak(style);

        result.into_raw()
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_mason_masonkit_Style_nativeSetSize(
    _: JNIEnv,
    _: JObject,
    style: jlong,
    width_type: jint,
    width: jfloat,
    height_type: jint,
    height: jfloat,
) {
    if style == 0 {
        return;
    }
    unsafe {
        let mut style: Box<Style> = Box::from_raw(style as *mut Style);
        Style::set_size_width(&mut style, dimension_with_auto(width_type, width).into());
        Style::set_size_height(&mut style, dimension_with_auto(height_type, height).into());
        Box::leak(style);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_nativescript_mason_masonkit_Style_nativeSetDisplay(
    _: JNIEnv,
    _: JObject,
    style: jlong,
    display: jint,
) {
    if style == 0 {
        return;
    }
    unsafe {
        let mut style: Box<Style> = Box::from_raw(style as *mut Style);
        Style::set_display(&mut style, display_from_enum(display).unwrap());
        Box::leak(style);
    }
}
