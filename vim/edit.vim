" -----------------------------------------------------------------------------------------------------------
" VimScript for `edit` command integration
" Copyright © David Weber
" See https://github.com/dsw7/edit for more information
" -----------------------------------------------------------------------------------------------------------

function! s:handle_submission(user_input)
  if a:user_input ==# 'quit'
      execute 'q!'
      return
  endif

  call prompt_setprompt(bufnr('%'), '> ')
endfunction

function! s:start_repl_loop(code_to_edit) abort
  vnew Prompt
  setlocal buftype=prompt
  setlocal bufhidden=wipe
  setlocal noswapfile

  call prompt_setprompt(bufnr('%'), '> ')
  call prompt_setcallback(bufnr('%'), 's:handle_submission')

  let l:separator = repeat('─', ((&colorcolumn > 0 ? &colorcolumn : 81) - 1))

  call setline(1, "Type 'quit' to exit")
  call matchaddpos('Comment', [1])
  call setline(2, '')
  call setline(3, split(a:code_to_edit, "\n"))
  call append('$', l:separator)
  call matchaddpos('Comment', [line('$')])

  startinsert
endfunction

function! s:yank_code_to_edit() abort
  let l:old_reg = getreg('x')
  let l:old_regtype = getregtype('x')

  normal! gv"xy

  let l:selection = getreg('x')
  call setreg('x', l:old_reg, l:old_regtype)

  return l:selection
endfunction

function! s:copy_selected_code_to_new_window() abort
  let l:code_to_edit = s:yank_code_to_edit()
  let l:original_filename = bufname('%')

  call s:start_repl_loop(l:code_to_edit)

  let b:code_to_edit = l:code_to_edit
  let b:original_filename = l:original_filename

  normal! Go
  startinsert
endfunction

xnoremap <silent> ed :<C-u>call <SID>copy_selected_code_to_new_window()<CR>
