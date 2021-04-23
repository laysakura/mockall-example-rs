use clap::{App, Arg, ArgMatches};
use domain::{EmailAddress, UserFirstName, UserLastName, UserName};
use interface_adapter::{AddUserRequestDTO, Controller};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(crate) struct Cli;

impl Cli {
    pub(crate) fn process_cmd() {
        let matches = Self::create_matches();

        if let Some(m) = matches.subcommand_matches("search") {
            Self::process_search_cmd(m);
        } else if let Some(m) = matches.subcommand_matches("add") {
            Self::process_add_cmd(m);
        }
        if let Some(m) = matches.subcommand_matches("update") {
            Self::process_update_cmd(m);
        } else {
            panic!("Invalid command. Run with --help for usage.")
        }
    }

    fn process_search_cmd(matches: &ArgMatches) {
        todo!()
    }
    fn process_add_cmd(matches: &ArgMatches) {
        let firstname = matches.value_of("firstname").expect("required");
        let lastname = matches.value_of("lastname").expect("required");
        let email = matches.value_of("email").expect("required");

        let req = AddUserRequestDTO {
            email: EmailAddress::new(email),
            name: UserName::new(UserFirstName::new(firstname), UserLastName::new(lastname)),
        };

        match Controller::add_user(req) {
            Ok(_res) => {
                eprintln!("Successfully added a user.")
            }
            Err(e) => {
                // TODO 丁寧なエラーハンドリング
                eprintln!("Failed to add a user: {:?}", e)
            }
        }
    }
    fn process_update_cmd(matches: &ArgMatches) {
        todo!()
    }

    fn create_matches() -> ArgMatches {
        let firstname_arg = Arg::new("firstname")
            .long("firstname")
            .short('f')
            .about("First name")
            .takes_value(true);
        let lastname_arg = Arg::new("lastname")
            .long("lastname")
            .short('l')
            .about("Last name")
            .takes_value(true);
        let email_arg = Arg::new("email")
            .long("email")
            .short('e')
            .about("Email address")
            .takes_value(true);

        App::new("User list")
            .version("1.0")
            .author("Sho Nakatani <lay.sakura@gmail.com>")
            .about("Example program to show how to use mockall crate.")
            .subcommand(
                App::new("search")
                    .about("Searches users by name and/or email address")
                    .arg(firstname_arg.clone())
                    .arg(lastname_arg.clone())
                    .arg(email_arg.clone()),
            )
            .subcommand(
                App::new("add")
                    .about("Adds a user")
                    .arg(firstname_arg.clone().required(true))
                    .arg(lastname_arg.clone().required(true))
                    .arg(email_arg.clone().required(true)),
            )
            .subcommand(
                App::new("update")
                    .about("Updates a user's name")
                    .arg(email_arg.clone().required(true))
                    .arg(firstname_arg.clone())
                    .arg(lastname_arg.clone()),
            )
            .get_matches()
    }
}
