exercism fetch rust
for test in */tests/*.rs; do
   sed -i -e '/#\[ignore\]/{
      s/#\[ignore\]\s*//
      /^\s*$/d
   }' $test
done
