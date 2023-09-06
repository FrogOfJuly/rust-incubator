use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

#[allow(dead_code)]
struct EmailError {
    cause: String,
}

struct EmailString<'a>(Cow<'a, str>);

impl<'a> EmailString<'a> {
    fn new(email: Cow<'a, str>) -> Self {
        EmailString(email)
    }

    fn check_email(value: &str) -> Option<EmailError> {
        let mut parts = value.split('@');
        let Some(left) = parts.next() else {
            return Some(EmailError {
                cause: "Exactly one @ is required".to_owned(),
            });
        };

        let Some(right) = parts.next() else {
            return Some(EmailError {
                cause: "Exactly one @ is required".to_owned(),
            });
        };

        if parts.count() != 0 {
            return Some(EmailError {
                cause: "Exactly one @ is required".to_owned(),
            });
        }

        if left.is_empty() || right.is_empty() {
            return Some(EmailError {
                cause: "@ can't be at the start or the end of an email".to_owned(),
            });
        }

        if !right.contains('.') {
            return Some(EmailError {
                cause: "Domen must be specified after @".to_owned(),
            });
        }
        None
    }
}

impl<'a> TryFrom<String> for EmailString<'a> {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(err) = Self::check_email(value.as_str()) {
            return Err(err);
        }
        Ok(EmailString(Cow::Owned(value)))
    }
}

impl<'a> TryFrom<&'a str> for EmailString<'a> {
    type Error = EmailError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some(err) = Self::check_email(value) {
            return Err(err);
        }
        Ok(EmailString::new(Cow::Borrowed(value)))
    }
}

impl<'a> From<EmailString<'a>> for String {
    fn from(val: EmailString) -> Self {
        val.0.into_owned()
    }
}

impl<'a> AsRef<str> for EmailString<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> AsMut<str> for EmailString<'a> {
    fn as_mut(&mut self) -> &mut str {
        self.0.to_mut()
    }
}

// impl Deref for EmailString {
//     type Target = str;

//     fn deref(&self) -> &Self::Target {
//         self.0.as_ref()
//     }
// }

struct Random<T>(T, T, T);

impl<T> Random<T> {
    fn choose(&self) -> &T {
        match rand::random::<usize>() % 3 {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => unreachable!(),
        }
    }

    fn choose_mut(&mut self) -> &mut T {
        match rand::random::<usize>() % 3 {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => unreachable!(),
        }
    }
}

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.choose()
    }
}

impl<T> DerefMut for Random<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.choose_mut()
    }
}

// impl<T> AsRef<T> for Random<T> {
//     fn as_ref(&self) -> &T {
//         self.choose()
//     }
// }

// impl<T> AsMut<T> for Random<T> {
//     fn as_mut(&mut self) -> &mut T {
//         self.choose_mut()
//     }
// }

fn main() {
    println!("Implement me!");
}
