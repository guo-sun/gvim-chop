let s:absDllPath = fnamemodify(expand('<sfile>'), ':p:h:h')."/rust-lib/gvim_chop.dll"

let s:pinned = 0
let s:opacity = 100
let s:bgDropped = 0
let s:bgColor = "#000000"

function! s:CallRustFn(fnName, arg)
    if !filereadable(s:absDllPath)
        echoerr "Couldn't find dll at: ".s:absDllPath
        return
    endif

    return libcallnr(s:absDllPath, a:fnName, a:arg)
endfunction

function! chop#fullscreen()
    call s:CallRustFn("fullscreen", 0)
endfunction

function! chop#opacity(opacity)
        " opacity: Number from 0 to 100
    let l:opacity = min([max([a:opacity, 0]), 100])
    let s:opacity = l:opacity
    call s:CallRustFn("opacity", float2nr((l:opacity / 100.0) * 255))
    echom "Opacity: ".s:opacity
endfunction

function! chop#add_title_bar()
    call s:CallRustFn("add_title_bar", 0)
endfunction

function! chop#remove_title_bar()
    call s:CallRustFn("remove_title_bar", 0)
endfunction

function! chop#position_window(pos_string)
        " pos_string: x-y-width-height in percentage (Number 0 to 100)
    call s:CallRustFn("position_window", a:pos_string)
endfunction

function! chop#pin_topmost()
    call s:CallRustFn("pin_window", 1)
endfunction

function! chop#notepad()
    call chop#remove_title_bar()
    call chop#position_window("60-40-40-60")
endfunction

function! chop#maxscreen()
    call chop#remove_title_bar()
    call chop#fullscreen()
endfunction

function! chop#clarify()
    call chop#opacity(s:opacity - (v:count1 * 5))
endfunction

function! chop#obscure()
    call chop#opacity(s:opacity + (v:count1 * 5))
endfunction

function! chop#pin()
    if s:pinned
        let s:pinned = 0
        call s:CallRustFn("pin_window", 0)
        echom "Unpinned"
    else
        let s:pinned = 1
        call s:CallRustFn("pin_window", 1)
        echom "Pinned"
    endif
endfunction

function! s:GetHiGuibg()
    return matchstr(execute('hi Normal'), 'guibg=\zs\S*')
endfunction

function! s:DropBg()
    let s:bgColor = s:GetHiGuibg()
    let s:bgDropped = 1

    echom "Saved background as ".s:bgColor

    if &bg == "light"
        hi Normal guibg=#FFFFFF
        call s:CallRustFn("transparent_white", 0)
    else
        hi Normal guibg=#000000
        call s:CallRustFn("transparent_black", 0)
    endif
endfunction

function! s:RevertBg()
    let s:bgDropped = 0
    echom "Reverting background as ".s:bgColor

    call s:CallRustFn("transparent_none", 0)
    execute('hi Normal guibg='.s:bgColor)
endfunction

function! chop#bg()
    if s:bgDropped
        call s:RevertBg()
    else
        call s:DropBg()
    endif
endfunction
