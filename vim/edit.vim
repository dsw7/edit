" -----------------------------------------------------------------------------------------------------------
" VimScript for `edit` command integration
" Copyright © David Weber
" See https://github.com/dsw7/edit for more information
" -----------------------------------------------------------------------------------------------------------

let s:input_open = repeat('>', 5)
let s:input_split = repeat('-', 5)
let s:input_close = repeat('<', 5)

let s:is_payload_consumed = v:false

let s:hl_ids_msg = []
let s:hl_ids_err = []

function! s:color_lines_msg(lines)
  let l:hl_id = matchaddpos('MoreMsg', a:lines)
  call add(s:hl_ids_msg, l:hl_id)
endfunction

function! s:color_lines_err(lines)
  let l:hl_id = matchaddpos('WarningMsg', a:lines)
  call add(s:hl_ids_err, l:hl_id)
endfunction

function! s:reset_color_on_msg_lines()
  for id in s:hl_ids_msg
    call matchdelete(id)
  endfor

  let s:hl_ids_msg = []
endfunction

function! s:reset_color_on_err_lines()
  for id in s:hl_ids_err
    call matchdelete(id)
  endfor

  let s:hl_ids_err = []
endfunction

" -----------------------------------------------------------------------------------------------------------
" Buffer management

function! s:set_buffer_template()
  normal! ggdG

  call setline(1, ':W to submit prompt')
  call setline(2, ':C to reset prompt')
  call matchaddpos('Comment', [1, 2])

  call setline(3, '')
  call setline(4, s:input_open)
  call setline(5, s:input_split)
  call setline(6, s:input_close)
endfunction

function! s:set_code_to_edit(code_to_edit)
  let l:open_line = search(s:input_open, 'n')
  call append(l:open_line, a:code_to_edit)
endfunction

function! s:get_instructions()
  let l:start_line = search(s:input_split)
  let l:end_line = search(s:input_close)

  let l:lines = getline(l:start_line + 1, l:end_line - 1)
  return join(l:lines, "\n")
endfunction

function! s:set_results_and_return_range(results)
  let l:start_line = search(s:input_close)
  let l:lines = split(a:results, "\n")

  call setline(l:start_line + 1, '')
  call append('$', l:lines)

  let l:end_line = line('$')
  return [l:start_line + 2, l:end_line]
endfunction

function! s:is_valid_working_buffer()
  let l:open_line = search(s:input_open, 'n')
  let l:split_line = search(s:input_split, 'n')
  let l:close_line = search(s:input_close, 'n')

  if l:open_line == 0 || l:split_line == 0 || l:close_line == 0
    echoerr 'one or more delimiters not found'
    return v:false
  endif

  if l:open_line >= l:split_line || l:split_line >= l:close_line
    echoerr 'delimiters not in correct order'
    return v:false
  endif

  return v:true
endfunction

" -----------------------------------------------------------------------------------------------------------
" Transfer highlighted code to new buffer on the right

function! s:yank_code_to_edit()
  let l:old_reg = getreg('x')
  let l:old_regtype = getregtype('x')

  normal! gv"xy

  let l:selection = getreg('x')
  call setreg('x', l:old_reg, l:old_regtype)

  return split(l:selection, "\n")
endfunction

function! s:open_edit_command_buf()
  let l:code_to_edit = s:yank_code_to_edit()
  let l:original_filename = bufname('%')

  vnew
  setlocal buftype=nofile
  setlocal bufhidden=wipe
  setlocal noswapfile

  let b:code_to_edit = l:code_to_edit
  let b:original_filename = l:original_filename

  call s:set_buffer_template()
  call s:set_code_to_edit(b:code_to_edit)

  normal! GO
  startinsert
endfunction

xnoremap <silent> ed :<C-u>call <SID>open_edit_command_buf()<CR>

" -----------------------------------------------------------------------------------------------------------
" Operate on code in new buffer

function! s:run_edit_command(filename, code_to_edit, instructions)
  let l:command = []
  call add(l:command, '/tmp/foo.py')
  call add(l:command, shellescape(a:code_to_edit))
  call add(l:command, '--filename=' . shellescape(a:filename))
  call add(l:command, '--instructions=' . shellescape(a:instructions))

  let l:output = system(join(l:command, ' '))
  let [l:start_line, l:end_line] = s:set_results_and_return_range(l:output)

  if v:shell_error == 0
    call s:color_lines_msg(range(l:start_line, l:end_line))
  else
    call s:color_lines_err(range(l:start_line, l:end_line))
  endif
endfunction

function! s:consume_payload()
  if ! s:is_valid_working_buffer()
    quit
    return
  endif

  if s:is_payload_consumed
    echom 'payload was already consumed'
    echom 'invoke :C to reset'
    return
  endif

  let s:is_payload_consumed = v:true

  let l:instructions = s:get_instructions()
  call s:run_edit_command(b:original_filename, b:code_to_edit, l:instructions)

  normal! G
endfunction

command! W call <SID>consume_payload()

" -----------------------------------------------------------------------------------------------------------
" Retry logic

function! s:run_reset()
  if ! s:is_valid_working_buffer()
    quit
    return
  endif

  call s:reset_color_on_msg_lines()
  call s:reset_color_on_err_lines()

  call s:set_buffer_template()
  call s:set_code_to_edit(b:code_to_edit)

  let s:is_payload_consumed = v:false

  normal! GO
  startinsert
endfunction

command! C call <SID>run_reset()

" -----------------------------------------------------------------------------------------------------------
" Miscellaneous cleanup logic

augroup reset_payload_consumed_state_on_buffer_close
  autocmd!
  autocmd BufUnload * let s:is_payload_consumed = v:false
  autocmd BufUnload * call s:reset_color_on_msg_lines()
  autocmd BufUnload * call s:reset_color_on_err_lines()
augroup END
