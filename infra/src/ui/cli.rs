use clap::{App, Arg};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(crate) struct Cli;

impl Cli {
    pub(crate) fn process_cmd() {
        let matches = App::new("User list")
            .version("1.0")
            .author("Sho Nakatani <lay.sakura@gmail.com>")
            .about("Example program to show how to use mockall crate.")
            .subcommand(
                App::new("search")
                    .about("Searches users by name and/or email address")
                    .arg(
                        Arg::new("firstname")
                            .long("firstname")
                            .short('f')
                            .about("First name"),
                    )
                    .arg(
                        Arg::new("lastname")
                            .long("lastname")
                            .short('l')
                            .about("Last name"),
                    )
                    .arg(
                        Arg::new("email")
                            .long("email")
                            .short('e')
                            .about("Email address"),
                    ),
            )
            .subcommand(
                App::new("add")
                    .about("Adds a user")
                    .arg(
                        Arg::new("firstname")
                            .long("firstname")
                            .short('f')
                            .about("First name")
                            .required(true),
                    )
                    .arg(
                        Arg::new("lastname")
                            .long("lastname")
                            .short('l')
                            .about("Last name")
                            .required(true),
                    )
                    .arg(
                        Arg::new("email")
                            .long("email")
                            .short('e')
                            .about("Email address")
                            .required(true),
                    ),
            )
            .subcommand(
                App::new("update")
                    .about("Updates a user's name")
                    .arg(
                        Arg::new("email")
                            .long("email")
                            .short('e')
                            .about("Email address to search a user")
                            .required(true),
                    )
                    .arg(
                        Arg::new("firstname")
                            .long("firstname")
                            .short('f')
                            .about("First name to update"),
                    )
                    .arg(
                        Arg::new("lastname")
                            .long("lastname")
                            .short('l')
                            .about("Last name to update"),
                    ),
            )
            .get_matches();

        todo!()
        // ここで CreateRequestDTO とかに詰めていき、presenに処理を移譲。エラーハンドリングも全部やる
    }
}
