let s:input_open = repeat('>', 5)
let s:input_split = repeat('-', 5)
let s:input_close = repeat('<', 5)

function! s:color_lines_comment(lines)
  call matchaddpos('Comment', a:lines)
endfunction

function! s:color_lines_msg(lines)
  call matchaddpos('MoreMsg', a:lines)
endfunction

function! s:color_lines_red(lines)
  call matchaddpos('WarningMsg', a:lines)
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

  call setline(1, ':Run to submit prompt')
  call setline(2, ':Reset to reset prompt')
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
  let l:start_line = search(s:input_open)
  let l:middle_line = search(s:input_split)

  let l:code_to_edit = ''

  if l:start_line > 0 && l:middle_line > l:start_line
    let l:code_to_edit = join(getline(l:start_line + 1, l:middle_line - 1), "\n")
  endif

  return l:code_to_edit
endfunction

function! s:get_instructions()
  let l:middle_line = search(s:input_split)
  let l:end_line = search(s:input_close)

  let l:instructions = ''

  if l:middle_line > 0 && l:end_line > l:middle_line
    let l:instructions = join(getline(l:middle_line + 1, l:end_line - 1), "\n")
  endif

  return l:instructions
endfunction

function! s:print_results(results)
  let l:start_line = line('$')
  call append('$', [''])
  call append('$', split(a:results, "\n"))
  let l:end_line = line('$')

  let l:lines_to_color = range(l:start_line + 1, l:end_line)
  call s:color_lines_msg(l:lines_to_color)
endfunction

function! s:print_error(errmsg)
  let l:start_line = line('$')
  call append('$', [''])
  call append('$', split(a:errmsg, "\n"))
  let l:end_line = line('$')

  let l:lines_to_color = range(l:start_line + 1, l:end_line)
  call s:color_lines_red(l:lines_to_color)
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
    echoerr 'Payload was already consumed. Reset buffer and try again.'
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

command Run call <SID>consume_payload()

" -----------------------------------------------------------------------------------------------------------
" Reset prompt while preserving buffer

function! s:run_reset()
  let l:middle_line = search(s:input_split)
  execute (l:middle_line + 1). ',$delete'

  call append('$', s:input_close)
  normal! G
  normal! O
  startinsert

  let s:is_payload_consumed = v:false
endfunction

command Reset call <SID>run_reset()
