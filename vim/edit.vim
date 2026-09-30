function! s:print_separator()
  let s:separator = repeat('=', 109)
  call append('$', s:separator)
endfunction

function! s:get_highlighted_block()
  let l:old_reg = getreg('x')
  let l:old_regtype = getregtype('x')

  normal! gv"xy

  let l:selection = getreg('x')
  call setreg('x', l:old_reg, l:old_regtype)

  return l:selection
endfunction
