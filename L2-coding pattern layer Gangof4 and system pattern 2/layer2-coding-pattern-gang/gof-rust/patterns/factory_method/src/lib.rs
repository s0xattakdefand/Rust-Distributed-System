// TODO: src
//! Factory Method sample – GUI buttons.

pub trait Button { fn render(&self) -> String; }

struct HtmlButton;
impl Button for HtmlButton { fn render(&self) -> String { "<button>HTML</button>".into() } }

struct WindowsButton;
impl Button for WindowsButton { fn render(&self) -> String { "WindowsButton()".into() } }

pub trait Dialog {
    fn make_button(&self) -> Box<dyn Button>;
    fn render(&self) -> String {
        format!("Dialog->{}", self.make_button().render())
    }
}

pub struct HtmlDialog;
impl Dialog for HtmlDialog { fn make_button(&self) -> Box<dyn Button> { Box::new(HtmlButton) } }

pub struct WindowsDialog;
impl Dialog for WindowsDialog { fn make_button(&self) -> Box<dyn Button> { Box::new(WindowsButton) } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn renders() {
        assert!(HtmlDialog.render().contains("HTML"));
        assert!(WindowsDialog.render().contains("Windows"));
    }
}
