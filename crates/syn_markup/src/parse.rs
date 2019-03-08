use crate::types::*;
use proc_macro2::{LineColumn, Span, TokenStream, TokenTree};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream, Result},
    spanned::Spanned,
    token::Brace,
    Error, Expr, Ident, Token,
};

impl Parse for Markup {
    fn parse(input: ParseStream) -> Result<Self> {
        println!("***************** {}", input.cursor().span().end().column);
        let mut nodes = Vec::new();
        while !input.is_empty() {
            nodes.push(input.parse::<Node>()?);
        }
        Ok(Self { nodes })
    }
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![<]) {
            let lt_token: Token![<] = input.parse()?;
            let name = input.call(Ident::parse_any)?;
            println!("????????????????????sdfg {}", name.span().end().column);
            let attributes = input.call(parse_attributes)?;
            let slash_token: Option<Token![/]> = input.parse()?;
            let gt_token: Token![>] = input.parse()?;
            match slash_token {
                Some(slash_token) => Ok(Node::VoidElement(VoidElement {
                    lt_token,
                    name,
                    attributes,
                    slash_token,
                    gt_token,
                })),
                None => {
                    let mut children = Vec::new();
                    let mut from_span = input.cursor().span();
                    loop {
                        if input.is_empty() {
                            break;
                        }
                        if input.peek(Token![<]) && input.peek2(Token![/]) {
                            break;
                        }

                        let node = input.parse::<Node>()?;
                        // let whitespace = match &node {
                        //     Node::OpenElement(el) => {
                        //         let to_span = el.open_tag.lt_token.span();
                        //         get_whitespace_node(from_span, to_span)
                        //     }
                        //     Node::VoidElement(el) => {
                        //         let to_span = el.lt_token.span();
                        //         get_whitespace_node(from_span, to_span)
                        //     }
                        //     _ => None,
                        // };
                        // if let Some(whitespace) {
                        //     children.push(whitespace);
                        // }
                        // from_span = match &node {
                        //     Node::OpenElement(el) => el.close_tag.gt_token.span(),
                        //     Node::VoidElement(el) => el.gt_token.span(),
                        //     _ => from_span,
                        // };
                        children.push(node);
                    }
                    let close_tag = input.parse::<CloseTag>()?;
                    if close_tag.name != name {
                        let span = close_tag.name.span();
                        let msg = format!(
                            "Expected closing tag with name '{}' but got '{}' instead",
                            name, close_tag.name
                        );
                        Err(Error::new(span, msg))
                    } else {
                        // let to_span = close_tag.lt_token.span();
                        // if let Some(whitespace) = get_whitespace_node(from_span, to_span) {
                        //     children.push(whitespace);
                        // }

                        Ok(Node::OpenElement(OpenElement {
                            open_tag: OpenTag {
                                lt_token,
                                name,
                                attributes,
                                gt_token,
                            },
                            children,
                            close_tag,
                        }))
                    }
                }
            }
        } else if input.peek(Brace) {
            input.parse::<Braced>().map(Node::Braced)
        } else {
            input.parse::<Text>().map(Node::Text)
        }
    }
}

fn get_whitespace_node(from: &LineColumn, to: &LineColumn) -> Option<Node> {
    let whitespace = get_whitespace(from, to);
    if !whitespace.is_empty() {
        Some(Node::Text(Text { text: whitespace }))
    } else {
        None
    }
}

fn get_whitespace(from: &LineColumn, to: &LineColumn) -> String {
    if from.line < to.line {
        // TODO: col diff?
        "\n".repeat(to.line - from.line)
    } else if from.column < to.column {
        " ".repeat(to.column - from.column)
    } else {
        String::new()
    }
}

impl Parse for CloseTag {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            lt_token: input.parse()?,
            slash_token: input.parse()?,
            name: input.call(Ident::parse_any)?,
            gt_token: input.parse()?,
        })
    }
}

impl Parse for Text {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut text = String::new();

        let mut last_span: Option<Span> = None;

        loop {
            if input.is_empty() {
                break;
            }

            if input.peek(Token![<]) || input.peek(Brace) {
                if let Some(from) = last_span {
                    let to = input.fork().parse::<TokenTree>()?.span();
                    text += &get_whitespace(&from.end(), &to.start());
                    last_span = Some(to);
                }
                break;
            } else {
                let tt: TokenTree = input.parse()?;
                let to = tt.span();
                if let Some(from) = last_span {
                    println!(
                        "FUCK {} {} {} {} {}",
                        from.start().column,
                        from.end().column,
                        to.start().column,
                        to.end().column,
                        tt.to_string(),
                    );
                    text += &get_whitespace(&from.end(), &to.start());
                }
                last_span = Some(to);
                text += &tt.to_string();
            }
        }
        println!("DONE {}", text);

        Ok(Text { text })
    }
}

impl Parse for Braced {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            block: input.parse()?,
        })
    }
}

fn parse_attributes(input: ParseStream) -> Result<Vec<Attribute>> {
    let mut attrs = Vec::new();

    // Do we see an identifier such as `id`? If so proceed
    while input.peek(Ident) || input.peek(Token![type]) {
        let key = input.call(Ident::parse_any)?;
        let eq_token = input.parse::<Token![=]>()?;

        // Continue parsing tokens until we see the next attribute or a closing > tag
        let mut value_tokens = TokenStream::new();

        loop {
            let tt: TokenTree = input.parse()?;
            value_tokens.extend(Some(tt));

            let has_attrib_key = input.peek(Ident) || input.peek(Token![type]);
            let peek_start_of_next_attr = has_attrib_key && input.peek2(Token![=]);

            let peek_end_of_tag = input.peek(Token![>]);

            let peek_self_closing = input.peek(Token![/]);

            if peek_end_of_tag || peek_start_of_next_attr || peek_self_closing {
                break;
            }
        }

        let value: Expr = syn::parse2(value_tokens)?;

        attrs.push(Attribute {
            key,
            eq_token,
            value,
        });
    }

    Ok(attrs)
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            key: input.call(Ident::parse_any)?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}
