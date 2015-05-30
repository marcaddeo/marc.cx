__START_META__
title: Hello World
published: 05/26/2015
excerpt: In which I make the first blog article on my new site. A short introduction to my blog, and the technologies behind it.
__END_META__
# Hello World!

I've decided to start a blog. It's an idea that I play with every so often, and
usually don't do much with. I've started blogs in the past, but never really post
anything worth while, if at all. Hopefully this time it's different.

This time I'm going to build my own blog software as I go. It's going to be using
the [Elixir Programming Language][] and the [Phoenix Framework][]. At work I've
built many blogs using Wordpress and it can be very powerful and some-what
flexible. This blog will not be powerful, and will not be flexible. At least not
anytime soon.

Currently it's very simple, and not even very efficient (but still very fast,
thanks to Elixir!). The homepage lists out articles, and articles are written
in [Markdown][]. I much prefer writing in Markdown, as opposed to using
Wordpress' WYSIWYG editor or something similar. Instead of fighting with a
WYSIWYG editor, I can simply fire up `vim` and start writing! The source code
for the blog will also always be [publicly available on Github][repo]!

In the future I'd like to enhance my blog software, maybe even add a back end.
The trick is going to be figuring out how to allow content to be edited as a file
and within the back end, and ensuring that all edits are kept in sync. I'd love
to be able to have the flexibility of a back end, and the simplicity of Markdown
all in one. While still allowing plugins, shortcodes, back end administration,
and file based content editing.

But for now, this blog will remain simple. I'll try to write new articles every
so often. I'd like to do at least one a month, but we'll see I suppose.

## Markdown Rendering

As I said, I'm writing these articles using Markdown. But one thing I wanted was
to have my code blocks "bleed out" of the narrow container that he rest of the
website resides in. I think it's a pretty cool effect, so I needed to figure out
how to do it.

The solution was quickly found when I was checking out what Markdown libraries
existed for Elixir. Most of them looked pretty solid, but [Dave Thomas'][]
library [Earmark][] stood out as I could pass it a custom HTML Renderer. Sweet!

So I started coding up the blog, and made my minor tweaks to the default HTML
Renderer that ships with the Earmark library. All in all, it was pretty simple.
The resulting change just made it so code blocks were rendered as such:

```html
</div>
<div class="container-fluid">
    <pre>
        <code class="elixir">
        ...
        </code>
    </pre>
</div>
<div class="container">
```

That's it. It's a little hacky, but it gets the job done. I just close the current
container that the rest of the article is in, open up a fluid container and throw
the `pre` and `code` blocks in there. And with a little styling, we now have
"full width" code blocks!

I'm then using [highlight.js][] to do the syntax highlighting in my favorite theme,
[Solarized Dark][].

## Markdown & Metadata

The next challenge was being able to specify "metadata" with my articles. I
needed to be able to specify the title, date of publication, and provide an
excerpt for the homepage.

This was actually pretty simple using recursion, lists, and pattern matching in
Elixir. Here's the snippet of code responsible for parsing the article metadata
currently:

```elixir
defp parse_metadata(["__START_META__" | tail], metadata) do
  parse_metadata(tail, metadata)
end
defp parse_metadata(["__END_META__" | content], metadata) do
  {metadata, content}
end
defp parse_metadata([head | tail], metadata) do
  [key, value] = String.split(head, ":", parts: 2, trim: true)
  parse_metadata(tail, Dict.put(metadata, String.to_atom(key), String.strip(value)))
end
defp parse_metadata(content) when is_list(content) do
  parse_metadata(content, %{})
end
```

It's fairly simple, the first line of the file must be `__START_META__` followed
by one `key: value` pair of metadata per line. One the line with `__END_META__`
has been reached, parsing is complete and the metadata and article content are
returned in a tuple.

## Crap

I'm new to Elixir, and to Phoenix. I come from an object oriented background,
not functional. I primarily make websites in PHP using Symfony components and
Wordpress, not Ruby and Rails. So my code is probably going to be crap for a
while as I get used to the new methodologies of a new language and web framework.
But, this is a learning experience with the end-goal of knowing more than I
previously knew about Elixir and Phoenix.

So far, I'm really enjoying Elixir and Phoenix. Hopefully the community can
continue to grow for these technologies, and they can thrive. Hopefully I can
quickly get "good" with Elixir and Phoenix and make some interesting things.


[Elixir Programming Language]: http://elixir-lang.org/
[Phoenix Framework]: http://www.phoenixframework.org/
[Markdown]: http://en.wikipedia.org/wiki/Markdown
[repo]: https://github.com/marcaddeo/marc.cx
[Dave Thomas']: http://pragdave.me/
[Earmark]: https://github.com/pragdave/earmark
[highlight.js]: https://highlightjs.org/
[Solarized Dark]: http://ethanschoonover.com/solarized
