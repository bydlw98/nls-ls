_nls() {
    local i cur prev opts cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="nls"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        nls)
            opts="-a -A -c -C -d -F -g -h -H -i -I -k -l -L -n -o -p -r -R -s -S -t -u -x -1 --all --almost-all --allocated-bytes --color --directory --classify --gitignore --human-readable --dereference-command-line --help --inode --ignore-glob --iec --ignore-file --kibibytes --dereference --max-depth --mode --numeric-uid-gid --reverse --recursive --size --si --time --version [FILE]..."
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --color)
                    COMPREPLY=($(compgen -W "always auto never" -- "${cur}"))
                    return 0
                    ;;
                --ignore-glob)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -I)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-depth)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --mode)
                    COMPREPLY=($(compgen -W "native pwsh rwx" -- "${cur}"))
                    return 0
                    ;;
                --time)
                    COMPREPLY=($(compgen -W "accessed changed created modified atime ctime btime mtime" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _nls -o nosort -o bashdefault -o default nls
