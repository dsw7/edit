let s:separator = repeat('=', 109)

let s:input_open = repeat('>', 5)
let s:input_split = repeat('-', 5)
let s:input_close = repeat('<', 5)

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

  let b:is_edit_cmd_buf = v:true

  call setline(1, s:input_open)
  call setline(2, l:code_to_edit)
  call append('$', s:input_split)

  normal! G
endfunction

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

let s:was_prompt_consumed = v:false

function! s:run_edit_command()
  if ! exists('b:is_edit_cmd_buf') || ! b:is_edit_cmd_buf
    echoerr 'Not a valid `edit` command buffer. Cannot proceed!'
    return
  endif

  if s:was_prompt_consumed
    call s:prompt_was_consumed()
    return
  endif

  let s:was_prompt_consumed = v:true
  call append('$', s:input_close)

  let l:instructions = s:get_instructions()
  let l:code_to_edit = s:get_code_to_edit()
  let l:prompt = shellescape('Apply the instructions: '. l:instructions . '\n\nTo the code: ' . l:code_to_edit)

  let l:command = 'gpt short ' . l:prompt
  call append('$', '> Running command:')
  call append('$', ['```console', l:command, '```'])
  call append('$', s:separator)

  let l:output = system(l:command)
  if v:shell_error != 0
    call append('$', 'An error occurred!')
  endif

  call append('$', split(l:output, '\n'))
  call append('$', s:separator)

  normal! G
endfunction

xnoremap <silent> ed :<C-u>call <SID>open_edit_command_buf()<CR>
command A call <SID>run_edit_command()
