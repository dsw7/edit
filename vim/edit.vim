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

function! s:OpenEditCommandBuffer()
  let s:code_to_edit = s:get_highlighted_block()
  let l:code_to_edit = split(s:code_to_edit, "\n")

  vnew
  setlocal buftype=nofile
  setlocal bufhidden=wipe
  setlocal noswapfile

  let b:is_edit_cmd_buf = v:true

  call setline(1, l:code_to_edit)
  s:print_separator()

  normal! G
endfunction

xnoremap <silent> ed :<C-u>call <SID>OpenEditCommandBuffer()<CR>
