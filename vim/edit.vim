let s:separator = repeat('=', 109)

function! s:get_highlighted_block()
  let l:old_reg = getreg('x')
  let l:old_regtype = getregtype('x')

  normal! gv"xy

  let l:selection = getreg('x')
  call setreg('x', l:old_reg, l:old_regtype)

  return l:selection
endfunction

function! s:open_edit_command_buf()
  let s:code_to_edit = s:get_highlighted_block()
  let l:code_to_edit = split(s:code_to_edit, "\n")

  vnew
  setlocal buftype=nofile
  setlocal bufhidden=wipe
  setlocal noswapfile

  let b:is_edit_cmd_buf = v:true

  call setline(1, l:code_to_edit)
  call append('$', s:separator)

  normal! G
endfunction

xnoremap <silent> ed :<C-u>call <SID>open_edit_command_buf()<CR>
