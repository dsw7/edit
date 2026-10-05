" -----------------------------------------------------------------------------------------------------------
" VimScript for `edit` command integration
" Copyright © David Weber
" See https://github.com/dsw7/edit for more information
" -----------------------------------------------------------------------------------------------------------

let s:hl_ids_msg = []
let s:hl_ids_err = []

let s:input_open = repeat('>', 5)
let s:input_split = repeat('-', 5)
let s:input_close = repeat('<', 5)

function! s:color_lines_msg(lines) abort
  let l:hl_id = matchaddpos('MoreMsg', a:lines)
  call add(s:hl_ids_msg, l:hl_id)
endfunction

function! s:color_lines_err(lines) abort
  let l:hl_id = matchaddpos('WarningMsg', a:lines)
  call add(s:hl_ids_err, l:hl_id)
endfunction

function! s:reset_color_on_msg_lines() abort
  for id in s:hl_ids_msg
    call matchdelete(id)
  endfor

  let s:hl_ids_msg = []
endfunction

function! s:reset_color_on_err_lines() abort
  for id in s:hl_ids_err
    call matchdelete(id)
  endfor

  let s:hl_ids_err = []
endfunction

function! s:print_exception() abort
  echohl ErrorMsg
  echo v:exception
  echohl None
endfunction

" -----------------------------------------------------------------------------------------------------------
" Buffer management

function! s:set_buffer_template() abort
  normal! ggdG

  call setline(1, ':W to submit prompt')
  call setline(2, ':C to reset prompt')
  call matchaddpos('Comment', [1, 2])

  call setline(3, '')
  call setline(4, s:input_open)
  call setline(5, s:input_split)
  call setline(6, s:input_close)
endfunction

function! s:open_new_buffer() abort
  vnew
  setlocal buftype=nofile
  setlocal bufhidden=wipe
  setlocal noswapfile

  call s:set_buffer_template()
  let b:valid_buffer = v:true
endfunction

function! s:is_valid_buffer() abort
  if exists('b:valid_buffer')
    return v:true
  endif

  echom 'not a valid `edit` command buffer'
  return v:false
endfunction

function! s:close_buffer() abort
  if exists('b:valid_buffer')
    quit
  endif
endfunction

augroup reset_payload_consumed_state_on_buffer_close
  autocmd!
  autocmd BufUnload * call s:reset_color_on_msg_lines()
  autocmd BufUnload * call s:reset_color_on_err_lines()
augroup END

" -----------------------------------------------------------------------------------------------------------
" State management

function! s:set_payload_is_ready_to_consume_state()
  let b:is_payload_consumed = v:false
endfunction

function! s:set_payload_is_consumed_state()
  let b:is_payload_consumed = v:true
endfunction

function! s:payload_is_consumed_state()
  if b:is_payload_consumed
    echom 'payload was already consumed'
    echom 'invoke :C to reset'
    return v:true
  endif

  return v:false
endfunction

" -----------------------------------------------------------------------------------------------------------
" Getters and setters

function! s:set_code_to_edit(code_to_edit) abort
  let l:start_line = search('^' . s:input_open . '$', 'n')

  if l:start_line == 0
    throw 'delimiter not found: ' . s:input_open
  else
    call append(l:start_line, a:code_to_edit)
  endif
endfunction

function! s:get_instructions() abort
  let l:start_line = search('^' . s:input_split . '$', 'n')
  if l:start_line == 0
    throw 'delimiter not found: ' . s:input_split
  endif

  let l:end_line = search('^' . s:input_close . '$', 'n')
  if l:end_line == 0
    throw 'delimiter not found: ' . s:input_close
  endif

  let l:lines = getline(l:start_line + 1, l:end_line - 1)
  return join(l:lines, "\n")
endfunction

function! s:set_results_and_return_range(results) abort
  let l:start_line = search('^' . s:input_close . '$', 'n')

  if l:start_line == 0
    throw 'delimiter not found: ' . s:input_close
  endif

  let l:lines = split(a:results, "\n")

  call setline(l:start_line + 1, '')
  call append('$', l:lines)

  let l:end_line = line('$')
  return [l:start_line + 2, l:end_line]
endfunction

" -----------------------------------------------------------------------------------------------------------
" Transfer highlighted code to new buffer on the right

function! s:yank_code_to_edit() abort
  let l:old_reg = getreg('x')
  let l:old_regtype = getregtype('x')

  normal! gv"xy

  let l:selection = getreg('x')
  call setreg('x', l:old_reg, l:old_regtype)

  return split(l:selection, "\n")
endfunction

function! s:open_edit_command_buf() abort
  let l:code_to_edit = s:yank_code_to_edit()
  let l:original_filename = bufname('%')

  call s:open_new_buffer()
  call s:set_code_to_edit(l:code_to_edit)

  let b:code_to_edit = l:code_to_edit
  let b:original_filename = l:original_filename
  call s:set_payload_is_ready_to_consume_state()

  normal! GO
  startinsert
endfunction

xnoremap <silent> ed :<C-u>call <SID>open_edit_command_buf()<CR>

" -----------------------------------------------------------------------------------------------------------
" Operate on code in new buffer

function! s:consume_payload() abort
  if s:payload_is_consumed_state()
    return
  endif

  call s:set_payload_is_consumed_state()

  let l:code_to_edit = join(b:code_to_edit, "\n")
  let l:instructions = s:get_instructions()

  let l:command = []
  call add(l:command, '/tmp/foo.py')
  call add(l:command, shellescape(l:code_to_edit))
  call add(l:command, '--filename=' . shellescape(b:original_filename))
  call add(l:command, '--instructions=' . shellescape(l:instructions))
  let l:output = system(join(l:command, ' '))

  let [l:start_line, l:end_line] = s:set_results_and_return_range(l:output)

  if v:shell_error == 0
    call s:color_lines_msg(range(l:start_line, l:end_line))
  else
    call s:color_lines_err(range(l:start_line, l:end_line))
  endif

  normal! G
endfunction

function! s:run_edit_command() abort
  if s:is_valid_buffer()
    try
      call s:consume_payload()
    catch /.*/
      call s:print_exception()
      call input('Press ENTER to close this window...')
      call s:close_buffer()
    endtry
  endif
endfunction

command! W call <SID>run_edit_command()

" -----------------------------------------------------------------------------------------------------------
" Retry logic

function! s:reset_edit_buffer() abort
  call s:reset_color_on_msg_lines()
  call s:reset_color_on_err_lines()

  call s:set_buffer_template()
  call s:set_code_to_edit(b:code_to_edit)

  call s:set_payload_is_ready_to_consume_state()
  normal! GO
  startinsert
endfunction

function s:run_reset_command() abort
  if s:is_valid_buffer()
    call s:reset_edit_buffer()
  endif
endfunction

command! C call <SID>run_reset_command()
