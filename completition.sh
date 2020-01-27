#!/usr/bin/env bash

# mya
if type complete &>/dev/null; then
	_mya_completitions()
	{  	
    local cur
    _get_comp_words_by_ref -n : cur

		COMPREPLY=()
		case "${#COMP_WORDS[@]}" in
    2) while IFS='' read -r line; do COMPREPLY+=("$line"); done < <(compgen -W "run list" "${COMP_WORDS[1]}")
      ;;
		3) # we are at mya list 
      if [ "${COMP_WORDS[1]}" = "list" ]; then
        #echo "x has the value 'valid'"
        while IFS='' read -r line; do COMPREPLY+=("$line"); done < <(compgen -W "$(mya list -g)" "${COMP_WORDS[2]}")
      
      elif [ "${COMP_WORDS[1]}" = "run" ]; then
        # we are at mya run . Can suggest commands and groups too
        while IFS='' read -r line; do COMPREPLY+=("$line"); done < <(compgen -W "$(mya list -s)" "${COMP_WORDS[2]}")
        while IFS='' read -r line; do COMPREPLY+=("$line"); done < <(compgen -W "$(mya list -g)" "${COMP_WORDS[2]}")
      fi
		   ;;
		4)  # we are at mya run group - only run can have fourth param
      if [ "${COMP_WORDS[1]}" = "run" ]; then
        while IFS='' read -r line; do COMPREPLY+=("$line"); done < <(compgen -W "$(mya list "${COMP_WORDS[2]}" -s)" "${COMP_WORDS[3]}")
      fi  
      ;;  
		esac
    __ltrim_colon_completions "$cur"
	}

	complete -F _mya_completitions mya
fi