" -----------------------------------------------------------------------------------------------------------
" VimScript for `edit` command integration
" Copyright © David Weber
" See https://github.com/dsw7/edit for more information
" -----------------------------------------------------------------------------------------------------------

let s:separator = repeat('─', ((&colorcolumn > 0 ? &colorcolumn : 81) - 1))

function! s:print_exception() abort
  echohl ErrorMsg
  echo v:exception
  echohl None
endfunction

function! s:open_prompt_window(code_to_edit) abort
  vnew Prompt
  setlocal buftype=nofile
  setlocal bufhidden=wipe
  setlocal noswapfile

  call setline(1, ':W to submit prompt')
  call matchaddpos('Comment', [1])

  call setline(2, '')
  call append('$', split(a:code_to_edit, "\n"))
  call append('$', s:separator)
  call matchaddpos('Comment', [line('$')])
endfunction

function! s:close_prompt_window() abort
  if expand('%:t') ==# 'Prompt'
    quit
  endif
endfunction

function! s:unpack_output(completion) abort
  let l:json = json_decode(a:completion)

  return {'edited_code': l:json.content, 'language': l:json.lang}
endfunction

function! s:write_to_completion_window(results) abort
  let l:win_id = bufwinid('Completion')

  if l:win_id == -1
    let l:bufnr = bufnr('Completion')

    if l:bufnr == -1
      new Completion
      setlocal buftype=nofile bufhidden=wipe noswapfile
    else
      split
      buffer l:bufnr
    endif
  else
    call win_gotoid(l:win_id)
  endif

  silent %delete _

  if v:shell_error == 0
    let l:results = s:unpack_output(a:results)
    execute 'setlocal syntax=' . l:results.language
    call setline(1, split(l:results.edited_code, "\n"))
  else
    setlocal syntax=off
    call setline(1, split(a:results, "\n"))
    call matchaddpos('WarningMsg', range(1, line('$')))
  endif

endfunction

" -----------------------------------------------------------------------------------------------------------
" Step 1: execute `ed` in normal mode to transfer code to new window

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

  call s:open_prompt_window(l:code_to_edit)

  let b:code_to_edit = l:code_to_edit
  let b:original_filename = l:original_filename

  normal! Go
  startinsert
endfunction

xnoremap <silent> ed :<C-u>call <SID>copy_selected_code_to_new_window()<CR>

" -----------------------------------------------------------------------------------------------------------
" Step 2: execute `:W` after instructions have been provided

function! s:get_instructions_after_delimiter() abort
  let l:start_line = search('^' . s:separator . '$', 'n')

  if l:start_line > 0
    return getline(l:start_line + 1, line('$'))
  endif

  throw 'delimiter not found'
endfunction

function! s:build_command() abort
  let l:command = []
  call add(l:command, 'edit')
  call add(l:command, '--filename=' . shellescape(b:original_filename))
  call add(l:command, shellescape(b:code_to_edit))

  let l:instructions = join(s:get_instructions_after_delimiter(), "\n")
  call add(l:command, shellescape(l:instructions))
  return join(l:command, ' ')
endfunction

function! s:consume_code_and_instructions() abort
  if executable('edit')
    call s:write_to_completion_window(system(s:build_command()))
  else
    throw 'could not find `edit` binary in $PATH'
  endif
endfunction

function! s:run_edit_command() abort
  if expand('%:t') ==# 'Prompt'
    try
      call s:consume_code_and_instructions()
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
