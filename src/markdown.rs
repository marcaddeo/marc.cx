//! HTML renderer that takes an iterator of events as input.
//! Based on the default pulldown_cmark html renderer.

use std::collections::HashMap;
use std::io::{self, Write};

use pulldown_cmark::escape::{escape_href, escape_html, StrWrite, WriteWrapper};
use pulldown_cmark::CowStr;
use pulldown_cmark::Event::*;
use pulldown_cmark::{Alignment, CodeBlockKind, Event, LinkType, Tag};

enum TableState {
    Head,
    Body,
}

struct HtmlWriter<'a, I, W> {
    /// Iterator supplying events.
    iter: I,

    /// Writer to write to.
    writer: W,

    /// Whether or not the last write wrote a newline.
    end_newline: bool,

    buffers: Vec<String>,
    table_state: TableState,
    table_alignments: Vec<Alignment>,
    table_cell_index: usize,
    numbers: HashMap<CowStr<'a>, usize>,
}

impl<'a, I, W> HtmlWriter<'a, I, W>
where
    I: Iterator<Item = Event<'a>>,
    W: StrWrite,
{
    fn new(iter: I, writer: W) -> Self {
        Self {
            iter,
            writer,
            end_newline: true,
            buffers: vec![],
            table_state: TableState::Head,
            table_alignments: vec![],
            table_cell_index: 0,
            numbers: HashMap::new(),
        }
    }

    /// Writes a new line.
    fn write_newline(&mut self) -> io::Result<()> {
        self.end_newline = true;
        self.write("\n")
    }

    /// Writes a buffer, and tracks whether or not a newline was written.
    #[inline]
    fn write(&mut self, s: &str) -> io::Result<()> {
        if let Some(current_buffer) = &mut self.buffers.last_mut() {
            current_buffer.push_str(s);
        } else {
            self.writer.write_str(s)?;
        }

        if !s.is_empty() {
            self.end_newline = s.ends_with('\n');
        }
        Ok(())
    }

    #[inline]
    fn write_escape(&mut self, s: &str) -> io::Result<()> {
        let mut escaped: String = String::new();
        escape_html(&mut escaped, &s)?;

        self.write(&escaped)
    }

    #[inline]
    fn write_escape_href(&mut self, s: &str) -> io::Result<()> {
        let mut escaped: String = String::new();
        escape_href(&mut escaped, &s)?;

        self.write(&escaped)
    }

    fn run(mut self) -> io::Result<()> {
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => {
                    self.start_tag(tag)?;
                }
                End(tag) => {
                    self.end_tag(tag)?;
                }
                Text(text) => {
                    self.write_escape(&text)?;
                    self.end_newline = text.ends_with('\n');
                }
                Code(text) => {
                    self.write("<code>")?;
                    self.write_escape(&text)?;
                    self.write("</code>")?;
                }
                Html(html) => {
                    self.write(&html)?;
                }
                SoftBreak => {
                    self.write_newline()?;
                }
                HardBreak => {
                    self.write("<br />\n")?;
                }
                Rule => {
                    if self.end_newline {
                        self.write("<hr />\n")?;
                    } else {
                        self.write("\n<hr />\n")?;
                    }
                }
                FootnoteReference(name) => {
                    let len = self.numbers.len() + 1;
                    self.write("<sup class=\"footnote-reference\"><a href=\"#")?;
                    self.write_escape(&name)?;
                    self.write("\">")?;
                    let number = *self.numbers.entry(name).or_insert(len);
                    self.write(&format!("{}", number))?;
                    self.write("</a></sup>")?;
                }
                TaskListMarker(true) => {
                    self.write("<input disabled=\"\" type=\"checkbox\" checked=\"\"/>\n")?;
                }
                TaskListMarker(false) => {
                    self.write("<input disabled=\"\" type=\"checkbox\"/>\n")?;
                }
            }
        }
        Ok(())
    }

    /// Writes the start of an HTML tag.
    fn start_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
        match tag {
            Tag::Paragraph => {
                if self.end_newline {
                    self.write("<p>")
                } else {
                    self.write("\n<p>")
                }
            }
            Tag::Heading(level, id, classes) => {
                if self.end_newline {
                    self.end_newline = false;
                    self.write("<")?;
                } else {
                    self.write("\n<")?;
                }
                self.write(&format!("{}", level))?;
                if let Some(id) = id {
                    self.write(" id=\"")?;
                    self.write_escape(id)?;
                    self.write("\"")?;
                }
                let mut classes = classes.iter();
                if let Some(class) = classes.next() {
                    self.write(" class=\"")?;
                    self.write_escape(class)?;
                    for class in classes {
                        self.write(" ")?;
                        self.write_escape(class)?;
                    }
                    self.write("\"")?;
                }
                self.write(">")
            }
            Tag::Table(alignments) => {
                self.table_alignments = alignments;
                self.write("<table>")
            }
            Tag::TableHead => {
                self.table_state = TableState::Head;
                self.table_cell_index = 0;
                self.write("<thead><tr>")
            }
            Tag::TableRow => {
                self.table_cell_index = 0;
                self.write("<tr>")
            }
            Tag::TableCell => {
                match self.table_state {
                    TableState::Head => {
                        self.write("<th")?;
                    }
                    TableState::Body => {
                        self.write("<td")?;
                    }
                }
                match self.table_alignments.get(self.table_cell_index) {
                    Some(&Alignment::Left) => self.write(" style=\"text-align: left\">"),
                    Some(&Alignment::Center) => self.write(" style=\"text-align: center\">"),
                    Some(&Alignment::Right) => self.write(" style=\"text-align: right\">"),
                    _ => self.write(">"),
                }
            }
            Tag::BlockQuote => {
                // if self.end_newline {
                //     self.write("<blockquote>\n")
                // } else {
                //     self.write("\n<blockquote>\n")
                // }
                self.buffers.push(String::new());

                Ok(())
            }
            Tag::CodeBlock(_) => {
                if !self.end_newline {
                    self.write_newline()?;
                }

                self.buffers.push(String::new());

                Ok(())

                // match info {
                //     CodeBlockKind::Fenced(info) => {
                //         let lang = info.split(' ').next().unwrap();

                //         if lang.is_empty() {
                //             self.write("<pre><code>")
                //         } else {
                //             self.write("<pre><code class=\"language-")?;
                //             self.write_escape(lang)?;
                //             self.write("\">")
                //         }
                //     }
                //     CodeBlockKind::Indented => {
                //         self.write("<pre><code>")
                //     }
                // }
            }
            Tag::List(Some(1)) => {
                if self.end_newline {
                    self.write("<ol>\n")
                } else {
                    self.write("\n<ol>\n")
                }
            }
            Tag::List(Some(start)) => {
                if self.end_newline {
                    self.write("<ol start=\"")?;
                } else {
                    self.write("\n<ol start=\"")?;
                }
                self.write(&format!("{}", start))?;
                self.write("\">\n")
            }
            Tag::List(None) => {
                if self.end_newline {
                    self.write("<ul>\n")
                } else {
                    self.write("\n<ul>\n")
                }
            }
            Tag::Item => {
                if self.end_newline {
                    self.write("<li>")
                } else {
                    self.write("\n<li>")
                }
            }
            Tag::Emphasis => self.write("<em>"),
            Tag::Strong => self.write("<strong>"),
            Tag::Strikethrough => self.write("<del>"),
            Tag::Link(LinkType::Email, dest, title) => {
                self.write("<a href=\"mailto:")?;
                self.write_escape_href(&dest)?;
                if !title.is_empty() {
                    self.write("\" title=\"")?;
                    self.write_escape(&title)?;
                }
                self.write("\">")
            }
            Tag::Link(_link_type, _dest, _title) => {
                self.buffers.push(String::new());

                Ok(())
                // self.write("<a href=\"")?;
                // self.write_escape_href(&dest)?;
                // if !title.is_empty() {
                //     self.write("\" title=\"")?;
                //     self.write_escape(&title)?;
                // }
                // self.write("\">")
            }
            Tag::Image(_link_type, dest, title) => {
                self.write("<img src=\"")?;
                self.write_escape_href(&dest)?;
                self.write("\" alt=\"")?;
                self.raw_text()?;
                if !title.is_empty() {
                    self.write("\" title=\"")?;
                    self.write_escape(&title)?;
                }
                self.write("\" />")
            }
            Tag::FootnoteDefinition(name) => {
                if self.end_newline {
                    self.write("<div class=\"footnote-definition\" id=\"")?;
                } else {
                    self.write("\n<div class=\"footnote-definition\" id=\"")?;
                }
                self.write_escape(&*name)?;
                self.write("\"><sup class=\"footnote-definition-label\">")?;
                let len = self.numbers.len() + 1;
                let number = *self.numbers.entry(name).or_insert(len);
                self.write(&format!("{}", number))?;
                self.write("</sup>")
            }
        }
    }

    fn end_tag(&mut self, tag: Tag) -> io::Result<()> {
        match tag {
            Tag::Paragraph => {
                self.write("</p>\n")?;
            }
            Tag::Heading(level, _id, _classes) => {
                self.write("</")?;
                self.write(&format!("{}", level))?;
                self.write(">\n")?;
            }
            Tag::Table(_) => {
                self.write("</tbody></table>\n")?;
            }
            Tag::TableHead => {
                self.write("</tr></thead><tbody>\n")?;
                self.table_state = TableState::Body;
            }
            Tag::TableRow => {
                self.write("</tr>\n")?;
            }
            Tag::TableCell => {
                match self.table_state {
                    TableState::Head => {
                        self.write("</th>")?;
                    }
                    TableState::Body => {
                        self.write("</td>")?;
                    }
                }
                self.table_cell_index += 1;
            }
            Tag::BlockQuote => {
                let blockquote = self.buffers.pop().unwrap();
                let mut lines = blockquote.lines();
                let first_line = lines.next();

                let mut alert_type = String::new();
                let mut quote = String::new();

                if let Some(mut line) = first_line {
                    if line.starts_with("<p>") {
                        line = line.strip_prefix("<p>").unwrap();
                        quote += "<p>";
                    }
                    match line {
                        "[!NOTE]" => {
                            alert_type = "note".into();
                        }
                        "[!IMPORTANT]" => {
                            alert_type = "important".into();
                        }
                        "[!WARNING]" => {
                            alert_type = "warning".into();
                        }
                        _ => (),
                    }

                    // If this isn't an alert, we need to preserve the first
                    // line.
                    if alert_type.is_empty() {
                        quote += line;
                    }
                }

                // Join the remaining lines back together.
                quote += &lines.collect::<Vec<_>>().join("\n");

                if self.end_newline {
                    self.write("<noscript><blockquote class=\"alert--")?;
                } else {
                    self.write("\n<noscript><blockquote class=\"alert--")?;
                }
                self.write_escape(&alert_type)?;
                self.write("\">\n")?;
                self.write(&quote)?;
                self.write("</blockquote></noscript>\n")?;

                if self.end_newline {
                    self.write("<alert-block type=\"")?;
                } else {
                    self.write("\n<alert-block type=\"")?;
                }
                self.write_escape(&alert_type)?;
                self.write("\" value=\"")?;
                self.write_escape(&quote)?;
                self.write("\">\n")?;
                self.write("</alert-block>")?;
            }
            Tag::CodeBlock(info) => {
                let mut language = "plaintext";
                match info {
                    CodeBlockKind::Fenced(ref info) => {
                        let lang = info.split(' ').next().unwrap();
                        language = lang;
                    }
                    _ => (),
                };

                let code = self.buffers.pop().unwrap();

                self.write("<noscript><pre><code class=\"language-")?;
                self.write_escape(language)?;
                self.write("\">")?;
                self.write(&code)?;
                self.write("</code></pre></noscript>")?;
                self.write("<code-block language=\"")?;
                self.write_escape(language)?;
                self.write("\" code=\"")?;
                self.write(&code)?;
                self.write("\"></code-block>")?;
            }
            Tag::List(Some(_)) => {
                self.write("</ol>\n")?;
            }
            Tag::List(None) => {
                self.write("</ul>\n")?;
            }
            Tag::Item => {
                self.write("</li>\n")?;
            }
            Tag::Emphasis => {
                self.write("</em>")?;
            }
            Tag::Strong => {
                self.write("</strong>")?;
            }
            Tag::Strikethrough => {
                self.write("</del>")?;
            }
            Tag::Link(_, dest, title) => {
                let inner = self.buffers.pop().unwrap();
                let is_external = !(dest.starts_with("/") || dest.starts_with("https://marc.cx"));

                self.write("<a href=\"")?;
                self.write_escape_href(&dest)?;
                if !title.is_empty() {
                    self.write("\" title=\"")?;
                    self.write_escape(&title)?;
                }
                if is_external {
                    self.write("\" target=\"_blank\" rel=\"noopener\" class=\"external")?;
                }
                self.write("\">")?;
                self.write(&inner)?;
                self.write("</a>")?;
            }
            Tag::Image(_, _, _) => (), // shouldn't happen, handled in start
            Tag::FootnoteDefinition(_) => {
                self.write("</div>\n")?;
            }
        }
        Ok(())
    }

    // run raw text, consuming end tag
    fn raw_text(&mut self) -> io::Result<()> {
        let mut nest = 0;
        while let Some(event) = self.iter.next() {
            match event {
                Start(_) => nest += 1,
                End(_) => {
                    if nest == 0 {
                        break;
                    }
                    nest -= 1;
                }
                Html(text) | Code(text) | Text(text) => {
                    self.write_escape(&text)?;
                    self.end_newline = text.ends_with('\n');
                }
                SoftBreak | HardBreak | Rule => {
                    self.write(" ")?;
                }
                FootnoteReference(name) => {
                    let len = self.numbers.len() + 1;
                    let number = *self.numbers.entry(name).or_insert(len);
                    self.write(&format!("{}", number))?;
                }
                TaskListMarker(true) => self.write("[x]")?,
                TaskListMarker(false) => self.write("[ ]")?,
            }
        }
        Ok(())
    }
}

/// Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and
/// push it to a `String`.
///
/// # Examples
///
/// ```
/// use pulldown_cmark::{html, Parser};
///
/// let markdown_str = r#"
/// hello
/// =====
///
/// * alpha
/// * beta
/// "#;
/// let parser = Parser::new(markdown_str);
///
/// let mut html_buf = String::new();
/// html::push_html(&mut html_buf, parser);
///
/// assert_eq!(html_buf, r#"<h1>hello</h1>
/// <ul>
/// <li>alpha</li>
/// <li>beta</li>
/// </ul>
/// "#);
/// ```
pub fn push_html<'a, I>(s: &mut String, iter: I)
where
    I: Iterator<Item = Event<'a>>,
{
    HtmlWriter::new(iter, s).run().unwrap();
}

/// Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and
/// write it out to a writable stream.
///
/// **Note**: using this function with an unbuffered writer like a file or socket
/// will result in poor performance. Wrap these in a
/// [`BufWriter`](https://doc.rust-lang.org/std/io/struct.BufWriter.html) to
/// prevent unnecessary slowdowns.
///
/// # Examples
///
/// ```
/// use pulldown_cmark::{html, Parser};
/// use std::io::Cursor;
///
/// let markdown_str = r#"
/// hello
/// =====
///
/// * alpha
/// * beta
/// "#;
/// let mut bytes = Vec::new();
/// let parser = Parser::new(markdown_str);
///
/// html::write_html(Cursor::new(&mut bytes), parser);
///
/// assert_eq!(&String::from_utf8_lossy(&bytes)[..], r#"<h1>hello</h1>
/// <ul>
/// <li>alpha</li>
/// <li>beta</li>
/// </ul>
/// "#);
/// ```
pub fn write_html<'a, I, W>(writer: W, iter: I) -> io::Result<()>
where
    I: Iterator<Item = Event<'a>>,
    W: Write,
{
    HtmlWriter::new(iter, WriteWrapper(writer)).run()
}
