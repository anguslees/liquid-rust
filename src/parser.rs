use Renderable;
use LiquidOptions;
use output::Output;
use text::Text;
use std::slice::IterMut;
//use filters::{Variable, Value};
use lexer::Token;
use lexer::Token::{Identifier};
use lexer::Element;
use lexer::Element::{Expression, Tag, Raw};

pub fn parse<'a> (elements: Vec<Element>, options: &'a LiquidOptions) -> Vec<Box<Renderable + 'a>> {
    let mut ret = vec![];
    let mut iter = elements.iter();
    let mut token = iter.next();
    while token.is_some() {
        match token.unwrap() {
            &Expression(ref tokens,_) => ret.push(parse_expression(tokens, options)),
            &Tag(ref tokens,_) => ret.push(parse_tag(&mut iter, tokens, options)),
            &Raw(ref x) => ret.push(box Text::new(x.as_slice()) as Box<Renderable>)
        }
        token = iter.next();
    }
    ret
}

// creates an expression, which wraps everything that gets rendered
fn parse_expression<'a> (tokens: &Vec<Token>, options: &'a LiquidOptions) -> Box<Renderable + 'a> {
    match tokens[0] {
        Identifier(ref x) if options.tags.contains_key(&x.to_string()) => {
            options.tags.get(x).unwrap().initialize(x.as_slice(), tokens.tail(), options)
        },
        Identifier(ref x) => parse_output(tokens, options),
        // TODO implement warnings/errors
        ref x => panic!("parse_expression: {} not implemented", x)
    }
}

// creates an output, basically a wrapper around values, variables and filters
fn parse_output<'a> (tokens: &Vec<Token>, options: &'a LiquidOptions) -> Box<Renderable + 'a> {
    let first_item = match tokens[0] {
        Identifier(ref x) => box Output::new(x.as_slice()),
        // TODO implement warnings/errors
        ref x => panic!("parse_output: {} not implemented", x)
    };

    let mut items = vec![];
    let mut iter = tokens.iter();
    let mut token = iter.next();
    while token.is_some() {
        match token.unwrap() {
        }
        token = iter.next();
    }

    box Output::new(first_item as Box<Renderable>, &items) as Box<Renderable>
}

// a tag can be either a single-element tag or a block, which can contain other elements
// and is delimited by a closing tag named {{end + the_name_of_the_tag}}
// tags do not get rendered, but blocks may contain renderable expressions
fn parse_tag<'a> (iter: &mut IterMut<Element>, tokens: &Vec<Token>, options: &'a LiquidOptions) -> Box<Renderable + 'a> {
    match tokens[0] {

        // is a tag
        Identifier(ref x) if options.tags.contains_key(x) => {
            options.tags.get(x).unwrap().initialize(x.as_slice(), tokens.tail(), options)
        },

        // is a block
        Identifier(ref x) if options.blocks.contains_key(x) => {
            let end_tag = Identifier("end".to_string() + x.as_slice());
            let mut children = vec![];
            loop {
                children.push(match iter.next() {
                    Some(&Tag(ref tokens,_)) if tokens[0] == end_tag => break,
                    None => break,
                    Some(t) => t.clone(),
                })
            }
            options.blocks.get(x).unwrap().initialize(x.as_slice(), tokens.tail(), children, options)
        },

        // TODO implement warnings/errors
        ref x => panic!("parse_tag: {} not implemented", x)
    }
}

