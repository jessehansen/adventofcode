for d in ./*/; do
  cd $d
  echo ""
  cargo r --release --quiet
  echo "----------------------------------------"
  cd ..
done
