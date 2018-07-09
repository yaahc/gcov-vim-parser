" Author: Bart Libert <bart.libert@gmail.com>
" Description: cppcheck linter for cpp files

call ale#Set('cpp_gcovcheck_executable', 'gcovcheck')
call ale#Set('cpp_gcovcheck_options', '')

function! ale_linters#cpp#gcovcheck#GetExecutable(buffer) abort
    return ale#Var(a:buffer, 'cpp_gcovcheck_executable')
endfunction

function! ale_linters#cpp#gcovcheck#GetCommand(buffer) abort
    let l:options = ale#Var(a:buffer, 'cpp_gcovcheck_options')

    return ale#Escape(ale_linters#cpp#gcovcheck#GetExecutable(a:buffer))
    \   . (!empty(l:options) ? ' ' . l:options : '')
    \   . ' %s'
endfunction

call ale#linter#Define('cpp', {
\   'name': 'gcovcheck',
\   'output_stream': 'both',
\   'executable_callback': 'ale_linters#cpp#gcovcheck#GetExecutable',
\   'command_callback': 'ale_linters#cpp#gcovcheck#GetCommand',
\   'callback': 'ale#handlers#gcovcheck#HandleGcovCheckFormat',
\})
