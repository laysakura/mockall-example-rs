# mockall-example-rs

[mockall](https://docs.rs/mockall/0.9.1/mockall/) を紹介するためのリポジトリ。
[ブログ記事](TBD) にコードレベルの解説が書いてあります。

## 動かし方

超簡易メアド帳です。

```bash
💻 sho.nakatani@mbp2019 📂 ~/.ghq/src/github.com/laysakura/mockall-example-rs ⏰ 12:56:26
% cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/infra --help`
User list 1.0
Sho Nakatani <lay.sakura@gmail.com>
Example program to show how to use mockall crate.

USAGE:
    infra [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       Adds a user
    help      Prints this message or the help of the given subcommand(s)
    search    Searches users by name and/or email address
    update    Updates a user's name

💻 sho.nakatani@mbp2019 📂 ~/.ghq/src/github.com/laysakura/mockall-example-rs ⏰ 12:57:08
% cargo run -- search --firstname='Sho'
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/infra search --firstname=Sho`
Found users:
[]

💻 sho.nakatani@mbp2019 📂 ~/.ghq/src/github.com/laysakura/mockall-example-rs ⏰ 12:57:26
% cargo run -- add --firstname='Sho' --lastname='Nakatani' --email='lay.sakura@gmail.com'
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/infra add --firstname=Sho --lastname=Nakatani '--email=lay.sakura@gmail.com'`
Successfully added a user.

💻 sho.nakatani@mbp2019 📂 ~/.ghq/src/github.com/laysakura/mockall-example-rs ⏰ 12:57:54
% cargo run -- add --firstname='Sho' --lastname='Suzuki' --email='ssuzuki@example.com'
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/infra add --firstname=Sho --lastname=Suzuki '--email=ssuzuki@example.com'`
Successfully added a user.

💻 sho.nakatani@mbp2019 📂 ~/.ghq/src/github.com/laysakura/mockall-example-rs ⏰ 12:58:59
% cargo run -- search --firstname='Sho'
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/infra search --firstname=Sho`
Found users:
[
    User {
        id: UserId(
            3903593026370596298,
        ),
        name: UserName {
            first_name: UserFirstName(
                "Sho",
            ),
            last_name: UserLastName(
                "Nakatani",
            ),
        },
        email: EmailAddress {
            user: "lay.sakura",
            domain: "gmail.com",
        },
    },
    User {
        id: UserId(
            5134220031631565557,
        ),
        name: UserName {
            first_name: UserFirstName(
                "Sho",
            ),
            last_name: UserLastName(
                "Suzuki",
            ),
        },
        email: EmailAddress {
            user: "ssuzuki",
            domain: "example.com",
        },
    },
]

💻 sho.nakatani@mbp2019 📂 ~/.ghq/src/github.com/laysakura/mockall-example-rs ⏰ 12:59:05
% cargo run -- update --email='ssuzuki@example.com' --firstname='Shohei'
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/infra update '--email=ssuzuki@example.com' --firstname=Shohei`
Successfully updated a user.

💻 sho.nakatani@mbp2019 📂 ~/.ghq/src/github.com/laysakura/mockall-example-rs ⏰ 12:59:41
% cargo run -- search --firstname='Sho'
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/infra search --firstname=Sho`
Found users:
[
    User {
        id: UserId(
            3903593026370596298,
        ),
        name: UserName {
            first_name: UserFirstName(
                "Sho",
            ),
            last_name: UserLastName(
                "Nakatani",
            ),
        },
        email: EmailAddress {
            user: "lay.sakura",
            domain: "gmail.com",
        },
    },
]

💻 sho.nakatani@mbp2019 📂 ~/.ghq/src/github.com/laysakura/mockall-example-rs ⏰ 12:59:43
rm users.yml  # メアド帳の削除
```

