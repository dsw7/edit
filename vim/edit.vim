" -----------------------------------------------------------------------------------------------------------
" VimScript for `edit` command integration
" Copyright © David Weber
" See https://github.com/dsw7/edit for more information
" -----------------------------------------------------------------------------------------------------------

let s:input_open = repeat('>', 5)
let s:input_split = repeat('-', 5)
let s:input_close = repeat('<', 5)

let s:hl_ids_msg = []
let s:hl_ids_err = []

function! s:color_lines_comment(lines)
  call matchaddpos('Comment', a:lines)
endfunction

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

function! s:get_range_code_to_edit()
  let l:start_line = search(s:input_open)
  let l:end_line = search(s:input_split)

  return [l:start_line + 1, l:end_line - 1]
endfunction

function! s:get_range_instructions()
  let l:start_line = search(s:input_split)
  let l:end_line = search(s:input_close)

  return [l:start_line + 1, l:end_line - 1]
endfunction

function! s:get_range_results()
  let l:start_line = search(s:input_close)
  let l:end_line = line('$')

  return [l:start_line + 1, l:end_line]
endfunction

function! s:is_valid_delimiter_layout()
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
" State management

function! s:enable_valid_buffer_state()
  let b:is_valid_edit_command_buffer = v:true
endfunction

function! s:is_valid_buffer_state()
  if exists('b:is_valid_edit_command_buffer') && b:is_valid_edit_command_buffer
    return v:true
  else
    echoerr 'Not a valid `edit` command buffer. Cannot proceed!'
    return v:false
  endif
endfunction

let s:is_payload_consumed = v:false

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

  vnew
  setlocal buftype=nofile
  setlocal bufhidden=wipe
  setlocal noswapfile

  call s:enable_valid_buffer_state()

  call setline(1, ':W to submit prompt')
  call setline(2, ':C to reset prompt')
  call s:color_lines_comment(range(1, 2))
  call append('$', [''])

  call setline(4, s:input_open)
  call setline(5, l:code_to_edit)
  call append('$', s:input_split)
  call append('$', s:input_close)

  normal! G
  normal! O
  startinsert
endfunction

xnoremap <silent> ed :<C-u>call <SID>open_edit_command_buf()<CR>

" -----------------------------------------------------------------------------------------------------------
" Operate on code in new buffer

function! s:get_code_to_edit()
  let [l:start, l:end] = s:get_range_code_to_edit()
  return join(getline(l:start, l:end), "\n")
endfunction

function! s:get_instructions()
  let [l:start, l:end] = s:get_range_instructions()
  return join(getline(l:start, l:end), "\n")
endfunction

function! s:print_results(results)
  call append('$', [''])
  call append('$', split(a:results, "\n"))

  let [l:start, l:end] = s:get_range_results()
  let l:lines_to_color = range(l:start, l:end)

  call s:color_lines_msg(l:lines_to_color)
endfunction

function! s:print_error(errmsg)
  call append('$', [''])
  call append('$', split(a:errmsg, "\n"))

  let [l:start, l:end] = s:get_range_results()
  let l:lines_to_color = range(l:start, l:end)

  call s:color_lines_err(l:lines_to_color)
endfunction

function! s:run_edit_command(code_to_edit, instructions, filename)
  let l:command = []
  call add(l:command, '/tmp/foo.py')
  call add(l:command, shellescape(a:code_to_edit))
  call add(l:command, '--filename=' . shellescape(a:filename))
  call add(l:command, '--instructions=' . shellescape(a:instructions))
  let l:output = system(join(l:command, ' '))

  if v:shell_error == 0
    call s:print_results(l:output)
  else
    call s:print_error(l:output)
  endif
endfunction

function! s:consume_payload()
  if ! s:is_valid_buffer_state()
    return
  endif

  if s:is_payload_consumed
    echom 'Payload was already consumed. Run :C to reset'
    return
  endif

  let s:is_payload_consumed = v:true

  let l:instructions = s:get_instructions()
  let l:code_to_edit = s:get_code_to_edit()
  let l:this_filename = bufname(1)

  call s:run_edit_command(l:code_to_edit, l:instructions, l:this_filename)
  normal! G
endfunction

augroup reset_payload_consumed_state_on_buffer_close
  autocmd!
  autocmd BufUnload * let s:is_payload_consumed = v:false
augroup END

command! W call <SID>consume_payload()

" -----------------------------------------------------------------------------------------------------------
" Reset prompt while preserving buffer

function! s:delete_existing_results()
  let l:middle_line = search(s:input_split)
  execute (l:middle_line + 1) . ',$delete'
endfunction

function! s:run_reset()
  call s:delete_existing_results()
  call append('$', s:input_close)

  let s:is_payload_consumed = v:false

  call s:reset_color_on_msg_lines()
  call s:reset_color_on_err_lines()

  normal! G
  normal! O
  startinsert
endfunction

command! C call <SID>run_reset()
