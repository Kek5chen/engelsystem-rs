use std::borrow::Cow;

use validator::ValidationError;
use zeroize::Zeroizing;

pub fn validate_password(password: &Zeroizing<String>) -> Result<(), ValidationError> {
    if password.len() < 8 {
        Err(
            ValidationError::new("password_too_short").with_message(Cow::Borrowed(
                "Das Passwort muss mindestens 8 Zeichen beihnalten",
            )),
        )
    } else if password
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && !"!@#%^&_+=;:.,".contains(c))
    {
        Err(
            ValidationError::new("password_invalid_char").with_message(Cow::Borrowed(
                "Das Passwort darf nur A-z, 0-9, oder !@#%^&_+=;:., beinhalten",
            )),
        )
    } else {
        Ok(())
    }
}

pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 2 {
        Err(
            ValidationError::new("username_too_short").with_message(Cow::Borrowed(
                "Der Benutzername muss mindestens 2 Zeichen beihnalten",
            )),
        )
    } else if username
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && !"_.#".contains(c))
    {
        Err(
            ValidationError::new("username_invalid_char").with_message(Cow::Borrowed(
                "Der Benutzername darf nur A-z, 0-9, oder _.# beinhalten",
            )),
        )
    } else {
        Ok(())
    }
}
