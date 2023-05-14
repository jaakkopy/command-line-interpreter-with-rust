# About

A basic command line interpreter for Linux written in Rust. 

Currently supports:
- pipes with '|'
- parallel commands with '&'
- scrolling the input history up and down with the arrow keys
- stdin and stdout redirections with '<' and '>'
- erasing the previous character with backspace
- changing the current working directory with `cd`
- autocomplete for the last part of the input with tab, '\t'