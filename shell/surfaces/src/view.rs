//! View — the core building block of AuraOS UI
//! Views compose to form complete surfaces.

use super::{Alignment, EdgeInsets, Size};

/// The View enum — every UI element is a View
#[derive(Debug, Clone)]
pub enum View {
    // Primitives
    Text(TextView),
    Image(ImageView),
    Icon(IconView),
    Spacer(SpacerView),
    Divider(DividerView),

    // Containers
    VStack(StackView),      // Vertical stack
    HStack(StackView),      // Horizontal stack
    ZStack(StackView),      // Overlay stack (z-axis)
    Scroll(ScrollView),
    Padding(PaddingView),

    // Interactive
    Button(ButtonView),
    TextField(TextFieldView),
    Toggle(ToggleView),
    Slider(SliderView),

    // Surfaces
    Card(CardView),         // Glassmorphic card
    Sheet(SheetView),       // Bottom sheet
    Modal(ModalView),

    // Custom
    Custom(CustomView),
}

#[derive(Debug, Clone)]
pub struct TextView {
    pub text: String,
    pub size: f32,
    pub weight: FontWeight,
    pub color: Option<(u8, u8, u8, u8)>,
    pub align: TextAlign,
    pub max_lines: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin, Light, Regular, Medium, SemiBold, Bold, Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign { Left, Center, Right }

#[derive(Debug, Clone)]
pub struct ImageView {
    pub source: ImageSource,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub fit: ImageFit,
    pub corner_radius: f32,
}

#[derive(Debug, Clone)]
pub enum ImageSource {
    Path(String),
    Url(String),
    Bytes(Vec<u8>),
    SystemIcon(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFit { Fill, Fit, Cover, None }

#[derive(Debug, Clone)]
pub struct IconView {
    pub name: String,
    pub size: f32,
    pub color: Option<(u8, u8, u8, u8)>,
}

#[derive(Debug, Clone, Copy)]
pub struct SpacerView {
    pub min_size: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct DividerView {
    pub thickness: f32,
    pub color: Option<(u8, u8, u8, u8)>,
}

#[derive(Debug, Clone)]
pub struct StackView {
    pub children: Vec<View>,
    pub spacing: f32,
    pub alignment: Alignment,
    pub padding: EdgeInsets,
}

#[derive(Debug, Clone)]
pub struct ScrollView {
    pub child: Box<View>,
    pub direction: ScrollDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection { Vertical, Horizontal, Both }

#[derive(Debug, Clone)]
pub struct PaddingView {
    pub child: Box<View>,
    pub insets: EdgeInsets,
}

#[derive(Debug, Clone)]
pub struct ButtonView {
    pub label: Box<View>,
    pub style: ButtonStyle,
    pub action_id: String,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle { Primary, Secondary, Ghost, Danger }

#[derive(Debug, Clone)]
pub struct TextFieldView {
    pub placeholder: String,
    pub value: String,
    pub secure: bool,    // Password field
    pub field_id: String,
}

#[derive(Debug, Clone)]
pub struct ToggleView {
    pub label: String,
    pub is_on: bool,
    pub toggle_id: String,
}

#[derive(Debug, Clone)]
pub struct SliderView {
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub step: f32,
    pub slider_id: String,
}

#[derive(Debug, Clone)]
pub struct CardView {
    pub child: Box<View>,
    pub blur: bool,          // Enable glassmorphism
    pub corner_radius: f32,
    pub padding: EdgeInsets,
}

#[derive(Debug, Clone)]
pub struct SheetView {
    pub child: Box<View>,
    pub detents: Vec<f32>,   // Heights as fraction of screen (0.5 = half)
    pub current_detent: usize,
}

#[derive(Debug, Clone)]
pub struct ModalView {
    pub title: String,
    pub child: Box<View>,
    pub dismissible: bool,
}

#[derive(Debug, Clone)]
pub struct CustomView {
    pub render_id: String,
    pub data: Vec<u8>,
}
