function! CallLocalLib(relativeDllPath, fnName, arg) abort
        " Vim secretly switches the current directory to
        " the Vim exe directory, so we need the absolute path here
    let l:absPath = fnamemodify(expand('%'), ':p:h:h').a:relativeDllPath

    if !filereadable(l:absPath)
        echoerr "Couldn't find dll at: ".l:absPath
        return
    endif

    return libcallnr(l:absPath, a:fnName, a:arg)
endfunction

function! CallRustFn(fnName, arg)
    let l:dllPath = "/rust-module/target/debug/gvim_chop.dll"

    return CallLocalLib(l:dllPath, a:fnName, a:arg)
endfunction

function! Test()
    echo CallRustFn("do_thing", 0)
endfunction
