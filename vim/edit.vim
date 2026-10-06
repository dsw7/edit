" -----------------------------------------------------------------------------------------------------------
" VimScript for `edit` command integration
" Copyright © David Weber
" See https://github.com/dsw7/edit for more information
" -----------------------------------------------------------------------------------------------------------

let s:input_split = repeat('─', winwidth(0))

function! s:print_exception() abort
  echohl ErrorMsg
  echo v:exception
  echohl None
endfunction

" -----------------------------------------------------------------------------------------------------------
" Buffer management

function! s:open_prompt_window(code_to_edit) abort
  vnew Prompt
  setlocal buftype=nofile
  setlocal bufhidden=wipe
  setlocal noswapfile

  normal! ggdG

  call setline(1, ':W to submit prompt')
  call matchaddpos('Comment', [1])

  call setline(2, '')
  call append('$', a:code_to_edit)
  call append('$', s:input_split)

  let b:prompt_window_is_open = v:true
endfunction

function! s:close_prompt_window() abort
  if exists('b:prompt_window_is_open')
    quit
  endif
endfunction

" -----------------------------------------------------------------------------------------------------------
" Transfer selected code to new window on the right

function! s:yank_code_to_edit() abort
  let l:old_reg = getreg('x')
  let l:old_regtype = getregtype('x')

  normal! gv"xy

  let l:selection = getreg('x')
  call setreg('x', l:old_reg, l:old_regtype)

  return split(l:selection, "\n")
endfunction

function! s:copy_selected_code_to_new_window() abort
  let l:code_to_edit = s:yank_code_to_edit()
  let l:original_filename = bufname('%')

  call s:open_prompt_window(l:code_to_edit)

  let b:code_to_edit = l:code_to_edit
  let b:original_filename = l:original_filename

  normal! Go
  startinsert
endfunction

xnoremap <silent> ed :<C-u>call <SID>copy_selected_code_to_new_window()<CR>

" -----------------------------------------------------------------------------------------------------------
" Operate on code in new window

function! s:get_instructions_after_delimiter() abort
  let l:start_line = search('^' . s:input_split . '$', 'n')

  if l:start_line > 0
    return getline(l:start_line + 1, line('$'))
  endif

  throw 'delimiter not found'
endfunction

function! s:open_completion_window(output) abort
  new Completion
  setlocal buftype=nofile
  setlocal bufhidden=wipe
  setlocal noswapfile

  let l:lines = split(a:output, "\n")
  call setline(1, l:lines)
endfunction

function! s:consume_payload() abort
  let l:code_to_edit = join(b:code_to_edit, "\n")
  let l:instructions = join(s:get_instructions_after_delimiter(), "\n")

  let l:command = []
  call add(l:command, '/tmp/foo.py')
  call add(l:command, shellescape(l:code_to_edit))
  call add(l:command, '--filename=' . shellescape(b:original_filename))
  call add(l:command, '--instructions=' . shellescape(l:instructions))
  let l:output = system(join(l:command, ' '))

  call s:open_completion_window(l:output)
  normal! G
endfunction

function! s:run_edit_command() abort
  if exists('b:prompt_window_is_open')
    try
      call s:consume_payload()
    catch /.*/
      call s:print_exception()
      call input('Press ENTER to close this window...')
      call s:close_prompt_window()
    endtry
  else
    echom 'command must follow `ed` invocation'
  endif
endfunction

command! W call <SID>run_edit_command()
