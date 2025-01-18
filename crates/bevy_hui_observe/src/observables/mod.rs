use bevy::{prelude::*, text::FontSmoothing, ui::{widget::NodeImageMode, FocusPolicy}};

pub enum HuiObservationValue {
    Bool(bool),
    I32(i32),
    F32(f32),
    OptionF32(Option<f32>),
    String(String),
    Color(Color),
    Display(Display),
    ImageMode(NodeImageMode),
    FontSmoothing(FontSmoothing),
    JustifyText(JustifyText),
    JustifyContent(JustifyContent),
    JustifyItems(JustifyItems),
    JustifySelf(JustifySelf),
    Visibility(Visibility),
    OverflowClipBox(OverflowClipBox),
    LineBreak(LineBreak),
    AlignItems(AlignItems),
    AlignContent(AlignContent),
    AlignSelf(AlignSelf),
    FlexDirection(FlexDirection),
    FlexWrap(FlexWrap),
    Overflow(Overflow),
    PositionType(PositionType),
    Val(Val),
    FocusPolicy(FocusPolicy),
}

#[derive(Event)]
pub struct HuiObservationEvent<T> {
    _marker: core::marker::PhantomData<T>,
    value: HuiObservationValue,
}

//start mods
pub mod image;
pub mod node;
pub mod observable;
pub mod text;
//end mods