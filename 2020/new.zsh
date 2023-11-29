number=1
project_name="aoc20-01"

while [ -d "$project_name" ]; do
  printf -v project_name 'aoc20-%02d' "$(( ++number ))"
done

cargo new $project_name
cd $project_name
cp -fr ../template.rs src/main.rs

echo "anyhow = \"1.0.75\"\naoc_common = { path =\"../../common\" }" >> Cargo.toml
