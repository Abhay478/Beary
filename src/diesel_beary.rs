#![allow(dead_code)]
use crate::{schema::*, Null, Pool, Res};
// use argon2::{Argon2, password_hash::SaltString, PasswordHasher};
use argon2::Argon2;
const DATABASE_DIR: &str =
    "/Library/Group Containers/9K33E3U3T4.net.shinyfrog.bear/Application Data";
use diesel::{prelude::*, r2d2::ConnectionManager};
use std::{
    env,
    fs::File,
    io::{Read, Write},
    process,
};
// use Texas::{component::*, document::*, *};
use Texas::*;

#[derive(Debug, Queryable)]
#[diesel(table_name = ZSFNOTE)]
pub struct Note {
    #[diesel(column_name = ZTITLE)]
    pub title: Option<String>,
    #[diesel(column_name = ZTEXT)]
    pub text: Option<String>,
}
#[derive(Debug)]
pub struct SureNote {
    title: String,
    text: String,
    tags: Vec<String>,
}

#[derive(Debug)]
pub struct Folder {
    elements: Vec<SureNote>,
    title: String,
}

pub fn connect() -> Res<Pool> {
    let url = format!("{}{DATABASE_DIR}/database.sqlite", env::var("HOME")?,);
    let cn = ConnectionManager::<SqliteConnection>::new(url);
    Ok(r2d2::Pool::builder().build(cn)?)
}

pub fn load_by_tags(db: &mut SqliteConnection, v: Vec<String>) -> Res<Vec<SureNote>> {
    use self::ZSFNOTE::dsl::*;
    let mut out: Vec<Vec<Note>> = vec![];
    for s in v.iter() {
        out.push(
            ZSFNOTE
                .filter(ZTEXT.like(format!("%#{}%", s)))
                .select((ZTITLE, ZTEXT))
                .get_results::<Note>(db)?,
        );
    }
    let out = out
        .into_iter()
        .flatten()
        .filter(|x| x.title.is_some() && x.text.is_some())
        .map(|n| parse_note(n))
        .collect::<Vec<_>>();

    Ok(out)

    // todo!()
}

pub fn load_by_title(db: &mut SqliteConnection, title: &str) -> Res<SureNote> {
    use self::ZSFNOTE::dsl::*;

    let note = parse_note(
        ZSFNOTE
            .filter(ZTITLE.eq(title))
            .select((ZTITLE, ZTEXT))
            .get_result::<Note>(db)?,
    );
    Ok(note)
}

fn parse_note(n: Note) -> SureNote {
    let x = n.text.unwrap();
    let tags = get_tags(&x);
    let v1 = x
        .lines()
        .filter(|c| !c.starts_with("#"))
        .map(|s| {
            if s.is_empty() {
                "\n".to_string()
            } else {
                format!("{}\n", s)
            }
        })
        // .map(|s| s.replace("*", "**").replace("/", "*"))
        .collect::<String>();

    SureNote {
        title: n.title.unwrap(),
        text: v1,
        tags,
    }
    // todo!()
}

pub fn load_part(db: &mut SqliteConnection, part: &str) -> Res<Folder> {
    use self::ZSFNOTE::dsl::*;
    let note = parse_note(
        ZSFNOTE
            .filter(ZTITLE.like(part))
            .select((ZTITLE, ZTEXT))
            .get_result::<Note>(db)?,
    );
    let ind = parse_index(&note);

    match ind {
        None => Ok(Folder {
            elements: vec![note],
            title: part.to_string(),
        }),
        Some(ind) => {
            let mut out = Vec::with_capacity(ind.elements.len());

            for s in ind.elements.iter() {
                out.push(parse_note(
                    ZSFNOTE
                        .filter(ZTITLE.like(s))
                        .select((ZTITLE, ZTEXT))
                        .get_result::<Note>(db)?,
                ))
            }

            Ok(Folder {
                elements: out,
                title: part.to_string(),
            })
        }
    }
}

pub struct IndexNode {
    pub elements: Vec<String>,
    pub title: String,
    pub part: Option<String>,
}

fn parse_index(note: &SureNote) -> Option<IndexNode> {
    let req = note.tags.iter().rfind(|q| q.ends_with("Index"));
    match req {
        None => None,
        Some(s) => {
            let v = s.split("/").collect::<Vec<&str>>();
            Some(IndexNode {
                elements: note
                    .text
                    .lines()
                    .filter(|x| !x.is_empty())
                    .map(|x| x.replace("[", "").replace("]", ""))
                    .collect::<Vec<_>>(),
                title: v[0].to_string(),
                part: if v.len() > 2 {
                    Some(v[1].to_string())
                } else {
                    None
                },
            })
        }
    }
}

fn get_tags(note: &String) -> Vec<String> {
    note.lines()
        .filter(|l| l.starts_with("#") && !l.starts_with(r"#+ "))
        .map(|x| x[1..].to_string())
        .collect()
}

pub fn load_book(db: &mut SqliteConnection, title: &str) -> Res<Vec<Folder>> {
    use self::ZSFNOTE::dsl::*;
    let s: SureNote = parse_note(
        ZSFNOTE
            .filter(ZTEXT.like(format!("%#{}/Index%", title)))
            .select((ZTITLE, ZTEXT))
            .get_result::<Note>(db)?,
    );
    let node = parse_index(&s).unwrap();

    let mut out = vec![];

    for t in node.elements.iter() {
        out.push(load_part(db, t)?);
    }

    Ok(out)
}

#[derive(Debug)]
pub struct Count {
    pub characters: Option<usize>,
    pub words: Option<usize>,
    pub lines: Option<usize>,
}

pub fn word_count(db: &mut SqliteConnection, title: &str, counter: &mut Count) -> Null {
    use self::ZSFNOTE::dsl::*;
    let notbook: Vec<Note> = ZSFNOTE
        .filter(ZTEXT.like(format!("%#{}%", title)))
        .select((ZTITLE, ZTEXT))
        .get_results::<Note>(db)?;

    let book = notbook
        .into_iter()
        .filter(|x| x.title.is_some() && x.text.is_some() && x.title.as_ref().unwrap() != "Index")
        .map(|x| parse_note(x))
        .collect::<Vec<_>>();

    if let Some(_) = counter.words {
        counter.words = Some(
            book.iter()
                .map(|x| {
                    x.text
                        .lines()
                        .flat_map(|s| s.split(" "))
                        .collect::<Vec<&str>>()
                        .len()
                })
                .sum(),
        )
    }

    if let Some(_) = counter.lines {
        counter.lines = Some(
            book.iter()
                .map(|x| x.text.lines().map(|x| x).collect::<Vec<&str>>().len())
                .sum(),
        )
    }

    if let Some(_) = counter.characters {
        counter.characters = Some(book.iter().map(|x| x.text.len()).sum());
    }

    Ok(())
    // todo!()
}

pub fn latexit(
    db: &mut SqliteConnection,
    title: &str,
    path: Option<String>,
    author: String,
) -> Res<File> {
    let mut f = File::create(format!(
        "{}{DATABASE_DIR}/tex/{title}.tex",
        env::var("HOME")?,
    ))?;
    let mut q = load_book(db, title)?;

    let mut doc = document!("book");
    doc.metadata = Metadata::new(title, &*author);
    doc.metadata.tableofcontents = true;
    let mut parskip = Package::new("parskip");
    parskip.add_option("parfill");
    doc.new_package(parskip);

    let chaps = q
        .iter_mut()
        .map(|x| {
            let mut p = Part::new(&*x.title);
            let cvec = x
                .elements
                .iter_mut()
                .map(|note| {
                    let mut ch = Chapter::new(&note.title);
                    ch.attach_vec(parse_text(&mut note.text)).unwrap();
                    ch.into()
                })
                .collect::<Vec<Component>>();
            p.attach_vec(cvec).unwrap();
            p.into()
        })
        .collect::<Vec<Component>>();

    for p in chaps {
        doc.new_component(p);
    }

    write!(f, "{}", doc.to_string())?;
    let q = compile(title, path)?;
    match q {
        Some(x) => Ok(x),
        None => Ok(f),
    }
}

fn parse_text(text: &mut String) -> Vec<Component> {
    *text = text.trim().to_string();
    let bindings = [
        ("/", TextType::Italic),
        ("*", TextType::Bold),
        ("_", TextType::Underlined),
    ];

    for b in bindings {
        let mut chunks = text
            .split(b.0)
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        for it in chunks.chunks_exact_mut(2) {
            it[1] = TextChunk::new(&*it[1], b.1.clone()).to_string()
        }
        *text = chunks.into_iter().collect::<String>();
    }

    let paras = text
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let lines = paras
        .iter()
        .map(|p| p.split("\n").map(|l| l.to_string()).collect())
        .collect::<Vec<Vec<String>>>();

    let out = lines
        .iter()
        .map(|p| {
            let mut pa = Paragraph::new();
            pa.attach_vec(
                p.iter()
                    .map(|l| {
                        let mut li = Line::new();
                        li.attach(TextChunk::new(&*l, TextType::Normal).into())
                            .unwrap();
                        li.into()
                    })
                    .collect::<Vec<Component>>(),
            )
            .unwrap();
            pa.into()
        })
        .collect::<Vec<Component>>();

    return out;
    // todo!()
}

pub fn texnote(db: &mut SqliteConnection, title: &str, path: Option<String>) -> Res<File> {
    let mut f = File::create(format!(
        "{}{}/tex/{}.tex",
        env::var("HOME")?,
        DATABASE_DIR,
        title
    ))?;

    let mut doc = document!("article");
    doc.metadata = Metadata::new(title, "");
    doc.new_package(package!("parskip", "parfill"));

    let mut sn = load_by_title(db, title)?;
    let comps = parse_text(&mut sn.text);
    for c in comps {
        doc.new_component(c);
    }

    write!(f, "{}", doc.to_string())?;
    let q = compile(title, path)?;
    match q {
        Some(x) => Ok(x),
        None => Ok(f),
    }
}

fn compile(title: &str, path: Option<String>) -> Res<Option<File>> {
    let c = process::Command::new("lualatex")
        .current_dir(format!("{}{}", env::var("HOME")?, DATABASE_DIR))
        .args(["--output-directory=pdf", &format!("tex/{title}.tex")])
        .output()?;

    if !c.status.success() {
        println!("Lualatex error. Try again.");
    }
    let _c = process::Command::new("rm")
        .current_dir(format!("{}{}/pdf", env::var("HOME")?, DATABASE_DIR))
        .args(["*.log", "*.aux"])
        .output()?;

    let f = if path.is_some() {
        let p = path.unwrap();
        let mut f = File::create(&p)?;

        let mut ff = File::open(format!(
            "{}{}/pdf/{title}.pdf",
            env::var("HOME")?,
            DATABASE_DIR
        ))?;
        let mut buf = vec![];
        ff.read_to_end(&mut buf)?;
        f.write_all(&mut buf)?;
        Some(f)
    } else {
        None
    };
    Ok(f)
}

pub fn configure(pwd: String) -> Null {
    let mut f = File::create(&format!("{}/.config/bear", env::var("HOME")?))?;
    let salt = b"this_is_not_a_salt";
    let mut out = [0u8; 32];
    Argon2::default()
        .hash_password_into(pwd.as_bytes(), salt, &mut out)
        .unwrap();
    f.write_all(&mut out)?;
    f.flush()?;
    Ok(())
}
