// Generates the blog templates and Rust module

#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

const MODULE_NAME: &str = "blog";

// Compiles drafts to templates and generates struct
#[derive(Parser)]
#[grammar = "draft.pest"]
struct Draft;

#[derive(Debug, Default, Clone)]
pub struct BlogPost {
    pub cover_image: Option<String>,
    pub description: Option<String>,
    pub edited: Option<String>, // only if published
    pub id: usize,
    pub published: bool,
    pub markdown: String,
    pub url_name: String,
    pub tags: String, // TODO Vec<String>
    pub title: String,
}

impl BlogPost {
    fn new(id: usize, path: PathBuf) -> Self {
        // Init empty post
        let mut ret = Self::default();
        ret.id = id;
        ret.url_name = format!(
            "/{}",
            path.file_stem().unwrap().to_str().unwrap().to_string()
        );

        // fill in struct from draft
        let md_file = fs::read_to_string(path.to_str().unwrap()).expect("Could not read draft");
        let parse_tree = Draft::parse(Rule::draft, &md_file)
            .expect("Could not parse draft")
            .next()
            .unwrap();
        // cycle through each attribute
        // unwrap is safe - if it parsed, there are between 3 and 6
        let mut parse_tree_inner = parse_tree.into_inner();

        // set header
        let header = parse_tree_inner.next().unwrap();
        let attributes = header.into_inner();
        for attr in attributes {
            let mut name: &str = "";
            let mut value: &str = "";
            for attr_part in attr.into_inner() {
                match attr_part.as_rule() {
                    Rule::key => name = attr_part.as_str(),
                    Rule::value => value = attr_part.as_str(),
                    _ => unreachable!(),
                }
            }
            match name {
                "cover_image" => ret.cover_image = Some(value.to_string()),
                "description" => ret.description = Some(value.to_string()),
                "edited" => ret.edited = Some(value.to_string()),
                "published" => {
                    ret.published = match value {
                        "true" => true,
                        _ => false,
                    }
                }
                "tags" => ret.tags = value.to_string(),
                "title" => ret.title = value.to_string(),
                _ => {}
            }
        }

        // set body
        let body = parse_tree_inner.next().unwrap();
        ret.markdown = body.as_str().to_string();

        // done
        ret
    }
    fn link_info(&'static self) -> LinkInfo {
        LinkInfo {
            id: self.id,
            title: &self.title,
            url_name: &self.url_name,
        }
    }
    fn get_template(&self) -> String {
        unimplemented!()
    }
}

#[derive(Debug, Default)]
pub struct Blog {
    pub drafts: Vec<BlogPost>,
    pub published: Vec<BlogPost>,
}

impl Blog {
    fn new() -> Self {
        let mut ret = Blog::default();
        // scrape posts
        let paths = fs::read_dir("blog").expect("Should locate blog directory");
        for path in paths {
            let path = path.expect("Could not open draft").path();
            let post = BlogPost::new(ret.total(), path);
            if post.published {
                ret.published.push(post);
            } else {
                ret.drafts.push(post);
            }
        }
        ret
    }
    fn link_info(&'static self) -> BlogLinkInfo {
        BlogLinkInfo {
            published: self.published.iter().map(|bp| bp.link_info()).collect(),
            drafts: self.drafts.iter().map(|bp| bp.link_info()).collect(),
        }
    }
    fn total(&self) -> usize {
        self.drafts.len() + self.published.len()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LinkInfo<'a> {
    pub id: usize,
    pub title: &'a str,
    pub url_name: &'a str,
}

#[derive(Debug, Default)]
pub struct BlogLinkInfo<'a> {
    published: Vec<LinkInfo<'a>>,
    drafts: Vec<LinkInfo<'a>>,
}

trait BuildGen {
    fn to_file(file: &mut fs::File) -> Result<(), io::Error>;
}

impl BuildGen for BlogPost {
    fn to_file(file: &mut fs::File) -> Result<(), io::Error> {
        writeln!(file, "// This module was autogenerated by build.rs\n")?;
        writeln!(file, "#[derive(Debug, Default, Clone)]")?;
        writeln!(file, "pub struct BlogPost {{");
        writeln!(file, "    pub cover_image: Option<String>,")?;
        writeln!(file, "    pub description: Option<String>,")?;
        writeln!(file, "    pub edited: Option<String>,")?;
        writeln!(file, "    pub id: usize,")?;
        writeln!(file, "    pub published: bool,")?;
        writeln!(file, "    pub markdown: String,")?;
        writeln!(file, "    pub url_name: String,")?;
        writeln!(file, "    pub tags: String,")?;
        writeln!(file, "    pub title: String,")?;
        writeln!(file, "}}\n");
        Ok(())
    }
}

impl<'a> BuildGen for LinkInfo<'a> {
    fn to_file(file: &mut fs::File) -> Result<(), io::Error> {
        writeln!(file, "#[derive(Debug, Clone, Copy, Default)]")?;
        writeln!(file, "pub struct LinkInfo {{")?;
        writeln!(file, "    pub id: usize,")?;
        writeln!(file, "    pub url_name: &'static str,")?;
        writeln!(file, "    pub title: &'static str,")?;
        writeln!(file, "}}\n")?;
        Ok(())
    }
}

impl<'a> BuildGen for BlogLinkInfo<'a> {
    fn to_file(file: &mut fs::File) -> Result<(), io::Error> {
        writeln!(file, "#[derive(Debug, Default)]")?;
        writeln!(file, "pub struct BlogLinkInfo {{")?;
        writeln!(file, "    pub drafts: Vec<LinkInfo>,")?;
        writeln!(file, "    pub published: Vec<LinkInfo>,")?;
        writeln!(file, "}}\n")?;
        Ok(())
    }
}

fn generate_posts(blog: &Blog) {}

fn generate_module(blog: &Blog) -> Result<(), io::Error> {
    let mut module = fs::File::create(&format!("src/{}.rs", MODULE_NAME))?;
    writeln!(module, "use lazy_static::lazy_static;\n")?;
    // types
    LinkInfo::to_file(&mut module)?;
    BlogLinkInfo::to_file(&mut module)?;

    // links

    writeln!(module, "lazy_static! {{")?;
    writeln!(module, "    pub static ref LINKINFO: BlogLinkInfo = {{")?;
    writeln!(module, "        let mut ret = BlogLinkInfo::default();")?;
    for p in &blog.drafts {
        writeln!(
            module,
            "        ret.drafts.push(LinkInfo {{ id: {}, title: \"{}\", url_name: \"{}\" }});",
            p.id, p.title, p.url_name
        )?;
    }
    for p in &blog.published {
        writeln!(
            module,
            "        ret.published.push(LinkInfo {{ id: {}, title: \"{}\", url_name: \"{}\" }});",
            p.id, p.title, p.url_name
        )?;
    }
    writeln!(module, "        ret")?;
    writeln!(module, "    }};\n}}")?;

    Ok(())
}

fn generate(blog: &Blog) {
    generate_posts(blog);
    generate_module(blog);
}

fn main() {
    let blog = Blog::new();
    println!("cargo:rerun-if-changed=blog");
    for p in &blog.drafts {
        println!("cargo:rerun-if-changed=blog/{}.md", p.url_name);
    }
    for p in &blog.published {
        println!("cargo:rerun-if-changed=blog/{}.md", p.url_name);
    }
    generate(&blog);
}