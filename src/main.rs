use clap::{Parser, Subcommand};
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use std::error::Error;

mod diesel_beary;
mod schema;
use diesel_beary::*;

type Res<T> = Result<T, Box<dyn Error>>;
type Null = Res<()>;
type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Parser, Debug)]
#[command(name = "beary")]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    WC {
        title: String,
        #[arg(short)]
        l: bool,
        #[arg(short)]
        w: bool,
        #[arg(short)]
        c: bool,
    },
    Collate {
        title: String,
    },
    Texdump {
        title: String,
        #[arg(short, long)]
        path: Option<String>,
        #[arg(short, long)]
        author: Option<String>,
    },
    Password {
        pwd: String,
    },
    Export {
        title: String,
        #[arg(short, long)]
        path: Option<String>,
    },
}

fn main() -> Null {
    dotenvy::dotenv()?;
    let pool = connect()?;
    let args = Args::parse();

    match args.command.unwrap() {
        Commands::Collate { title } => {
            // dbg!(&tag);
            let x = load_book(&mut pool.get().unwrap(), &title)?;
            dbg!(x);
        }
        Commands::WC { title, l, w, c } => {
            let mut counter = Count {
                lines: if l { Some(0) } else { None },
                words: if w { Some(0) } else { None },
                characters: if c { Some(0) } else { None },
            };

            if !l && !w && !c {
                counter = Count {
                    lines: Some(0),
                    words: Some(0),
                    characters: Some(0),
                }
            }

            word_count(&mut pool.get().unwrap(), &title, &mut counter)?;
            dbg!(counter);
        }
        Commands::Texdump {
            title,
            path,
            author,
        } => {
            latexit(
                &mut pool.get().unwrap(),
                &title,
                path,
                match author {
                    Some(a) => a,
                    None => "".to_string(),
                },
            )?;
        }
        Commands::Password { pwd } => {
            configure(pwd)?;
        }
        Commands::Export { title, path } => {
            texnote(&mut pool.get().unwrap(), &title, path)?;
        }
    };
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use crate::{Null, rusq_beary::connect};

//     #[test]
//     fn conn_check() -> Null{
//         let c = connect()?;
//         // dbg!(&c);
//         // let c = c.unwrap();
//         let mut st = c.prepare("select ZTITLE from ZSFNOTE")?;
//         let rows = st.query_map([], |row| row.get::<usize, String>(0))?;

//         let mut out = vec![];
//         out.extend(rows);
//         dbg!(&out);
//         Ok(())
//     }
// }
