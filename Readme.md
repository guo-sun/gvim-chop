## Requirements
Windows, rustup, cargo

## Installation

`> ./build32.bat`

or

`> ./build64.bat`

If you're missing targets, install with:

`> ./rust-lib/add_rustup_targets.bat`

## Usage

```
noremap <C-w>k :call chop#maxscreen()<CR>
noremap <C-w>j :call chop#notepad()<CR>
noremap <C-k>z :call chop#pin()<CR>
noremap <C-k>x :call chop#obscure()<CR>
noremap <C-k>c :call chop#clarify()<CR>
```

By default the opacity commands (obscure, clarify) have a step-size of 5. Counts can be added before the opacity commands, e.g. `2<C-k>c`.
