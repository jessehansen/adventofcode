number=1
project_name="aoc21-01"

while [ -d "$project_name" ]; do
  printf -v project_name 'aoc21-%02d' "$(( ++number ))"
done

cargo new $project_name
cd $project_name
touch input.txt
cp -fr ../template.rs src/main.rs
echo "aoc_common = { path =\"../../common\" }" >> Cargo.toml
