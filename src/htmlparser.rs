extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use std::default::Default;
use std::fs::File;
use std::io::{self, Write};
use std::io::Read;

use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, serialize};
use rcdom::{RcDom, SerializableHandle, Handle};
use rocket::futures::AsyncReadExt;

pub fn parse_html(data:&String) -> RcDom{
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut f = File::open("foo.html").unwrap();

    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut f)
        .unwrap();

    let document: SerializableHandle = dom.document.clone().into();
    // serialize(&mut io::stdout(), &document, Default::default())
    //     .ok()
    //     .expect("serialization failed");

    walk(4, &dom.document);

    return dom;
}

fn print_node(document:&Handle) {
    let data:&core::cell::Ref<Vec<Handle>> = &document.children.borrow();

    if(data.len() > 0) {
        for item in data.iter() {
            print_node(item)
        }
    } else {
        let data = &document.clone().data;
        println!("{data:?}");
    }
}

fn walk(indent: usize, handle: &Handle) {
    let node = handle;
    for _ in 0..indent { print!(" "); }
    match node.data {
        rcdom::NodeData::Document => println!("#Document"),

        rcdom::NodeData::Doctype {
            ref name,
            ref public_id,
            ref system_id,
        } => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

        rcdom::NodeData::Text { ref contents } => {
            println!("#text: {}", contents.borrow().escape_default())
        },

        rcdom::NodeData::Comment { ref contents } => println!("<!-- {} -->", contents.escape_default()),

        rcdom::NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            // assert!(name.ns == ns!(html));
            print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                // assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        },

        rcdom::NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    for child in node.children.borrow().iter() {
        walk(indent + 4, child);
    }
}