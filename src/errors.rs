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
        Diesel(diesel::result::Error),
        User(UserError),
        Bcrypt(bcrypt::BcryptError),
        Migrations(diesel_migrations::RunMigrationsError),
    }

    pub UserError {}
}
