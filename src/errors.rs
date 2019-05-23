macro_rules! define_error {
    ($($vis:vis $name:ident { $( $vname:ident($vinto:path), )* })*) => {
        $(
            #[derive(Debug)]
            $vis enum $name {
                $(
                    $vname($vinto),
                )*
            }

            $(
                impl From<$vinto> for $name {
                    fn from(err: $vinto) -> Self {
                        $name::$vname(err)
                    }
                }
            )*
        )*
    }
}

define_error! {
    pub Error {
        User(UserError),

        Utf8(std::string::FromUtf8Error),

        Base64(base64::DecodeError),
        Diesel(diesel::result::Error),
        Bcrypt(bcrypt::BcryptError),
        Migrations(diesel_migrations::RunMigrationsError),
    }
}

#[derive(Debug)]
pub enum UserError {
    InvalidUsernameOrPassword,
    BadApiKey,
    BadApiRequest,
}
