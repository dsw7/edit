" -----------------------------------------------------------------------------------------------------------
" VimScript for `edit` command integration
" Copyright © David Weber
" See https://github.com/dsw7/edit for more information
" -----------------------------------------------------------------------------------------------------------

function! s:run_edit_subprocess(instructions) abort
  let l:command = printf(
  \ 'edit --filename=%s %s %s',
  \ shellescape(b:original_filename),
  \ shellescape(b:code_to_edit),
  \ shellescape(a:instructions)
  \ )
  return system(l:command)
endfunction

function! s:run_edit_subprocess_with_echo() abort
  let l:command = printf(
  \ 'edit --echo --filename=%s %s %s',
  \ shellescape(b:original_filename),
  \ shellescape(b:code_to_edit),
  \ shellescape('This is a test')
  \ )
  return system(l:command)
endfunction

function! s:write_results_to_completion_window(json_results) abort
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

  execute 'setlocal syntax=' . a:json_results.lang_id
  call setline(1, split(a:json_results.updated_code, "\n"))
endfunction

function! s:write_usage_to_prompt(json_results) abort
  call append(line('$') - 1, '• model: ' . a:json_results.model)
  call append(line('$') - 1, '• input tokens: ' . a:json_results.input_tokens)
  call append(line('$') - 1, '• output tokens: ' . a:json_results.output_tokens)
  call matchaddpos('Title', range(line('$') - 3, line('$') - 1))
endfunction

function! s:unpack_json(results) abort
  let l:json_results = json_decode(a:results)
  call s:write_usage_to_prompt(l:json_results)
  call s:write_results_to_completion_window(l:json_results)
endfunction

function! s:write_error_to_prompt(error) abort
  let l:start_line = line('$')
  call append(l:start_line - 1, split(a:error, "\n"))
  call matchaddpos('WarningMsg', range(l:start_line, line('$') - 1))
endfunction

function! s:check_if_should_exit(user_input) abort
  return index(['quit', 'q', 'exit'], a:user_input) >= 0
endfunction

function! s:handle_submission(user_input) abort
  if s:check_if_should_exit(a:user_input)
    execute 'q!'
    return
  endif

  if a:user_input == 'echo'
    let l:results = s:run_edit_subprocess_with_echo()
  else
    let l:results = s:run_edit_subprocess(a:user_input)
  endif

  if v:shell_error == 0
    call s:unpack_json(l:results)
  else
    call s:write_error_to_prompt(l:results)
  endif

  call prompt_setprompt(bufnr('%'), '> ')
endfunction

function! s:write_header(code_to_edit) abort
  call setline(1, "• type 'quit', 'q' or 'exit' to exit")
  call setline(2, "• type 'echo' to echo payload (for debugging)")
  call matchaddpos('Comment', [1, 2])
  call setline(3, '')
  call setline(4, split(a:code_to_edit, "\n"))
  call append('$', repeat('─', ((&colorcolumn > 0 ? &colorcolumn : 81) - 1)))
  call matchaddpos('Comment', [line('$')])
endfunction

function! s:start_repl_loop(code_to_edit) abort
  vnew Prompt
  setlocal buftype=prompt
  setlocal bufhidden=wipe
  setlocal noswapfile

  call prompt_setprompt(bufnr('%'), '> ')
  call prompt_setcallback(bufnr('%'), 's:handle_submission')

  call s:write_header(a:code_to_edit)
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
  if executable('edit')
    let l:code_to_edit = s:yank_code_to_edit()
    let l:original_filename = bufname('%')
    call s:start_repl_loop(l:code_to_edit)
    let b:code_to_edit = l:code_to_edit
    let b:original_filename = l:original_filename
  else
    echohl ErrorMsg
    echomsg 'could not find `edit` command in $PATH'
    echohl None
  endif
endfunction

xnoremap ed :<C-u>call <SID>copy_selected_code_to_new_window()<CR>
