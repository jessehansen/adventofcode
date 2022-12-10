number=1
project_name="aoc22-01"

while [ -d "$project_name" ]; do
  printf -v project_name 'aoc22-%02d' "$(( ++number ))"
done

cargo new $project_name
cd $project_name
cp -fr ../template.rs src/main.rs

echo "anyhow = \"1.0.60\"\naoc_common = { path =\"../../common\" }" >> Cargo.toml
