function! s:CallLocalLib(relativeDllPath, fnName, arg) abort
        " Vim secretly switches the current directory to
        " the Vim exe directory, so we need the absolute path here
    let l:absPath = fnamemodify(expand('%'), ':p:h:h').a:relativeDllPath

    if !filereadable(l:absPath)
        echoerr "Couldn't find dll at: ".l:absPath
        return
    endif

    return libcallnr(l:absPath, a:fnName, a:arg)
endfunction

function! s:CallRustFn(fnName, arg)
    let l:dllPath = "/rust-module/gvim_chop.dll"

    return s:CallLocalLib(l:dllPath, a:fnName, a:arg)
endfunction

function! GvimChop#fullscreen()
    call s:CallRustFn("fullscreen", 0)
endfunction

function! GvimChop#opacity(alpha)
        " alpha: Number from 0 to 100
    call s:CallRustFn("opacity", float2nr((a:alpha / 100.0) * 255))
endfunction

function! GvimChop#remove_title_bar()
    call s:CallRustFn("remove_title_bar", 0)
endfunction

function! GvimChop#position_window(pos_string)
        " pos_string: x-y-width-height in percentage (Number 0 to 100)
    call s:CallRustFn("position_window", a:pos_string)
endfunction
