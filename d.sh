case "$(basename "$(ps -p $$ | awk 'NR > 1 { sub(/^-/, "", $4); print $4 }')")" in
  zsh)  __source_dir="$(dirname "$0:A")" ;;
  bash) __source_dir="$(builtin cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" ;;
  *) echo "unsupported shell" >&2 ; return 1 ;;
esac

d() {
  local tmp ret finalizer

  tmp="$(mktemp -u)"
  exec 9>"${tmp}"
  exec 8<"${tmp}"
  rm -f "${tmp}"

  "${__source_dir}/d" "$@"
  ret=$?

  while read finalizer; do
    case "${finalizer}" in
      cd:*) cd "${finalizer//cd:/}" ;;
      *) ;;
    esac
  done <&8

  exec 8<&- # close FD 8.
  exec 9<&- # close FD 9.

  return ${ret}
}