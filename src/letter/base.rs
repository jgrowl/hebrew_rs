use super::*;

use Base::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Base {
    _NonEnd(NonEnd),
    _End(End),
    _Other(String)
}

impl Base {

    pub fn non_end(&self) -> Option<NonEnd> {
        match self {
            _NonEnd(non_end) => Some(non_end.clone()),
            _End(end) => Some(end.primitive_letter()),
            _ => None
        }
    }

    pub fn from_str<S: AsRef<str>>(stringlike: S) -> Self {
        let str_ref = stringlike.as_ref();
        let count = str_ref.chars().count();
        assert_eq!(1, count);

        if let Ok(non_end) = NonEnd::from_str(str_ref) {
            return Base::_NonEnd(non_end);
        }

        if let Ok(end) = End::from_str(str_ref) {
            return Base::_End(end);
        }

        _Other(str_ref.to_owned())
    }

    pub fn endify(&self) -> Base {
        match &self {
            Self::_NonEnd(non_end) => {
                let letter = non_end.end_letter();
                if let Some(l) = letter {
                    return Self::_End(l);
                }            
                return self.clone();
            },

            _ => self.clone()
        }
    }

    pub fn unendify(&self) -> Base {
        match &self {
            Self::_End(end) => { Self::_NonEnd(end.primitive_letter()) },
            _ => self.clone()
        }
    }
}

impl fmt::Display for Base {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let underlying = match self {
            Self::_NonEnd(non_end) => non_end.to_string(),
            Self::_End(end) => end.to_string(),
            Self::_Other(other) => other.to_owned()
        };
        write!(f, "{}", underlying)
    }
}


//#[cfg(test)]
//mod tests {
//    use super::*;
//
//}

