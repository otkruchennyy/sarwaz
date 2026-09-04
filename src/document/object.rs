// Поворот считается строго против часовой стрелки и от точки 2π

// вдлженные struct + impl for ... блоки

// impl Document(&mut self, obj: Vec<Obj>) {
//     fn create_layerr( &mut self, obj: Vec<Obj>) {
//         let id_name = self.layer_id_counter.generatw=e();
//         let layer = Layer::new(obj, id_name);
//         self.layers.push(layer);
//     }
// } // TODO: uncomment

struct Dot {
    x: f32,
    y: f32,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

struct Scale {
    scale_x_before: f32, // тут скейл в edit
    scale_y_before: f32,
    scale_x_after: f32, // тут скейл после edit
    scale_y_after: f32,
}

impl Default for Scale {
    fn default() -> Self {
        Self {
            scale_x_before: 1.0,
            scale_y_before: 1.0,
            scale_x_after: 1.0,
            scale_y_after: 1.0,
        }
    }
}

#[derive(Default)]
struct Location {
    x: f32,
    y: f32,
}

pub struct IdName {
    pub id: u16,
    pub name: String,
}

// enum блоки

#[derive(Default)]
enum FontWeight {
    Thin,       // 100
    ExtraLight, // 200
    Light,      // 300
    #[default]
    Normal, //     400
    Medium,     // 500
    SemiBold,   // 600
    Bold,       // 700
    ExtraBold,  // 800
    Black,      // 900
}

impl FontWeight {
    fn value(&self) -> u32 {
        match self {
            FontWeight::Thin => 100,
            FontWeight::ExtraLight => 200,
            FontWeight::Light => 300,
            FontWeight::Normal => 400,
            FontWeight::Medium => 500,
            FontWeight::SemiBold => 600,
            FontWeight::Bold => 700,
            FontWeight::ExtraBold => 800,
            FontWeight::Black => 900,
        }
    }
}

#[derive(Default)]
enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
    BoldItalic,
}

impl FontStyle {
    fn value(&self) -> String {
        match self {
            FontStyle::Normal => "Normal".to_string(),
            FontStyle::Italic => "Italic".to_string(),
            FontStyle::Oblique => "Oblique".to_string(),
            FontStyle::BoldItalic => "BoldItalic".to_string(),
        }
    }
}

#[derive(Default)]
enum TextAlignment {
    #[default]
    Left,
    Middle,
    Right,
}

impl TextAlignment {
    fn value(&self) -> String {
        match self {
            TextAlignment::Left => "Left".to_string(),
            TextAlignment::Middle => "Middle".to_string(),
            TextAlignment::Right => "Right".to_string(),
        }
    }
}

enum ImageSource {
    Path(String),
    Url(String),
}

impl Default for ImageSource {
    fn default() -> Self {
        ImageSource::Path(String::new())
    }
}

enum Obj {
    Circle(Circle),
    Triangle(Triangle),
    Line(Line),
    Polygon(Polygon),
    Text(Text),
    Image(Image),
}

// объекты struct + impl for ...

struct Circle {
    location: Location,
    scale: Scale,
    rotation: f32,
    radius: f32,
    line_width: f32,
    opacity: f32,
    is_visibility: bool,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            location: Location::default(),
            scale: Scale::default(),
            rotation: 0.0,
            radius: 100.0,
            line_width: 5.0,
            opacity: 1.0,
            is_visibility: true,
        }
    }
}

struct Triangle {
    location: Location,
    scale: Scale,
    rotation: f32,
    height: f32,
    opacity: f32,
    is_visibility: bool,
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            location: Location::default(),
            scale: Scale::default(),
            rotation: 0.0,
            height: 50.0,
            opacity: 1.0,
            is_visibility: true,
        }
    }
}

struct Line {
    dot_1: Location,
    dot_2: Location,
    scale: Scale,
    rotation: f32,
    opacity: f32,
    is_visibility: bool,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            dot_1: Location::default(),
            dot_2: Location::default(),
            scale: Scale::default(),
            rotation: 0.0,
            opacity: 1.0,
            is_visibility: true,
        }
    }
}

struct Polygon {
    dots: Vec<Dot>,
    scale: Scale,
    rotation: f32,
    opacity: f32,
    is_visibility: bool,
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            dots: Vec::new(),
            scale: Scale::default(),
            rotation: 0.0,
            opacity: 1.0,
            is_visibility: true,
        }
    }
}

struct Text {
    location: Location,
    content: String,
    font_family: String,
    font_size: f32,
    font_weight: FontWeight, // enum: 400(Normal), 500, ...
    font_style: FontStyle,   // enum: Normal(Normal), Italic, Oblique, ...
    color: Color,
    rotation: f32,
    scale: Scale,
    alignment: TextAlignment,
    opacity: f32,
    is_visibility: bool,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            location: Location::default(),
            content: "Lorem Ipsum".to_string(),
            font_family: "Arial".to_string(),
            font_size: 16.0,
            font_weight: FontWeight::default(),
            font_style: FontStyle::default(),
            color: Color::default(),
            rotation: 0.0,
            scale: Scale::default(),
            alignment: TextAlignment::default(),
            opacity: 1.0,
            is_visibility: true,
        }
    }
}

struct Image {
    source: ImageSource, // enum: url, path
    location: Location,
    scale: Scale,
    width: f32,
    height: f32,
    rotation: f32,
    opacity: f32,
    is_visibility: bool,
}

impl Image {
    fn new(source: ImageSource) -> Self {
        Self {
            source,
            location: Location::default(),
            scale: Scale::default(),
            width: 0.0,
            height: 0.0,
            rotation: 0.0,
            opacity: 1.0,
            is_visibility: true,
        }
    }
}

struct Layer {
    obj: Vec<Obj>,
    id: u16,
    opacity: f32,
    name: String,
    is_visibility: bool,
}

impl Layer {
    fn new(obj: Vec<Obj>, id_name: IdName) -> Self {
        Self {
            obj,
            id: id_name.id,
            opacity: 1.0,
            name: id_name.name,
            is_visibility: true,
        }
    }
}

struct Group {
    Layers: Vec<Layer>,
    id: u16,
    opacity: f32,
    name: String,
    is_visibility: bool,
}
