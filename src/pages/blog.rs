use dioxus::prelude::*;

#[inline_props]
pub fn Header(cx: Scope, spans: Vec<markdown::Span>, size: usize) -> Element {


    cx.render(rsx!(
        {LazyNodes::new(move |f| {
            f.raw_element(
                // tag name
                &format!("h{}", size),

                None,

                // attributes
                &[],

                // listeners
                &[],

                // children
                &[rsx!(
                    div {
                        "foo"
                    }
                )],

                // key
                None,
            )
        })}
    ))
}

pub fn Blog(cx: Scope) -> Element {
    let foo = include_str!("../../posts/2022-05-16 - New Blog.md");

    let foo2 = markdown::tokenize(foo);

    cx.render(rsx!(
        div {
            class: "text-center pt-16 md:pt-32",
            h1 {
                class: "font-bold text-3xl md:text-5xl",
                "Welcome to my blog"
            }
            foo2.iter().map(|b| match b {
                markdown::Block::Header(b, s) => rsx!(Header { spans: b.to_vec(), size: *s }),
                markdown::Block::Paragraph(b) => rsx!(div { "todo:paragraph" }),
                markdown::Block::Blockquote(l) => rsx!(div { "todo:blockquote" }),
                markdown::Block::CodeBlock(t, b) => rsx!(div { "todo:codeblock" }),
                markdown::Block::OrderedList(l, t) => rsx!(div { "todo:orderedlist" }),
                markdown::Block::UnorderedList(l) => rsx!(div { "todo:unorderedlist" }),
                markdown::Block::Raw(s) => rsx!(div { "s" }),
                markdown::Block::Hr => rsx!(div { }),
            })
        }
    ))
}
