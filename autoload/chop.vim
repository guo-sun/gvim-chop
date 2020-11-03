let s:path = fnamemodify(expand('<sfile>'), ':p:h:h')
let s:pinned = 0
let s:opacity = 100

function! s:CallLocalLib(relativeDllPath, fnName, arg) abort
        " Vim secretly switches the current directory to
        " the Vim exe directory, so we need the absolute path here
    let l:absPath = s:path.a:relativeDllPath

    if !filereadable(l:absPath)
        echoerr "Couldn't find dll at: ".l:absPath
        return
    endif

    return libcallnr(l:absPath, a:fnName, a:arg)
endfunction

function! s:CallRustFn(fnName, arg)
    let l:dllPath = "/rust-lib/gvim_chop.dll"

    return s:CallLocalLib(l:dllPath, a:fnName, a:arg)
endfunction

function! chop#fullscreen()
    call s:CallRustFn("fullscreen", 0)
endfunction

function! chop#opacity(alpha)
        " alpha: Number from 0 to 100
    let s:opacity = a:alpha
    call s:CallRustFn("opacity", float2nr((a:alpha / 100.0) * 255))
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
    call chop#opacity(max([s:opacity - 5, 0]))
endfunction

function! chop#obscure()
    call chop#opacity(min([s:opacity + 5, 100]))
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
