use std::ops::{Deref, DerefMut};

#[allow(dead_code)]
struct EmailError {
    cause: String,
}

struct EmailString(String);

impl EmailString {
    fn new(email: String) -> Self {
        EmailString(email)
    }
}

impl TryFrom<String> for EmailString {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts = value.split('@').collect::<Vec<_>>();
        let [left, right] = parts.as_slice() else {
            return Err(EmailError {
                cause: "Only one @ permitted".to_owned(),
            });
        };

        if left.is_empty() || right.is_empty() {
            return Err(EmailError {
                cause: "@ can't be at the start or the end of an email".to_owned(),
            });
        }

        if !right.contains('.') {
            return Err(EmailError {
                cause: "Domen must be specified after @".to_owned(),
            });
        }

        Ok(EmailString::new(value))
    }
}

impl From<EmailString> for String {
    fn from(val: EmailString) -> Self {
        val.0
    }
}

impl AsRef<str> for EmailString {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsMut<str> for EmailString {
    fn as_mut(&mut self) -> &mut str {
        self.0.as_mut_str()
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
