//! Well, hello there.
//! 
//! So some of you may know my rust-texas (formerly Texas) crate - well, 
//! I wrote that so I could write this.
//! If you don't, that's okay. It's pretty small. If you want to programmatically 
//! generate latex files, you might want to check it out, along with `tex-rs` 
//! 
//! 
//! I've used Bear quite a bit and what always annoyed me was that the txt and rtf 
//! formats weren't really as clean or expressive as the note itself, and we needed pro to 
//! export to pdf.
//! 
//! So, I did it myself.
//! 
//! This requires an installed lualatex compiler, because one of the functions fork/exec'   s it.
//! You absolutely need it.
//! 
//! Also, I'm almost certain that I've made some rookie mistakes in here in terms of 
//! performance and code cleanliness (diesel is kinda ugly sometimes, but it's not just that),
//! so if there's a bug or if something bugs you, please put up an issue.
//! 
//! Peace.
//! 



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
    /// Word count for all notes with a tag.
    WC {
        title: String,
        #[arg(short)]
        l: bool,
        #[arg(short)]
        w: bool,
        #[arg(short)]
        c: bool,
    },

    // Collate {
    //     title: String,
    // },

    /// Exports an entire book to pdf.
    /// Might add other target?
    Texdump {
        title: String,
        #[arg(short, long)]
        path: Option<String>,
        #[arg(short, long)]
        author: Option<String>,
    },
    // Password {
    //     pwd: String,
    // },

    /// Exports a single note to pdf.
    /// Might add other target?
    Export {
        title: String,
        #[arg(short, long)]
        path: Option<String>,
    },
}

fn main() -> Null {
    let pool = connect()?;
    let args = Args::parse();

    match args.command.unwrap() {
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
            // dbg!(counter);
            println!("{}", counter);
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
        // Want to learn more about how password protection works before trying this, don't want to nuke the databse.
        // Commands::Password { pwd } => {
        //     configure(pwd)?;
        // }
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
