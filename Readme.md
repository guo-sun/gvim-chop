Modify the gVim window:
- Chop off the title bar
- Set opacity
- Pin window to top
- Make the background transparent
- Position or size window

## Requirements
Windows, rustup, cargo

## Installation

Build the version which matches your gVim install, either

`> ./build32.bat`

or

`> ./build64.bat`

### Plugin

In your .gvimrc:
`call minpac#add('guo-sun/gvim-chop', { 'do' : '!build32.bat'})`

### Troubleshoot

If you're missing targets, install with:

`> ./rust-lib/add_rustup_targets.bat`

## Usage

:e $MYGVIMRC

```
noremap <leader>wk :call chop#maxscreen()<CR>
noremap <leader>wj :call chop#notepad()<CR>
noremap <leader>wt :call chop#add_title_bar()<CR>
noremap <leader>wz :call chop#pin()<CR>
noremap <leader>wb :call chop#bg()<CR>
noremap <leader>wx :<C-U>call chop#obscure()<CR>
noremap <leader>wc :<C-U>call chop#clarify()<CR>
```

Check [chop.vim](autoload/chop.vim) to see available functions in `chop#`.


By default the opacity commands (obscure, clarify) have a step-size of 5. Counts can be added before the opacity commands, e.g. `2<leader>wc`.
