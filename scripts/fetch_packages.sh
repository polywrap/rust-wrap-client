function joinByString() {
  local separator="$1"
  shift
  local first="$1"
  shift
  printf "%s" "$first" "${@/#/$separator}"
}

packages_arr=($(ls packages | grep -v '^tests'))
# classic for-loop
for ((idx=0; idx < ${#packages_arr[@]}; ++idx)); do
    # act on ${packages_arr[$idx]}
    packages_arr[$idx]="\"${packages_arr[$idx]}\""
done

packages_str=$(joinByString ', ' ${packages_arr[@]})
packages_json="{ \"package\": [ ${packages_str} ] }"
echo $packages_json