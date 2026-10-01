let s:input_open = repeat('>', 5)
let s:input_split = repeat('-', 5)
let s:input_close = repeat('<', 5)

function! s:set_valid_edit_cmd_buf()
  let b:is_edit_cmd_buf = v:true
endfunction

function! s:is_valid_edit_cmd_buf()
  if exists('b:is_edit_cmd_buf') && b:is_edit_cmd_buf
    return v:true
  endif

  return v:false
endfunction

" -----------------------------------------------------------------------------------------------------------

function! s:get_highlighted_block()
  let l:old_reg = getreg('x')
  let l:old_regtype = getregtype('x')

  normal! gv"xy

  let l:selection = getreg('x')
  call setreg('x', l:old_reg, l:old_regtype)

  return l:selection
endfunction

function! s:open_edit_command_buf()
  let l:selected_text = s:get_highlighted_block()
  let l:code_to_edit = split(l:selected_text, "\n")

  vnew
  setlocal buftype=nofile
  setlocal bufhidden=wipe
  setlocal noswapfile

  call s:set_valid_edit_cmd_buf()

  call setline(1, s:input_open)
  call setline(2, l:code_to_edit)
  call append('$', s:input_split)
  call append('$', s:input_close)

  normal! G
  normal! O
  startinsert
endfunction

xnoremap <silent> ed :<C-u>call <SID>open_edit_command_buf()<CR>

" -----------------------------------------------------------------------------------------------------------

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

function! s:print_results(output)
  call append('$', ['', '', '> Results:'])
  call append('$', split(a:output, '\n'))
endfunction

function! s:print_error(errmsg)
  call append('$', ['', '', '> Error:'])
  call append('$', split(a:errmsg, '\n'))
endfunction

function! s:run_edit_command(code_to_edit, instructions)
  let l:command = []
  call add(l:command, '/tmp/foo.py')
  call add(l:command, shellescape(a:code_to_edit))
  call add(l:command, '--instructions=' . shellescape(a:instructions))
  let l:output = system(join(l:command, ' '))

  if v:shell_error == 0
    call s:print_results(l:output)
  else
    call s:print_error(l:output)
  endif
endfunction

let s:was_prompt_consumed = v:false

function! s:consume_payload()
  if ! s:is_valid_edit_cmd_buf()
    echoerr 'Not a valid `edit` command buffer. Cannot proceed!'
    return
  endif

  if s:was_prompt_consumed
    call append('$', 'Prompt was already consumed. Close buffer and try again.')
    return
  endif

  let s:was_prompt_consumed = v:true

  let l:instructions = s:get_instructions()
  let l:code_to_edit = s:get_code_to_edit()

  call s:run_edit_command(l:code_to_edit, l:instructions)
  normal! G
endfunction

command A call <SID>consume_payload()
