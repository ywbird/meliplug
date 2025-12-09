---
title: Test Meliplug
draft: true
toc: true
math: true
jsxgraph: true
---

{{< callout "note" >}}
callout note
{{< /callout >}}
{{< callout "tip" >}}
callout test
{{< /callout >}}
{{< callout "warning" >}}
callout warning
{{< /callout >}}
{{< callout "important" >}}
callout important
{{< /callout >}}
{{< callout "caution" >}}
callout caution
{{< /callout >}}

{{< jsxgraph "jxgbox" 500 200 >}}
<script type="text/javascript">
 var board = JXG.JSXGraph.initBoard('jxgbox', {boundingbox: [-5, 2, 5, -2]});
 var p = board.create('point',[-3,1]);
</script>

---

{{< h "banana" "primary1" >}}
{{< h "banana" "primary2" >}}
{{< h "banana" "primary3" >}}
{{< h "banana" "primary4" >}}
{{< h "banana" "primary5" >}}
{{< h "banana" "primary6" >}}
{{< h "banana" "primary7" >}}
{{< h "banana" "primary8" >}}

## Math

$math$

$$
bigger\ math \sum
$$

## Markdown Cheat Sheet

Thanks for visiting [The Markdown Guide](https://www.markdownguide.org)!

This Markdown cheat sheet provides a quick overview of all the Markdown syntax elements. It can’t cover every edge case, so if you need more information about any of these elements, refer to the reference guides for [basic syntax](https://www.markdownguide.org/basic-syntax/) and [extended syntax](https://www.markdownguide.org/extended-syntax/).

### Basic Syntax

These are the elements outlined in John Gruber’s original design document. All Markdown applications support these elements.

#### Heading

## H1
### H2
#### H3

#### Bold

**bold text**

#### Italic

*italicized text*

#### Blockquote

> blockquote

#### Ordered List

1. First item
2. Second item
3. Third item

#### Unordered List

- First item
- Second item
- Third item

#### Code

`code`

#### Horizontal Rule

---

#### Link

[Markdown Guide](https://www.markdownguide.org)

#### Image

![alt text](https://www.markdownguide.org/assets/images/tux.png)

### Extended Syntax

These elements extend the basic syntax by adding additional features. Not all Markdown applications support these elements.

#### Table

| Syntax | Description |
| ----------- | ----------- |
| Header | Title |
| Paragraph | Text |

#### Fenced Code Block

```json
{
  "firstName": "John",
  "lastName": "Smith",
  "age": 25
}
```

#### Footnote

Here's a sentence with a footnote. [^1]

[^1]: This is the footnote.

#### Heading ID

#### My Great Heading {#custom-id}

#### Definition List

term
: definition

#### Strikethrough

~~The world is flat.~~

#### Task List

- [x] Write the press release
- [ ] Update the website
- [ ] Contact the media

#### Emoji

That is so funny! :joy:

(See also [Copying and Pasting Emoji](https://www.markdownguide.org/extended-syntax/#copying-and-pasting-emoji))

#### Highlight

I need to highlight these ==very important words==.

#### Subscript

H<sub>2</sub>O

#### Superscript

X<sup>2</sup>
