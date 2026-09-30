let s:separator = repeat('=', 109)

function! s:get_highlighted_block()
  let l:old_reg = getreg('x')
  let l:old_regtype = getregtype('x')

  normal! gv"xy

  let l:selection = getreg('x')
  call setreg('x', l:old_reg, l:old_regtype)

  return l:selection
endfunction

function! s:open_edit_command_buf()
  let s:code_to_edit = s:get_highlighted_block()
  let l:code_to_edit = split(s:code_to_edit, "\n")

  vnew
  setlocal buftype=nofile
  setlocal bufhidden=wipe
  setlocal noswapfile

  let b:is_edit_cmd_buf = v:true

  call setline(1, l:code_to_edit)
  call append('$', s:separator)

  normal! G
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

  let l:line_number = search('^' . s:separator, 'n') + 1
  let l:last_line = line('$')
  if l:line_number <= l:last_line
    let l:instructions = join(getline(l:line_number, l:last_line), "\n")
  else
    let l:instructions = ''
  endif

  if strlen(l:instructions) < 1
    echoerr 'No instructions provided. Cannot proceed!'
    return
  endif

  let l:prompt = shellescape('Apply the instructions: '. l:instructions . '\n\nTo the code: ' . s:code_to_edit)

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
