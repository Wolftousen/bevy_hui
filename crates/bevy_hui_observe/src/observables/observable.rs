#![allow(dead_code)]

use bevy::{prelude::*, utils::HashMap};

use crate::prelude::*;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct HuiObservableProperties(HashMap<String, Box<dyn HuiObservableType>>);

impl HuiObservableProperties {
    pub fn register(&mut self, key: impl Into<String>, typ: impl HuiObservableType){
        self.insert(key.into(), Box::new(typ));
    }
    
    pub fn initialize() -> Self {
        let mut this = Self::default();

        this.register("image_color", HuiImageColor);
        this.register("flip_x", HuiFlipX);
        this.register("flip_y", HuiFlipY);
        this.register("image", HuiImage);
        this.register("image_mode", HuiImageMode);
        this.register("text", HuiText);
        this.register("font", HuiFont);
        this.register("font_size", HuiFontSize);
        this.register("font_colors", HuiFontColor);
        this.register("font_smoothing", HuiFontSmoothing);
        this.register("justify_text", HuiJustifyText);
        this.register("linebreak", HuiLineBreak);
        this.register("align_content", HuiAlignContent);
        this.register("align_items", HuiAlignItems);
        this.register("align_self", HuiAlignSelf);
        this.register("aspect_ratio", HuiAspectRatio);
        this.register("background_color", HuiBackgroundColor);
        this.register("border_bottom", HuiBorderBottom);
        this.register("border_color", HuiBorderColor);
        this.register("border_left", HuiBorderLeft);
        this.register("border_radius_bottom_left", HuiBorderRadiusBottomLeft);
        this.register("border_radius_bottom_right", HuiBorderRadiusBottomRight);
        this.register("border_radius_top_left", HuiBorderRadiusTopLeft);
        this.register("border_radius_top_right", HuiBorderRadiusTopRight);
        this.register("border_right", HuiBorderRight);
        this.register("border_top", HuiBorderTop);
        this.register("bottom", HuiBottom);
        this.register("column_gap", HuiColumnGap);
        this.register("display", HuiDisplay);
        this.register("flex_basis", HuiFlexBasis);
        this.register("flex_direction", HuiFlexDirection);
        this.register("flex_grow", HuiFlexGrow);
        this.register("flex_shrink", HuiFlexShrink);
        this.register("flex_wrap", HuiFlexWrap);
        this.register("focus_policy", HuiFocusPolicy);
        this.register("height", HuiHeight);
        this.register("justify_content", HuiJustifyContent);
        this.register("justify_items", HuiJustifyItems);
        this.register("justify_self", HuiJustifySelf);
        this.register("left", HuiLeft);
        this.register("margin_bottom", HuiMarginBottom);
        this.register("margin_left", HuiMarginLeft);
        this.register("margin_right", HuiMarginRight);
        this.register("margin_top", HuiMarginTop);
        this.register("max_height", HuiMaxHeight);
        this.register("max_width", HuiMaxWidth);
        this.register("min_height", HuiMinHeight);
        this.register("min_width", HuiMinWidth);
        this.register("overflow_clip_box", HuiOverflowClipBox);
        this.register("overflow_clip_margin", HuiOverflowClipMargin);
        this.register("overflow_x", HuiOverflowX);
        this.register("overflow_y", HuiOverflowY);
        this.register("padding_bottom", HuiPaddingBottom);
        this.register("padding_left", HuiPaddingLeft);
        this.register("padding_right", HuiPaddingRight);
        this.register("padding_top", HuiPaddingTop);
        this.register("position_type", HuiPositionType);
        this.register("right", HuiRight);
        this.register("row_gap", HuiRowGap);
        this.register("scale_x", HuiScaleX);
        this.register("scale_y", HuiScaleY);
        this.register("scroll_position_x", HuiScrollPositionX);
        this.register("scroll_position_y", HuiScrollPositionY);
        this.register("top", HuiTop);
        this.register("visibility", HuiVisibility);
        this.register("width", HuiWidth);
        this.register("z_index", HuiZIndex);

        this
    }

    pub fn observe(&self, key: impl Into<String>, entity_commands: &mut EntityCommands) {
        if let Some(typ) = self.get(&key.into()) {
            typ.observe(entity_commands);
        }
    }
}