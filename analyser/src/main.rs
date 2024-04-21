use serde::Serialize;
use std::fs;
use wikidump::{config, Parser};
use xml::ParserConfig;

fn extract_links(text: &str) -> Vec<&str> {
    // Regular expression to match wiki links
    let re = regex::Regex::new(r"\[\[([^\[\]|]+)(?:\|([^\[\]]+))?\]\]").unwrap();

    // Collect captured links
    let links: Vec<&str> = re
        .captures_iter(text)
        .flat_map(|cap| cap.get(1).or_else(|| cap.get(2)).map(|m| m.as_str()))
        .collect();

    links
}

#[derive(Serialize, Debug, Clone)]
struct Page<'a> {
    title: &'a str,
    links: Vec<&'a str>,
}

fn main() {
    let mut parser = Parser::new();
    let raw_parser = parser.process_text(false);

    let site = raw_parser
        .parse_file("../data.xml")
        .expect("Could not parse wikipedia dump file.");

    let mut pages: Vec<Page> = vec![];
    let mut page_titles: Vec<String> = vec![];

    println!("PARSED");

    for page in &site.pages {
        let rev = &page.revisions.last().unwrap();

        let links = extract_links(&rev.text);

        let filtered_links: Vec<&str> = links //filter links that are link to categories
            .iter()
            .filter(|link| !link.starts_with("Kategorija:"))
            .cloned()
            .collect();

        let page = Page {
            title: &page.title,
            links: filtered_links,
        };
        page_titles.push(page.title.to_lowercase());
        pages.push(page);
        println!("{}", pages.len());
    }
    println!("pushed");

    let mut filtered_pages: Vec<Page> = vec![];

    for page in pages.clone() {
        let mut filtered_links: Vec<&str> = vec![];

        for link in page.links {
            if page_titles.contains(&link.to_lowercase()) {
                filtered_links.push(link);
            }
        }
        filtered_pages.push(Page {
            title: page.title,
            links: filtered_links,
        });
        println!("{}", filtered_pages.len());
    }
    println!("writing");

    let json = serde_json::to_string(&filtered_pages).unwrap();
    fs::write("result.json", json).expect("Unable to write file");
}
