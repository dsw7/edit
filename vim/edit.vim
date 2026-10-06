" -----------------------------------------------------------------------------------------------------------
" VimScript for `edit` command integration
" Copyright © David Weber
" See https://github.com/dsw7/edit for more information
" -----------------------------------------------------------------------------------------------------------

let s:hl_ids_msg = []
let s:hl_ids_err = []

let s:input_split = repeat('─', winwidth(0))

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

function! s:open_prompt_window(code_to_edit) abort
  vnew Prompt
  setlocal buftype=nofile
  setlocal bufhidden=wipe
  setlocal noswapfile

  normal! ggdG

  call setline(1, ':W to submit prompt')
  call matchaddpos('Comment', [1])

  call setline(2, '')
  call append('$', a:code_to_edit)
  call append('$', s:input_split)

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
" Getters and setters

function! s:get_instructions() abort
  let l:start_line = search('^' . s:input_split . '$', 'n')
  if l:start_line == 0
    throw 'delimiter not found: ' . s:input_split
  endif

  let l:end_line = line('$')
  let l:lines = getline(l:start_line + 1, l:end_line)
  return join(l:lines, "\n")
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

  call s:open_prompt_window(l:code_to_edit)

  let b:code_to_edit = l:code_to_edit
  let b:original_filename = l:original_filename

  normal! Go
  startinsert
endfunction

xnoremap <silent> ed :<C-u>call <SID>open_edit_command_buf()<CR>

" -----------------------------------------------------------------------------------------------------------
" Operate on code in new buffer

function! s:open_completion_window(output) abort
  new Completion
  setlocal buftype=nofile
  setlocal bufhidden=wipe
  setlocal noswapfile

  let l:lines = split(a:output, "\n")
  call setline(1, l:lines)
endfunction

function! s:consume_payload() abort
  let l:code_to_edit = join(b:code_to_edit, "\n")
  let l:instructions = s:get_instructions()

  let l:command = []
  call add(l:command, '/tmp/foo.py')
  call add(l:command, shellescape(l:code_to_edit))
  call add(l:command, '--filename=' . shellescape(b:original_filename))
  call add(l:command, '--instructions=' . shellescape(l:instructions))
  let l:output = system(join(l:command, ' '))

  call s:open_completion_window(l:output)
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
