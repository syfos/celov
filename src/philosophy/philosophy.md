# Softwrap

Break rope line string for the renderer at genuine point such as:

1. Mostly at grapheme boundaries.
2. In general common chars where it feels right to such as commas

# Hardwrap 

This is more about file based prebuilt hard wrapping display to map Rope's Line Break defaults.

1. Unicodes that do linebreak (Listed below in Heading 3)

# Cursor movement philosophy

Cursor movement must be :

1. Grapheme range and width aware.
2. Must be word start/end aware.

# Rope and Render 1:1 on line break.

By default Ropey crate supports the following line break characters :

```txt
U + 000A -- LF (Line Feed) -- \n -- Move to next line.
U + 000B -- VT (Vertical Tab) -- \v --Move vertically down.
U + 000C -- Form Feed -- \f -- Advance to the next page/form
U + 000D -- Cariage Return -- \r -- Return to the beginning of the line
U + 0085 -- Next Line NEL -- Go to next line
U + 2028 -- Line Seperator -- Explicit Unicode line seperator 
U + 2029 -- Paragraph Seperator PS -- Explict Unicode Paragraoh Seperator.

CRLF -> Carriage return + Line Feed.
VT is the counter part of HT(Horizontal Tabbing).

Form Feed -- Switch to next page but since in text editor there is no page concept the way similar to typewriters hence ropey instead take it as a line break.
```

> The viewport renderer must be 1:1 with rope recognized newline sequences.

> LF must be the top priority as whole modern ecosystem depends on it. Hence so Enter must give insert "\n".

> Ropey crate treats all the above listed unicodes as real line breaks. If you stacks them all in a single line through python cmd :

```sh
python3 -c 'w=["apple","river","mountain","window","garden","computer","language"]; b=["\u000A","\u000B","\u000C","\u000D","\u0085","\u2028","\u2029"]; open("txt.txt","w",encoding="utf-8",newline="").write("".join(x+"".join(b[j] for j in range(7) if j!=i) for i,x in enumerate(w)))'
```

```sh
python3 -c 'w=["apple","river","mountain","window","garden","computer","language"]; b=["\u000A","\u000B","\u000C","\u000D","\u0085","\u2028","\u2029"]; open("txt.txt","w",encoding="utf-8",newline="").write("".join(x+b[i]+b[(i+1)%7] for i,x in enumerate(w)))'
```

and run the following :

```rust
use anyhow::Context;
use ropey::Rope;
use std::path::{Path, PathBuf};
use std::{fs, io::BufReader};

fn string_to_path(path_string: &str) -> anyhow::Result<PathBuf> {
  let expanded = shellexpand::full(path_string)
    .with_context(|| format!("failed to expand path: `{path_string}`"))?;

  let canonical = Path::new(expanded.as_ref())
      .canonicalize()
      .with_context(|| format!("The expanded path do not exists or is inaccessible : `{expanded}` probably the `{path_string}` is wrong."))?;
  Ok(canonical)
}

fn main() -> anyhow::Result<()> {
  let mut rope = Rope::from_reader(BufReader::new(fs::File::open(string_to_path(
    "~/impl/rust/fgit/src/bin/txt.txt",
  )?)?))?;
  println!("chars: {}", rope.len_chars());
  println!("lines: {}", rope.len_lines());

  for i in 0..rope.len_lines() {
    println!("line {i}: {:?}", rope.line(i));
  }
  Ok(())
}
```

> You will clearly see that each line break unicode is take seriously.

Hence, iterate for each of theme and create the next line for each of their occurences.

# Philosophy of Scroll

## Why use VecDeque

Scoll is just continous pop of line towards the opposite direction along continious push of lines in the moving direction.

E.g,

Imagine a buffer which contains only unwrapped lines such that each row is a line.

1. There are 10 unwrapped lines on 10 rows(max) of viewport, 
2. Move the cusror down once (j × 1)
3. Pop the very first line 
4. Push the 11th lith 

Here `VecDeque` allows me to efficiently:
1. Pop the front and back side.
2. Push to the front and back side.

## How we scroll fast with `<C-f>` or `<C-b>`

Same as pervious case but just at scale for e.g:

Imagine your scroll down/up is set to 5 lines.

Now say on scroll Down :

1. Pop 5 Lines from front.
2. Push 5 Lines from back.

## How command line based teleporation will work?

Well doing this is simple because you just have to draw the cursor at cursor col 0 of that line.

But the real challange arrives when you have to draw the whole viewport around it as this thing needs smart logic.

## Scrolloff=8 (configurable)

The scrolloff is a limit band that neovim uses when the buffer is bigger than the viewport.

Scrolloff tells the program that keep the set limit (default is 8) say scrolloff=8 sets the limit that there must be at minimun of --> 8 rows between the scrolloffset(first row) and the `cursor line`.

While the implementation of scrolloff is only limited to the condition where :

1. Viewport is smaller than the length of whole buffer

**So I have to implement a row margine on both ends of viewport**

The extended viewport will look like this:

```txt
[Extended_Front] [Viewport Main] [Extended_Back]
    [10 rows]       [10 rows]       [10 row]
```

Where both the `Extended_Front` and `Extended_Back` must be and `Viewport` are equal to each other in terms of `net row`.

Now using some logic I can regulate when shall be the scrolloff must be respected and when shall not.


