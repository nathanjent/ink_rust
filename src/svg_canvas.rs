
use conrod::{self, Colorable, Dimensions, Labelable, Point, Positionable, Widget};

/// The type upon which we'll implement the `Widget` trait.
pub struct SVGCanvas<'a, F> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    common: conrod::CommonBuilder,
    /// Optional label string for the button.
    maybe_label: Option<&'a str>,
    /// Optional callback for when the button is pressed. If you want the button to
    /// do anything, this callback must exist.
    maybe_react: Option<F>,
    /// See the Style struct below.
    style: Style,
    /// Whether the button is currently enabled, i.e. whether it responds to
    /// user input.
    enabled: bool
}

// We use the `widget_style!` macro to vastly simplify the definition and implementation of the
// widget's associated `Style` type. This generates both a `Style` struct, as well as an
// implementation that automatically retrieves defaults from the provided theme.
//
// See the documenation of the macro for a more details.
widget_style!{
    /// Represents the unique styling for our SVGCanvas widget.
    style Style {
        /// Color of the button.
        - color: conrod::Color { theme.shape_color }
        /// Color of the button's label.
        - label_color: conrod::Color { theme.label_color }
        /// Font size of the button's label.
        - label_font_size: conrod::FontSize { theme.font_size_medium }
    }
}

/// Represents the unique, cached state for our SVGCanvas widget.
#[derive(Clone, Debug, PartialEq)]
pub struct State {
    /// An index to use for our **Circle** primitive graphics widget.
    circle_idx: conrod::IndexSlot,
    /// An index to use for our **Text** primitive graphics widget (for the label).
    text_idx: conrod::IndexSlot,
}

/// Return whether or not a given point is over a circle at a given point on a
/// Cartesian plane. We use this to determine whether the mouse is over the button.
pub fn is_over_circ(circ_center: Point, mouse_point: Point, dim: Dimensions) -> bool {
    // Offset vector from the center of the circle to the mouse.
    let offset = conrod::utils::vec2_sub(mouse_point, circ_center);

    // If the length of the offset vector is less than or equal to the circle's
    // radius, then the mouse is inside the circle. We assume that dim is a square
    // bounding box around the circle, thus 2 * radius == dim[0] == dim[1].
    let distance = (offset[0].powf(2.0) + offset[1].powf(2.0)).sqrt();
    let radius = dim[0] / 2.0;
    distance <= radius
}

impl<'a, F> SVGCanvas<'a, F> {
    /// Create a button context to be built upon.
    pub fn new() -> SVGCanvas<'a, F> {
        SVGCanvas {
            common: conrod::CommonBuilder::new(),
            maybe_react: None,
            maybe_label: None,
            style: Style::new(),
            enabled: true,
        }
    }

    /// Set the reaction for the Button. The reaction will be triggered upon release
    /// of the button. Like other Conrod configs, this returns self for chainability.
    pub fn react(mut self, reaction: F) -> Self {
        self.maybe_react = Some(reaction);
        self
    }

    /// If true, will allow user inputs.  If false, will disallow user inputs.  Like
    /// other Conrod configs, this returns self for chainability. Allow dead code
    /// because we never call this in the example.
    #[allow(dead_code)]
    pub fn enabled(mut self, flag: bool) -> Self {
        self.enabled = flag;
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, F> Widget for SVGCanvas<'a, F>
    where F: FnMut()
{
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;

    fn common(&self) -> &conrod::CommonBuilder {
        &self.common
    }

    fn common_mut(&mut self) -> &mut conrod::CommonBuilder {
        &mut self.common
    }

    fn init_state(&self) -> State {
        State {
            circle_idx: conrod::IndexSlot::new(),
            text_idx: conrod::IndexSlot::new(),
        }
    }

    fn style(&self) -> Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: conrod::UpdateArgs<Self>) {
        let conrod::UpdateArgs { idx, state, rect, mut ui, style, .. } = args;

        let color = {
            let input = ui.widget_input(idx);

            // If the button was clicked, call the user's `react` function.
            if input.clicks().left().next().is_some() {
                if let Some(mut react) = self.maybe_react {
                    react();
                }
            }

            let color = style.color(ui.theme());
            input.mouse()
                .map(|mouse| {
                    if is_over_circ([0.0, 0.0], mouse.rel_xy(), rect.dim()) {
                        if mouse.buttons.left().is_down() {
                            color.clicked()
                        } else {
                            color.highlighted()
                        }
                    } else {
                        color
                    }
                })
                .unwrap_or(color)
        };

        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        // Conrod will automatically determine whether or not any changes have occurred and
        // whether or not any widgets need to be re-drawn.
        //
        // The primitive graphics widgets are special in that their unique state is used within
        // conrod's backend to do the actual drawing. This allows us to build up more complex
        // widgets by using these simple primitives with our familiar layout, coloring, etc
        // methods.
        //
        // If you notice that conrod is missing some sort of primitive graphics that you
        // require, please file an issue or open a PR so we can add it! :)

        // First, we'll draw the **Circle** with a radius that is half our given width.
        let radius = rect.w() / 2.0;
        let circle_idx = state.circle_idx.get(&mut ui);
        conrod::Circle::fill(radius)
            .middle_of(idx)
            .graphics_for(idx)
            .color(color)
            .set(circle_idx, &mut ui);

        // Now we'll instantiate our label using the **Text** widget.
        let label_color = style.label_color(ui.theme());
        let font_size = style.label_font_size(ui.theme());
        let text_idx = state.text_idx.get(&mut ui);
        if let Some(ref label) = self.maybe_label {
            conrod::Text::new(label)
                .middle_of(idx)
                .font_size(font_size)
                .graphics_for(idx)
                .color(label_color)
                .set(text_idx, &mut ui);
        }
    }

}

/// Provide the chainable color() configuration method.
impl<'a, F> Colorable for SVGCanvas<'a, F> {
    fn color(mut self, color: conrod::Color) -> Self {
        self.style.color = Some(color);
        self
    }
}

/// Provide the chainable label(), label_color(), and label_font_size()
/// configuration methods.
impl<'a, F> Labelable<'a> for SVGCanvas<'a, F> {
    fn label(mut self, text: &'a str) -> Self {
        self.maybe_label = Some(text);
        self
    }
    fn label_color(mut self, color: conrod::Color) -> Self {
        self.style.label_color = Some(color);
        self
    }
    fn label_font_size(mut self, size: conrod::FontSize) -> Self {
        self.style.label_font_size = Some(size);
        self
    }
}