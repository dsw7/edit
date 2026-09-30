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

  let l:prompt = shellescape('Reverse the text: ' . s:code_to_edit)

  if strlen(l:prompt) < 1
    echoerr 'No prompt provided. Cannot proceed!'
    return
  endif

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
