use crate::color::{Color};
use crate::style::{Styles};

#[derive(Clone, Copy, Debug)]
pub struct ColorStyle {
    pub bgcolor: Option<Color>,
    pub fgcolor: Option<Color>,
    pub style: Styles,
}

impl ColorStyle {
    pub fn new() -> ColorStyle {
        ColorStyle{
            bgcolor: None,
            fgcolor: None,
            style: Styles::Clear,
        }
    }

    pub fn bold(&self) -> ColorStyle {
        ColorStyle{
            bgcolor: self.bgcolor,
            fgcolor: self.fgcolor,
            style: Styles::Bold,
        }
    }

    pub fn prefix(&self) -> String {
        let mut res = String::from("\x1B[");
        let mut has_wrote = if self.style != Styles::Clear {
            res.push_str(&self.style._to_str());
            true
        } else {
            false
        };

        if let Some(ref bgcolor) = self.bgcolor {
            if has_wrote {
                res.push(';');
            }

            res.push_str(&bgcolor.to_bg_str());
            has_wrote = true;
        }

        if let Some(ref fgcolor) = self.fgcolor {
            if has_wrote {
                res.push(';');
            }

            res.push_str(&fgcolor.to_fg_str());
        }

        res.push('m');
        res
    }

    pub fn suffix(&self) -> &'static str {
        "\x1B[0m"
    }
}
