#[derive(Debug, Copy, Clone, Default)]
pub struct NodeLayout {
    pub align_x: HorizontalAlignment,
    pub align_y: VerticalAlignment,
    pub padding: Padding,
}

#[derive(Debug, Clone)]
pub struct ComponentNode {
    pub node: Node,
    pub layout: NodeLayout,
}

#[derive(Debug, Clone)]
pub enum Node {
    Button(ButtonNode),
    Text(TextNode),
    Column(ColumnNode),
    Row(RowNode),
}

#[derive(Debug, Clone)]
pub struct Component {
    pub content: Option<ComponentNode>,
    pub message_type: Option<String>,
    pub model_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ButtonNode {
    pub content: String, // TODO: Make more general
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct TextNode {
    pub content: String,
    pub size: f32,
}

#[derive(Debug, Clone)]
pub struct ColumnNode {
    pub content: Vec<Box<ComponentNode>>,
}

#[derive(Debug, Clone)]
pub struct RowNode {
    pub content: Vec<Box<ComponentNode>>,
}

#[derive(Debug, Copy, Clone)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
    Stretch,
}
impl Default for HorizontalAlignment {
    fn default() -> Self {
        Self::Left
    }
}
impl<'a> From<&str> for HorizontalAlignment {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "left" => HorizontalAlignment::Left,
            "center" => HorizontalAlignment::Center,
            "right" => HorizontalAlignment::Right,
            "stretch" => HorizontalAlignment::Stretch,
            _ => HorizontalAlignment::default(),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
    Stretch,
}
impl Default for VerticalAlignment {
    fn default() -> Self {
        Self::Top
    }
}
impl<'a> From<&str> for VerticalAlignment {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "top" => VerticalAlignment::Top,
            "center" => VerticalAlignment::Center,
            "bottom" => VerticalAlignment::Bottom,
            "stretch" => VerticalAlignment::Stretch,
            _ => VerticalAlignment::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Padding {
    pub value: f32,
}
