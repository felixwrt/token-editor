# Token Editor

This repo contains an old experiment that I did in 2019, exploring an idea to make code editing more ergonomic. I updated the code and documented the project in 2024. I don't plan to do any maintenance work on it, but I'm always happy to hear your thoughts regarding this project and related ideas. Write me an [email](mailto:me@felixwrt.dev)!

[**Live demo**](TODO) (should be used on a Desktop, mobile likely doesn't work right)

See below for more info.

## Motivation

Maintaining formatted code is tedious and a lot of editing operations I do are changes to whitespace characters to make the code look nice. It's possible to automate some of this effort by frequently running a code formatter like `rustfmt` on the file being edited (e.g. after editing a line of source code). Code editors usually have shortcuts for formatting the current text file and there's often also an option to format on save. Such a workflow is quite convenient can automate the maintenance of whitespace characters.

The whitespace characters are still in the source code though, so when navigating the cursor through a source code file, one needs to perform additional key presses to move over characters that are purely there for aesthetics and don't have any semantic meaning.

The idea of this experiment is an editor where optional whitespace characters don't need to be typed but are still displayed. 

I think it's best shown with an example:

In the video below, I'm doing a simple text edit, typing the characters ` = a - b` and moving the cursor around:

doc/example_manual_with_ws.mp4

Not that I'm typing the three space characters myself. When moving the cursor, I need to do extra key presses to move over the whitespace characters.

An alternative that requires less typing is to omit space characters that aren't necessary. Typing the same code, but without spaces (`=a-b`):

doc/example_manual_no_ws.mp4

In this second video, I typed less characters (4 instead of 7) and because there are fewer characters, moving the cursor back to the start of the code requires less typing. The downside though is that the code looks ugly and would have bad readability in any non-trivial case.

This experiment tries to combine the readability of having optional whitespace characters with the editing experience of not having them:

doc/example_virtual_whitespace.mp4

In the video above, I'm typing the same characters as in the previous video (`=a-b`). The editor automatically displays spaces I haven't typed to make the code look nice. These spaces also can't be selected, so moving the cursor behaves like in the previous video where I didn't have any spaces.

## How it works

This experiment uses [`prettyplease`](https://github.com/dtolnay/prettyplease), a code formatter for Rust code, to detect where in the typed source code whitespace should be displayed. Whitespace that is inserted by the formatter becomes so-called "virtual whitespace". Virtual whitespace is displayed, but cannot be selected. 

The editor displays a custom cursor that shows virtual whitespace. A regular cursor sits between two characters in the source code and its the same in this experiment. The difference is that if there is virtual whitespace between two characters, the visual space between them becomes wider. In this experiment, the cursor then changes from a slim bar (`|`) into a highlighted area that looks like a selection in a text editor.

### Build

This project uses [`trunk`](https://trunkrs.dev/) to build and serve the web app locally. After installing `trunk`, the web app can be built and served using:

```
trunk serve
```

## Status

The implementation seems to work fine, but I haven't used it beyond simple examples. For it to be usable for regular programming, one would need to implement additional features like selections, mouse support and probably a ton of other features you're used to.

## Discussion

Would I want to use something like this? I don't know. I could imagine that editing like this could feel very fluent and nice. On the other hand, it's not easy to see where virtual whitespace is used compared to typed whitespace characters (which are still needed in some places, e.g. between keywords like `pub fn`). Updating the virtual whitespace is also only possible if the code formatter is able to format the code. When editing, the code is often in a state that doesn't parse correctly, and if that's the case, the virtual whitespace also cannot be updated. This leads to unformatted output until the code has been reworked to parse again. When that's the case, the code suddenly changes shape and looks differently. This is something I'd need to get used to. A code formatter that can work with code containing errors might be a big improvement.

What do you think about this?